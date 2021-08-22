use super::{Handler, Storage};
use hookmap_core::{Button, ButtonEvent, MouseCursorEvent, MouseWheelEvent};
use std::{cell::RefCell, sync::Arc};

#[derive(Default)]
pub(crate) struct Register {
    storage: RefCell<Storage>,
}

impl Register {
    pub(crate) fn into_inner(self) -> Storage {
        self.storage.into_inner()
    }

    pub(crate) fn register_button_on_press(
        &self,
        button: Button,
        handler: Arc<Handler<ButtonEvent>>,
    ) {
        self.storage
            .borrow_mut()
            .button_on_press
            .entry(button)
            .or_default()
            .push(handler)
    }

    pub(crate) fn register_button_on_release(
        &self,
        button: Button,
        handler: Arc<Handler<ButtonEvent>>,
    ) {
        self.storage
            .borrow_mut()
            .button_on_release
            .entry(button)
            .or_default()
            .push(handler)
    }

    pub(crate) fn register_mouse_cursor(&self, handler: Arc<Handler<MouseCursorEvent>>) {
        self.storage.borrow_mut().mouse_cursor.push(handler);
    }

    pub(crate) fn register_mouse_wheel(&self, handler: Arc<Handler<MouseWheelEvent>>) {
        self.storage.borrow_mut().mouse_wheel.push(handler);
    }
}
