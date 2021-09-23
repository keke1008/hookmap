use crate::hotkey::{Hotkey, MouseHotkey};
use hookmap_core::{Button, ButtonEvent, MouseCursorEvent, MouseWheelEvent};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

pub(crate) type ButtonStorage = HashMap<Button, Vec<Arc<Hotkey>>>;

type MouseStorage<E> = Vec<Arc<MouseHotkey<E>>>;
pub(crate) type MouseCursorStorage = MouseStorage<MouseCursorEvent>;
pub(crate) type MouseWheelStorage = MouseStorage<MouseWheelEvent>;

#[derive(Default, Debug)]
pub(crate) struct Storage {
    pub(crate) button: ButtonStorage,
    pub(crate) mouse_cursor: MouseCursorStorage,
    pub(crate) mouse_wheel: MouseWheelStorage,
}

pub(crate) struct ButtonFetcher {
    storage: ButtonStorage,
}

impl ButtonFetcher {
    pub(crate) fn new(storage: ButtonStorage) -> Self {
        Self { storage }
    }

    pub(crate) fn fetch(&self, event: &ButtonEvent) -> Vec<Arc<Hotkey>> {
        self.storage
            .get(&event.target)
            .map(|hotkeys| {
                hotkeys
                    .iter()
                    .filter(|hotkey| hotkey.is_satisfied(event))
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }
}

pub(crate) struct MouseFetcher<E> {
    storage: MouseStorage<E>,
}

impl<E> MouseFetcher<E> {
    pub(crate) fn new(storage: MouseStorage<E>) -> Self {
        Self { storage }
    }

    pub(crate) fn fetch(&self) -> Vec<Arc<MouseHotkey<E>>> {
        self.storage
            .iter()
            .filter(|handler| handler.is_satisfied())
            .cloned()
            .collect()
    }
}

#[derive(Default, Debug)]
pub(crate) struct Register {
    storage: RefCell<Storage>,
}

impl Register {
    pub(crate) fn into_inner(self) -> Storage {
        self.storage.into_inner()
    }

    pub(crate) fn register_hotkey(&self, hotkey: Hotkey) {
        let mut storage = self.storage.borrow_mut();
        let hotkey = Arc::new(hotkey);
        for trigger in hotkey.trigger.iter_buttons() {
            storage
                .button
                .entry(*trigger)
                .or_default()
                .push(Arc::clone(&hotkey))
        }
    }

    pub(crate) fn register_cursor_event_handler(&self, handler: MouseHotkey<MouseCursorEvent>) {
        self.storage
            .borrow_mut()
            .mouse_cursor
            .push(Arc::new(handler))
    }

    pub(crate) fn register_wheel_event_event_handler(&self, handler: MouseHotkey<MouseWheelEvent>) {
        self.storage
            .borrow_mut()
            .mouse_wheel
            .push(Arc::new(handler))
    }
}
