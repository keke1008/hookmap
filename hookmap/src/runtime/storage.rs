use crate::hotkey::{Action, ModifierKeys, MouseEventHandler};
use hookmap_core::{Button, ButtonEvent, EventBlock, MouseCursorEvent, MouseWheelEvent};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Debug)]
pub(super) struct OnPressHook {
    pub(super) action: Action<ButtonEvent>,
    pub(super) modifier_keys: Arc<ModifierKeys>,
    pub(super) activated: Arc<AtomicBool>,
    pub(super) event_block: EventBlock,
}

impl OnPressHook {
    pub(super) fn new(
        action: impl Into<Action<ButtonEvent>>,
        modifier_keys: Arc<ModifierKeys>,
        activated: Arc<AtomicBool>,
        event_block: impl Into<EventBlock>,
    ) -> Self {
        Self {
            action: action.into(),
            modifier_keys,
            activated,
            event_block: event_block.into(),
        }
    }

    pub(super) fn satisfies_condition(&self) -> bool {
        self.modifier_keys.satisfies_condition() && {
            self.activated.store(true, Ordering::SeqCst);
            true
        }
    }
}

#[derive(Debug)]
pub(super) struct OnReleaseHook {
    pub(super) action: Action<ButtonEvent>,
    pub(super) activated: Arc<AtomicBool>,
    pub(super) event_block: EventBlock,
}

impl OnReleaseHook {
    pub(super) fn new(
        action: impl Into<Action<ButtonEvent>>,
        activated: Arc<AtomicBool>,
        event_block: impl Into<EventBlock>,
    ) -> Self {
        Self {
            action: action.into(),
            activated,
            event_block: event_block.into(),
        }
    }

    pub(super) fn satisfies_condition(&self) -> bool {
        self.activated.swap(false, Ordering::SeqCst)
    }
}

#[derive(Debug)]
pub struct MouseHook<E> {
    pub(super) modifier: Arc<ModifierKeys>,
    pub(super) action: Action<E>,
    pub(super) event_block: EventBlock,
}

pub(super) type ButtonStorage<T> = HashMap<Button, Vec<Arc<T>>>;
pub(super) type MouseStorage<E> = Vec<Arc<MouseEventHandler<E>>>;
pub(super) type MouseCursorStorage = MouseStorage<MouseCursorEvent>;
pub(super) type MouseWheelStorage = MouseStorage<MouseWheelEvent>;

#[derive(Default, Debug)]
pub(super) struct Storage {
    pub(super) on_press: ButtonStorage<OnPressHook>,
    pub(super) on_release: ButtonStorage<OnReleaseHook>,
    pub(super) mouse_cursor: MouseCursorStorage,
    pub(super) mouse_wheel: MouseWheelStorage,
}
