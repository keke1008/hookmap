use super::{ButtonRegister, MouseCursorRegister, MouseWheelRegister, SelectHandleTarget};
use crate::{
    handler::Handler,
    modifier::{ModifierButtonSet, ModifierSet},
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
    modifier_set: Arc<ModifierSet>,
    modifiers_list: Weak<RefCell<ModifierButtonSet>>,
}

impl Modifier {
    pub(crate) fn new(
        handler: Weak<Handler>,
        modifier: Arc<ModifierSet>,
        modifier_button: Weak<RefCell<ModifierButtonSet>>,
    ) -> Self {
        Self {
            handler,
            modifier_set: modifier,
            modifiers_list: modifier_button,
        }
    }
}

impl SelectHandleTarget for Modifier {
    fn bind_key(&self, key: Key) -> ButtonRegister<Key> {
        ButtonRegister::new(
            Rc::downgrade(&self.handler.upgrade().unwrap().keyboard),
            Arc::clone(&self.modifier_set),
            key,
        )
    }

    fn bind_mouse(&self, mouse: Mouse) -> ButtonRegister<Mouse> {
        ButtonRegister::new(
            Rc::downgrade(&self.handler.upgrade().unwrap().mouse_button),
            Arc::clone(&self.modifier_set),
            mouse,
        )
    }

    fn bind_mouse_wheel(&self) -> MouseWheelRegister {
        MouseWheelRegister::new(
            Rc::downgrade(&self.handler.upgrade().unwrap().mouse_wheel),
            Arc::clone(&self.modifier_set),
        )
    }

    fn bind_mouse_cursor(&self) -> MouseCursorRegister {
        MouseCursorRegister::new(
            Rc::downgrade(&self.handler.upgrade().unwrap().mouse_cursor),
            Arc::clone(&self.modifier_set),
        )
    }

    fn modifier_key(&self, key: Key, event_block: EventBlock) -> Self {
        self.modifiers_list
            .upgrade()
            .unwrap()
            .borrow_mut()
            .add_keyboard(key, event_block);
        Self {
            handler: Weak::clone(&self.handler),
            modifier_set: Arc::new(self.modifier_set.added_key(key)),
            modifiers_list: Weak::clone(&self.modifiers_list),
        }
    }

    fn modifier_mouse_button(&self, mouse: Mouse, event_block: EventBlock) -> Self {
        self.modifiers_list
            .upgrade()
            .unwrap()
            .borrow_mut()
            .add_mouse(mouse, event_block);
        Self {
            handler: Weak::clone(&self.handler),
            modifier_set: Arc::new(self.modifier_set.added_mouse_button(mouse)),
            modifiers_list: Weak::clone(&self.modifiers_list),
        }
    }
}
