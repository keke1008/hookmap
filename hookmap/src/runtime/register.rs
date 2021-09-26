use super::{
    compute_event_block,
    storage::{Hook, HookKind, Storage},
};
use crate::hotkey::{Action, Hotkey, Modifier, MouseEventHandler};
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

#[derive(Default, Debug)]
pub(crate) struct Register {
    storage: Storage,
    modifier: HashMap<Arc<Modifier>, HashMap<Button, ModifierHook>>,
    mouse_cursor: Vec<MouseEventHandler<MouseCursorEvent>>,
    mouse_wheel: Vec<MouseEventHandler<MouseWheelEvent>>,
}

impl<E: Copy + 'static> From<Vec<Action<E>>> for Action<E> {
    fn from(actions: Vec<Action<E>>) -> Self {
        Action::from(move |e| actions.iter().for_each(|action| action.0(e)))
    }
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
                    kind: HookKind::Press {
                        modifier: modifier.clone(),
                        activated: Arc::clone(&activation_flag),
                    },
                });
                self.storage.button.entry(trigger).or_default().push(hook);

                let (actions, event_blocks): (Vec<_>, Vec<_>) =
                    modifier_hook.on_release.into_iter().unzip();
                let hook = Arc::new(Hook {
                    action: Action::from(actions),
                    event_block: compute_event_block(&event_blocks),
                    kind: HookKind::Release {
                        activated: Arc::clone(&activation_flag),
                    },
                });
                for modifier in modifier.iter() {
                    let hook = Arc::clone(&hook);
                    self.storage.button.entry(*modifier).or_default().push(hook);
                }
            }
        }
        self.storage
    }

    pub(crate) fn register_hotkey(&mut self, hotkey: Hotkey) {
        if hotkey.modifier.is_empty() {
            let hook = Hook {
                action: hotkey.action,
                event_block: hotkey.event_block,
                kind: HookKind::Solo {
                    trigger_action: hotkey.trigger_action,
                },
            };
            self.storage
                .button
                .entry(hotkey.trigger)
                .or_default()
                .push(Arc::new(hook));
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
