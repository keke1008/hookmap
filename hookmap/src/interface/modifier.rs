use super::{ButtonRegister, MouseCursorRegister, MouseWheelRegister, SelectHandleTarget};
use crate::{
    handler::Handler,
    modifier::{ModifierEventBlock, ModifierSet},
};
use hookmap_core::{EventBlock, Key, Mouse};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
    sync::Arc,
};

/// A struct taht handler generated input events.
#[derive(Debug)]
pub struct Modifier {
    handler: Weak<Handler>,
    modifier: Arc<ModifierSet>,
    modifier_event_block: Weak<RefCell<ModifierEventBlock>>,
}

impl Modifier {
    pub(crate) fn new(
        handler: Weak<Handler>,
        modifier: Arc<ModifierSet>,
        modifier_event_block: Weak<RefCell<ModifierEventBlock>>,
    ) -> Self {
        Self {
            handler,
            modifier,
            modifier_event_block,
        }
    }
}

impl SelectHandleTarget for Modifier {
    fn bind_key(&self, key: Key) -> ButtonRegister<Key> {
        ButtonRegister::new(
            Rc::downgrade(&self.handler.upgrade().unwrap().keyboard),
            Arc::clone(&self.modifier),
            key,
        )
    }

    fn bind_mouse(&self, mouse: Mouse) -> ButtonRegister<Mouse> {
        ButtonRegister::new(
            Rc::downgrade(&self.handler.upgrade().unwrap().mouse_button),
            Arc::clone(&self.modifier),
            mouse,
        )
    }

    fn bind_mouse_wheel(&self) -> MouseWheelRegister {
        MouseWheelRegister::new(
            Rc::downgrade(&self.handler.upgrade().unwrap().mouse_wheel),
            Arc::clone(&self.modifier),
        )
    }

    fn bind_mouse_cursor(&self) -> MouseCursorRegister {
        MouseCursorRegister::new(
            Rc::downgrade(&self.handler.upgrade().unwrap().mouse_cursor),
            Arc::clone(&self.modifier),
        )
    }

    fn modifier_key(&self, key: Key, event_block: EventBlock) -> Self {
        self.modifier_event_block
            .upgrade()
            .unwrap()
            .borrow_mut()
            .keyboard
            .insert(key, event_block);
        Self {
            handler: Weak::clone(&self.handler),
            modifier: Arc::new(self.modifier.added_key(key)),
            modifier_event_block: Weak::clone(&self.modifier_event_block),
        }
    }

    fn modifier_mouse_button(&self, mouse: Mouse, event_block: EventBlock) -> Self {
        self.modifier_event_block
            .upgrade()
            .unwrap()
            .borrow_mut()
            .mouse
            .insert(mouse, event_block);
        Self {
            handler: Weak::clone(&self.handler),
            modifier: Arc::new(self.modifier.added_mouse_button(mouse)),
            modifier_event_block: Weak::clone(&self.modifier_event_block),
        }
    }
}
