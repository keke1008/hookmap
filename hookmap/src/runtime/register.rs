use super::{
    compute_event_block,
    storage::{OnPressHook, OnReleaseHook, Storage},
};
use crate::hotkey::{Action, ButtonSet, HotkeyInfo, ModifierKeys, MouseEventHandler};
use hookmap_core::{
    Button, ButtonAction, ButtonEvent, EventBlock, MouseCursorEvent, MouseWheelEvent,
};
use std::{
    collections::HashMap,
    iter,
    sync::{atomic::AtomicBool, Arc},
};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum TriggerButtonKind {
    Single(Button),
    Any(Vec<Button>),
}

impl TriggerButtonKind {
    fn iter(&self) -> impl Iterator<Item = &Button> {
        match self {
            Self::Single(trigger) => {
                Box::new(iter::once(trigger)) as Box<dyn Iterator<Item = &Button>>
            }
            Self::Any(triggers) => Box::new(triggers.iter()),
        }
    }
}

impl From<ButtonSet> for TriggerButtonKind {
    fn from(buttons: ButtonSet) -> Self {
        match buttons {
            ButtonSet::Any(buttons) => Self::Any(buttons),
            ButtonSet::Single(button) => Self::Single(button),
            ButtonSet::All(_) => unreachable!(),
        }
    }
}

type BufferedHookInfo = (Vec<Action<ButtonEvent>>, Vec<EventBlock>);
#[derive(Debug, Default)]
struct BufferedHook {
    on_press: BufferedHookInfo,
    on_release: BufferedHookInfo,
}

type BufferInner = HashMap<Arc<ModifierKeys>, HashMap<TriggerButtonKind, BufferedHook>>;

#[derive(Debug, Default)]
struct Buffer(BufferInner);

impl Buffer {
    fn register_modifier(
        &mut self,
        modifier_keys: Arc<ModifierKeys>,
        trigger: TriggerButtonKind,
        hotkey: &HotkeyInfo,
    ) {
        let modifier_buffers = self
            .0
            .entry(modifier_keys)
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
        let modifier_keys = match hotkey.trigger {
            ButtonSet::All(ref buttons) => Arc::new(hotkey.modifier.add(buttons, &[])),
            _ => Arc::clone(&hotkey.modifier),
        };
        let triggers: Box<dyn Iterator<Item = TriggerButtonKind>> = match hotkey.trigger.clone() {
            ButtonSet::All(buttons) => Box::new(buttons.into_iter().map(TriggerButtonKind::Single)),
            ButtonSet::Single(button) => Box::new(iter::once(TriggerButtonKind::Single(button))),
            ButtonSet::Any(buttons) => Box::new(iter::once(TriggerButtonKind::Any(buttons))),
        };
        triggers.into_iter().for_each(|trigger| {
            self.register_modifier(Arc::clone(&modifier_keys), trigger, &hotkey)
        });
    }
}

#[derive(Default, Debug)]
pub(crate) struct Register {
    storage: Storage,
    buffer: Buffer,
}

impl Register {
    fn generate_pressed_hook(
        hook_info: BufferedHookInfo,
        modifier_keys: Arc<ModifierKeys>,
        activated: Arc<AtomicBool>,
    ) -> Arc<OnPressHook> {
        let (actions, event_blocks) = hook_info;
        Arc::new(OnPressHook::new(
            actions,
            modifier_keys,
            compute_event_block(&event_blocks),
            activated,
        ))
    }

    fn generate_released_hook(
        hook_info: BufferedHookInfo,
        activated: Arc<AtomicBool>,
    ) -> Arc<OnReleaseHook> {
        let (actions, event_blocks) = hook_info;
        Arc::new(OnReleaseHook::new(
            actions,
            compute_event_block(&event_blocks),
            activated,
        ))
    }

    fn disable_modifier_keys(&mut self) {
        let modifiers = self
            .buffer
            .0
            .keys()
            .map(|modifier_keys| modifier_keys.iter())
            .flatten()
            .cloned()
            .collect::<Vec<_>>();
        let empty_modifier_buffer = self.buffer.0.entry(Arc::default()).or_default();
        modifiers.iter().for_each(|modifier| {
            let buffer = empty_modifier_buffer
                .entry(TriggerButtonKind::Single(*modifier))
                .or_default();
            buffer.on_press.1.push(EventBlock::Block);
            buffer.on_release.1.push(EventBlock::Block);
        });
    }

    pub(super) fn into_inner(mut self) -> Storage {
        self.disable_modifier_keys();

        for (modifier_keys, hook_info) in self.buffer.0 {
            for (trigger_buttons, buffered_hook) in hook_info {
                let activated = Arc::default();
                let hook = Self::generate_pressed_hook(
                    buffered_hook.on_press,
                    Arc::clone(&modifier_keys),
                    Arc::clone(&activated),
                );
                let storage = &mut self.storage;
                trigger_buttons
                    .iter()
                    .zip(iter::repeat(hook))
                    .for_each(|(&trigger, hook)| {
                        storage.on_press.entry(trigger).or_default().push(hook);
                    });

                let hook = Self::generate_released_hook(buffered_hook.on_release, activated);
                let storage = &mut self.storage;
                trigger_buttons
                    .iter()
                    .chain(modifier_keys.iter())
                    .zip(iter::repeat(hook))
                    .for_each(|(&trigger, hook)| {
                        storage.on_release.entry(trigger).or_default().push(hook);
                    });
            }
        }

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
