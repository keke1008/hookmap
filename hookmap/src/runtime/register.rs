use super::{
    compute_event_block,
    storage::{Hook, HookKind, Storage},
};
use crate::hotkey::{Action, ButtonSet, HotkeyInfo, Modifier, MouseEventHandler};
use hookmap_core::{
    Button, ButtonAction, ButtonEvent, EventBlock, MouseCursorEvent, MouseWheelEvent,
};
use std::{
    collections::HashMap,
    iter,
    sync::{atomic::AtomicBool, Arc},
};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum HookTrigger {
    Single(Button),
    Any(Vec<Button>),
}

impl HookTrigger {
    fn iter(&self) -> impl Iterator<Item = &Button> {
        match self {
            Self::Single(trigger) => {
                Box::new(iter::once(trigger)) as Box<dyn Iterator<Item = &Button>>
            }
            Self::Any(triggers) => Box::new(triggers.iter()),
        }
    }
}

impl From<ButtonSet> for HookTrigger {
    fn from(buttons: ButtonSet) -> Self {
        match buttons {
            ButtonSet::Any(buttons) => Self::Any(buttons),
            ButtonSet::Single(button) => Self::Single(button),
            ButtonSet::All(_) => unreachable!(),
        }
    }
}

type ModifierHookInfo = (Vec<Action<ButtonEvent>>, Vec<EventBlock>);
#[derive(Debug, Default)]
struct ModifierHook {
    on_press: ModifierHookInfo,
    on_release: ModifierHookInfo,
}

type SoloHookBuffer = HashMap<HookTrigger, (Vec<Action<ButtonEvent>>, Vec<EventBlock>)>;
type ModifierHookBuffer = HashMap<Arc<Modifier>, HashMap<HookTrigger, ModifierHook>>;

#[derive(Debug, Default)]
struct Buffer {
    solo_on_press: SoloHookBuffer,
    solo_on_release: SoloHookBuffer,
    modifier: ModifierHookBuffer,
}

impl Buffer {
    fn register_solo(&mut self, trigger: HookTrigger, hotkey: &HotkeyInfo) {
        if hotkey.trigger_action.is_satisfied(ButtonAction::Press) {
            let buffer = self.solo_on_press.entry(trigger.clone()).or_default();
            buffer.0.push(hotkey.action.clone());
            buffer.1.push(hotkey.event_block);
        }
        if hotkey.trigger_action.is_satisfied(ButtonAction::Release) {
            let buffer = self.solo_on_release.entry(trigger).or_default();
            buffer.0.push(hotkey.action.clone());
            buffer.1.push(hotkey.event_block);
        }
    }

    fn register_modifier(
        &mut self,
        modifier: Arc<Modifier>,
        trigger: HookTrigger,
        hotkey: &HotkeyInfo,
    ) {
        let modifier_buffers = self
            .modifier
            .entry(modifier)
            .or_default()
            .entry(trigger)
            .or_default();
        if hotkey.trigger_action.is_satisfied(ButtonAction::Press) {
            let buffer = &mut modifier_buffers.on_press;
            buffer.0.push(hotkey.action.clone());
            buffer.1.push(hotkey.event_block);
        }
        if hotkey.trigger_action.is_satisfied(ButtonAction::Release) {
            let buffer = &mut modifier_buffers.on_release;
            buffer.0.push(hotkey.action.clone());
            buffer.1.push(hotkey.event_block);
        }
    }

    fn register_hotkey(&mut self, hotkey: HotkeyInfo) {
        let modifier = match hotkey.trigger {
            ButtonSet::All(ref buttons) => Arc::new(hotkey.modifier.add(buttons, &[])),
            _ => Arc::clone(&hotkey.modifier),
        };
        let triggers: Box<dyn Iterator<Item = HookTrigger>> = match hotkey.trigger.clone() {
            ButtonSet::All(buttons) => Box::new(buttons.into_iter().map(HookTrigger::Single)),
            ButtonSet::Single(button) => Box::new(iter::once(HookTrigger::Single(button))),
            ButtonSet::Any(buttons) => Box::new(iter::once(HookTrigger::Any(buttons))),
        };
        if modifier.is_empty() {
            triggers
                .into_iter()
                .for_each(|trigger| self.register_solo(trigger, &hotkey));
        } else {
            triggers
                .into_iter()
                .for_each(|trigger| self.register_modifier(modifier.clone(), trigger, &hotkey));
        }
    }
}

#[derive(Default, Debug)]
pub(crate) struct Register {
    storage: Storage,
    buffer: Buffer,
}

impl Register {
    fn generate_modifier_pressed_hook(
        hook_info: ModifierHookInfo,
        modifier: Arc<Modifier>,
        activated: Arc<AtomicBool>,
    ) -> Arc<Hook> {
        let (actions, event_blocks) = hook_info;
        Arc::new(Hook {
            action: Action::from(actions),
            event_block: compute_event_block(&event_blocks),
            kind: HookKind::OnModifierPressed {
                modifier,
                activated,
            },
        })
    }

    fn generate_modifier_released_hook(
        hook_info: ModifierHookInfo,
        activated: Arc<AtomicBool>,
    ) -> Arc<Hook> {
        let (actions, event_blocks) = hook_info;
        Arc::new(Hook {
            action: Action::from(actions),
            event_block: compute_event_block(&event_blocks),
            kind: HookKind::OnModifierReleased { activated },
        })
    }

    fn generate_solo_hook(buffer: SoloHookBuffer) -> Vec<(Button, Arc<Hook>)> {
        let mut result = Vec::with_capacity(buffer.len());
        for (triggers, (actions, event_blocks)) in buffer {
            let hook = Arc::new(Hook {
                action: Action::from(actions),
                event_block: compute_event_block(&event_blocks),
                kind: HookKind::Solo,
            });
            result.extend_from_slice(
                &triggers
                    .iter()
                    .cloned()
                    .zip(iter::repeat(hook))
                    .collect::<Vec<_>>(),
            );
        }
        result
    }

    fn disable_modifier_key(modifier: Button, storage: &mut SoloHookBuffer) {
        storage
            .entry(HookTrigger::Single(modifier))
            .or_default()
            .1
            .push(EventBlock::Block)
    }

    pub(super) fn into_inner(mut self) -> Storage {
        for (modifier, hook_info) in self.buffer.modifier {
            for (triggers, modifier_hook) in hook_info {
                let activated = Arc::default();
                let hook = Self::generate_modifier_pressed_hook(
                    modifier_hook.on_press,
                    Arc::clone(&modifier),
                    Arc::clone(&activated),
                );
                let storage = &mut self.storage;
                triggers
                    .iter()
                    .zip(iter::repeat(hook))
                    .for_each(|(&trigger, hook)| {
                        storage.on_press.entry(trigger).or_default().push(hook);
                    });

                let hook =
                    Self::generate_modifier_released_hook(modifier_hook.on_release, activated);
                let storage = &mut self.storage;
                triggers
                    .iter()
                    .chain(modifier.iter())
                    .zip(iter::repeat(hook))
                    .for_each(|(&trigger, hook)| {
                        storage.on_release.entry(trigger).or_default().push(hook);
                    });
                for modifier in modifier.iter() {
                    Self::disable_modifier_key(*modifier, &mut self.buffer.solo_on_press);
                    Self::disable_modifier_key(*modifier, &mut self.buffer.solo_on_release);
                }
            }
        }

        let storage = &mut self.storage;
        Self::generate_solo_hook(self.buffer.solo_on_press)
            .into_iter()
            .for_each(|(trigger, hook)| {
                storage.on_press.entry(trigger).or_default().push(hook);
            });
        let storage = &mut self.storage;
        Self::generate_solo_hook(self.buffer.solo_on_release)
            .into_iter()
            .for_each(|(trigger, hook)| {
                storage.on_release.entry(trigger).or_default().push(hook);
            });

        self.storage
    }

    pub(crate) fn register_hotkey(&mut self, hotkey: HotkeyInfo) {
        self.buffer.register_hotkey(hotkey);
    }

    pub(crate) fn register_cursor_event_handler(
        &mut self,
        handler: MouseEventHandler<MouseCursorEvent>,
    ) {
        self.storage.mouse_cursor.push(Arc::new(handler));
    }

    pub(crate) fn register_wheel_event_event_handler(
        &mut self,
        handler: MouseEventHandler<MouseWheelEvent>,
    ) {
        self.storage.mouse_wheel.push(Arc::new(handler));
    }
}
