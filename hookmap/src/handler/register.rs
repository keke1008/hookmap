use super::{storage::ButtonStorage, EventCallback, Handler, Storage};
use crate::{button::All, interface::Conditions, ButtonSet, ButtonState};
use hookmap_core::{ButtonEvent, EventBlock, MouseCursorEvent, MouseWheelEvent};
use std::{cell::RefCell, sync::Arc};

#[derive(Default, Debug)]
pub(crate) struct Register {
    storage: RefCell<Storage>,
}

impl Register {
    pub(crate) fn into_inner(self) -> Storage {
        self.storage.into_inner()
    }

    fn generate_callback(
        callback: EventCallback<ButtonEvent>,
        button: &ButtonSet,
        predict: fn(&All) -> bool,
    ) -> EventCallback<ButtonEvent> {
        if let ButtonSet::All(ref all) = button {
            let all = all.clone();
            Arc::new(move |e| {
                if predict(&all) {
                    callback(e)
                }
            })
        } else {
            callback
        }
    }

    fn register_button_handler(
        button: ButtonSet,
        handler: Handler<ButtonEvent>,
        storage: &mut ButtonStorage,
    ) {
        let handler = Arc::new(handler);
        button.iter_buttons().for_each(|&button| {
            storage
                .entry(button)
                .or_default()
                .push(Arc::clone(&handler))
        })
    }

    pub(crate) fn register_button_on_press(
        &self,
        button: ButtonSet,
        callback: EventCallback<ButtonEvent>,
        conditions: Arc<Conditions>,
        event_block: EventBlock,
    ) {
        let callback = Self::generate_callback(callback, &button, All::is_pressed);
        let handler = Handler::new(callback, conditions, event_block);
        let storage = &mut self.storage.borrow_mut().button_on_press;
        Self::register_button_handler(button, handler, storage);
    }

    pub(crate) fn register_button_on_release(
        &self,
        button: ButtonSet,
        callback: EventCallback<ButtonEvent>,
        conditions: Arc<Conditions>,
        event_block: EventBlock,
    ) {
        let callback = Self::generate_callback(callback, &button, All::is_released);
        let handler = Handler::new(callback, conditions, event_block);
        let storage = &mut self.storage.borrow_mut().button_on_release;
        Self::register_button_handler(button, handler, storage);
    }

    pub(crate) fn register_mouse_cursor(&self, handler: Arc<Handler<MouseCursorEvent>>) {
        self.storage.borrow_mut().mouse_cursor.push(handler);
    }

    pub(crate) fn register_mouse_wheel(&self, handler: Arc<Handler<MouseWheelEvent>>) {
        self.storage.borrow_mut().mouse_wheel.push(handler);
    }
}
