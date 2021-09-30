use crate::hotkey::{Action, ModifierKeys, MouseEventHandler};
use hookmap_core::{Button, ButtonEvent, EventBlock, MouseCursorEvent, MouseWheelEvent};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Debug)]
pub(super) enum HookKind {
    Independet {
        modifier_keys: Arc<ModifierKeys>,
    },
    LinkedOnPress {
        modifier_keys: Arc<ModifierKeys>,
        activated: Arc<AtomicBool>,
    },
    LinkedOnRelease {
        activated: Arc<AtomicBool>,
    },
}

#[derive(Debug)]
pub(super) struct HookInfo {
    kind: HookKind,
    action: Action<ButtonEvent>,
    event_block: EventBlock,
}

impl HookInfo {
    fn new(kind: HookKind, action: Action<ButtonEvent>, event_block: EventBlock) -> Self {
        Self {
            kind,
            action,
            event_block,
        }
    }
}

impl HookInfo {
    pub(super) fn satisfies_condition(&self) -> bool {
        match &self.kind {
            HookKind::Independet { modifier_keys } => modifier_keys.satisfies_condition(),
            HookKind::LinkedOnRelease { activated } => activated.swap(false, Ordering::SeqCst),
            HookKind::LinkedOnPress {
                modifier_keys,
                activated,
            } => {
                modifier_keys.satisfies_condition() && {
                    activated.store(true, Ordering::SeqCst);
                    true
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct MouseHook<E> {
    pub(super) modifier_keys: Arc<ModifierKeys>,
    pub(super) action: Action<E>,
    pub(super) event_block: EventBlock,
}

pub(super) type ButtonStorage<T> = HashMap<Button, Vec<Arc<T>>>;
pub(super) type MouseStorage<E> = Vec<Arc<MouseEventHandler<E>>>;
pub(super) type MouseCursorStorage = MouseStorage<MouseCursorEvent>;
pub(super) type MouseWheelStorage = MouseStorage<MouseWheelEvent>;

#[derive(Default, Debug)]
pub(super) struct Storage {
    pub(super) on_press: ButtonStorage<HookKind>,
    pub(super) on_release: ButtonStorage<HookKind>,
    pub(super) mouse_cursor: MouseCursorStorage,
    pub(super) mouse_wheel: MouseWheelStorage,
}
