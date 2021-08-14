use super::{ButtonRegister, MouseCursorRegister, MouseWheelRegister, SelectHandleTarget};
use crate::{
    button::DownCastableButtonState,
    cond::{Cond, Conditions},
    handler::EventCallback,
    modifier::ModifierButtonSet,
};

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
    sync::Arc,
};

#[derive(Debug)]
pub struct ConditionalHook {
    handler: Weak<EventCallback>,
    conditions: Arc<Conditions>,
    modifier_list: Weak<RefCell<ModifierButtonSet>>,
}

impl ConditionalHook {
    pub(crate) fn new(
        handler: Weak<EventCallback>,
        conditions: Arc<Conditions>,
        modifiers_list: Weak<RefCell<ModifierButtonSet>>,
    ) -> Self {
        Self {
            handler,
            conditions,
            modifier_list: modifiers_list,
        }
    }
}

impl SelectHandleTarget for ConditionalHook {
    fn bind(&self, button: impl DownCastableButtonState) -> ButtonRegister {
        ButtonRegister::new(
            Rc::downgrade(&self.handler.upgrade().unwrap().button),
            Arc::clone(&self.conditions),
            button,
        )
    }

    fn bind_mouse_wheel(&self) -> MouseWheelRegister {
        MouseWheelRegister::new(
            Rc::downgrade(&self.handler.upgrade().unwrap().mouse_wheel),
            Arc::clone(&self.conditions),
        )
    }

    fn bind_mouse_cursor(&self) -> MouseCursorRegister {
        MouseCursorRegister::new(
            Rc::downgrade(&self.handler.upgrade().unwrap().mouse_cursor),
            Arc::clone(&self.conditions),
        )
    }

    fn cond(&self, cond: Cond) -> ConditionalHook {
        let mut conditions = (*self.conditions).clone();
        conditions.add(cond);
        ConditionalHook::new(
            Weak::clone(&self.handler),
            Arc::new(conditions),
            Weak::clone(&self.modifier_list),
        )
    }
}
