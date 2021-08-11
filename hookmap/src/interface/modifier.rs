use super::{ButtonRegister, MouseCursorRegister, MouseWheelRegister, SelectHandleTarget};
use crate::{handler::EventCallback, modifier::ModifierButtonSet};
use hookmap_core::Button;
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
    sync::Arc,
};

/// A struct taht handler generated input events.
#[derive(Debug)]
pub struct Modifier {
    handler: Weak<EventCallback>,
    modifier_set: Arc<ModifierButtonSet>,
    modifiers_list: Weak<RefCell<ModifierButtonSet>>,
}

impl Modifier {
    pub(crate) fn new(
        handler: Weak<EventCallback>,
        modifier: Arc<ModifierButtonSet>,
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
    fn bind(&self, button: Button) -> ButtonRegister {
        ButtonRegister::new(
            Rc::downgrade(&self.handler.upgrade().unwrap().button),
            Arc::clone(&self.modifier_set),
            button,
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

    fn modifier(&self, button: Button) -> Self {
        self.modifiers_list
            .upgrade()
            .unwrap()
            .borrow_mut()
            .add(button);
        let mut modifier_set = (*self.modifier_set).clone();
        modifier_set.add(button);
        Self {
            handler: Weak::clone(&self.handler),
            modifier_set: Arc::new(modifier_set),
            modifiers_list: Weak::clone(&self.modifiers_list),
        }
    }
}
