use crate::hotkey::{Action, Modifier, MouseEventHandler, TriggerAction};
use hookmap_core::{
    Button, ButtonAction, ButtonEvent, EventBlock, MouseCursorEvent, MouseWheelEvent,
};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Debug)]
pub(super) enum HookKind {
    Press {
        modifier: Arc<Modifier>,
        activated: Arc<AtomicBool>,
    },

    Release {
        activated: Arc<AtomicBool>,
    },

    Solo {
        trigger_action: TriggerAction,
    },
}

#[derive(Debug)]
pub(crate) struct Hook {
    pub(super) action: Action<ButtonEvent>,
    pub(super) event_block: EventBlock,
    pub(super) kind: HookKind,
}

impl Hook {
    pub(super) fn is_satisfied(&self, event: &ButtonEvent) -> bool {
        match &self.kind {
            HookKind::Solo { trigger_action } => trigger_action.is_satisfied(event.action),
            HookKind::Release { activated, .. } => {
                event.action == ButtonAction::Release && activated.swap(false, Ordering::SeqCst)
            }
            HookKind::Press {
                modifier,
                activated,
                ..
            } => {
                event.action == ButtonAction::Press && modifier.is_all_pressed() && {
                    activated.store(true, Ordering::SeqCst);
                    true
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct MouseHook<E> {
    pub(super) modifier: Arc<Modifier>,
    pub(super) action: Action<E>,
    pub(super) event_block: EventBlock,
}

pub(super) type ButtonStorage = HashMap<Button, Vec<Arc<Hook>>>;
pub(super) type MouseStorage<E> = Vec<Arc<MouseEventHandler<E>>>;
pub(super) type MouseCursorStorage = MouseStorage<MouseCursorEvent>;
pub(super) type MouseWheelStorage = MouseStorage<MouseWheelEvent>;

#[derive(Default, Debug)]
pub(super) struct Storage {
    pub(super) button: ButtonStorage,
    pub(super) mouse_cursor: MouseCursorStorage,
    pub(super) mouse_wheel: MouseWheelStorage,
}
