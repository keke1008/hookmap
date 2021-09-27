use super::{
    compute_event_block,
    storage::{ButtonStorage, Hook, HookKind, Storage},
};
use crate::hotkey::{Action, HotkeyInfo, Modifier, MouseEventHandler};
use hookmap_core::{
    Button, ButtonAction, ButtonEvent, EventBlock, MouseCursorEvent, MouseWheelEvent,
};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Default)]
struct ModifierHook {
    on_press: Vec<(Action<ButtonEvent>, EventBlock)>,
    on_release: Vec<(Action<ButtonEvent>, EventBlock)>,
}

type SoloHookBuffer = HashMap<Button, Vec<(Action<ButtonEvent>, EventBlock)>>;

#[derive(Default, Debug)]
pub(crate) struct Register {
    storage: Storage,
    solo_on_press: SoloHookBuffer,
    solo_on_release: SoloHookBuffer,
    modifier: HashMap<Arc<Modifier>, HashMap<Button, ModifierHook>>,
}

impl Register {
    pub(super) fn into_inner(mut self) -> Storage {
        for (modifier, hooks) in self.modifier {
            for (trigger, modifier_hook) in hooks {
                let (actions, event_blocks): (Vec<_>, Vec<_>) =
                    modifier_hook.on_press.into_iter().unzip();
                let activation_flag = Arc::default();
                let hook = Arc::new(Hook {
                    action: Action::from(actions),
                    event_block: compute_event_block(&event_blocks),
                    kind: HookKind::ActivateModifier {
                        modifier: modifier.clone(),
                        activated: Arc::clone(&activation_flag),
                    },
                });
                self.storage.on_press.entry(trigger).or_default().push(hook);

                let (actions, event_blocks): (Vec<_>, Vec<_>) =
                    modifier_hook.on_release.into_iter().unzip();
                let hook = Arc::new(Hook {
                    action: Action::from(actions),
                    event_block: compute_event_block(&event_blocks),
                    kind: HookKind::InactivateModifier {
                        activated: activation_flag,
                    },
                });
                let register_on_release = |storage: &mut ButtonStorage, trigger| {
                    storage.entry(trigger).or_default().push(Arc::clone(&hook));
                };

                let register_block = |storage: &mut SoloHookBuffer, trigger| {
                    storage
                        .entry(trigger)
                        .or_default()
                        .push((Action::Noop, EventBlock::Block));
                };

                for modifier in modifier.iter() {
                    register_on_release(&mut self.storage.on_release, *modifier);
                    register_block(&mut self.solo_on_press, *modifier);
                    register_block(&mut self.solo_on_release, *modifier);
                }
                register_on_release(&mut self.storage.on_release, trigger);
            }
        }

        let register_solo = |solo_buffer: SoloHookBuffer, storage: &mut ButtonStorage| {
            for (trigger, hooks) in solo_buffer {
                let (actions, event_blocks): (Vec<_>, Vec<_>) = hooks.into_iter().unzip();
                let hook = Hook {
                    action: Action::from(actions),
                    event_block: compute_event_block(&event_blocks),
                    kind: HookKind::Solo,
                };
                storage.entry(trigger).or_default().push(Arc::new(hook));
            }
        };
        register_solo(self.solo_on_press, &mut self.storage.on_press);
        register_solo(self.solo_on_release, &mut self.storage.on_release);
        self.storage
    }

    pub(crate) fn register_hotkey(&mut self, hotkey: HotkeyInfo) {
        if hotkey.modifier.is_empty() {
            if hotkey.trigger_action.is_satisfied(ButtonAction::Press) {
                self.solo_on_press
                    .entry(hotkey.trigger)
                    .or_default()
                    .push((hotkey.action.clone(), hotkey.event_block));
            }
            if hotkey.trigger_action.is_satisfied(ButtonAction::Release) {
                self.solo_on_release
                    .entry(hotkey.trigger)
                    .or_default()
                    .push((hotkey.action, hotkey.event_block));
            }
        } else {
            let modifier_hook = self
                .modifier
                .entry(hotkey.modifier)
                .or_default()
                .entry(hotkey.trigger)
                .or_default();
            if hotkey.trigger_action.is_satisfied(ButtonAction::Press) {
                modifier_hook
                    .on_press
                    .push((hotkey.action.clone(), hotkey.event_block));
            }
            if hotkey.trigger_action.is_satisfied(ButtonAction::Release) {
                modifier_hook
                    .on_release
                    .push((hotkey.action.clone(), hotkey.event_block));
            }
        }
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
