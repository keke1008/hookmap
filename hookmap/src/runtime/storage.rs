use crate::hotkey::{Action, Modifier, MouseEventHandler};
use hookmap_core::{Button, ButtonEvent, EventBlock, MouseCursorEvent, MouseWheelEvent};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Debug)]
pub(super) enum HookKind {
    OnModifierPressed {
        modifier: Arc<Modifier>,
        activated: Arc<AtomicBool>,
    },

    OnModifierReleased {
        activated: Arc<AtomicBool>,
    },

    Solo,
}

#[derive(Debug)]
pub(crate) struct Hook {
    pub(super) action: Action<ButtonEvent>,
    pub(super) event_block: EventBlock,
    pub(super) kind: HookKind,
}

impl Hook {
    pub(super) fn is_satisfied(&self) -> bool {
        match &self.kind {
            HookKind::Solo => true,
            HookKind::OnModifierReleased { activated, .. } => {
                activated.swap(false, Ordering::SeqCst)
            }
            HookKind::OnModifierPressed {
                modifier,
                activated,
                ..
            } => {
                modifier.satisfies_condition() && {
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
    pub(super) on_press: ButtonStorage,
    pub(super) on_release: ButtonStorage,
    pub(super) mouse_cursor: MouseCursorStorage,
    pub(super) mouse_wheel: MouseWheelStorage,
}
