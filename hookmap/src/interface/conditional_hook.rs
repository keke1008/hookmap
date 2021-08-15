use super::{
    ButtonRegister, Cond, Conditions, DownCastableButtonState, MouseCursorRegister,
    MouseWheelRegister, SelectHandleTarget,
};
use crate::handler::EventCallback;
use std::{
    rc::{Rc, Weak},
    sync::Arc,
};

#[derive(Debug)]
pub struct ConditionalHook {
    handler: Weak<EventCallback>,
    conditions: Arc<Conditions>,
}

impl ConditionalHook {
    pub(crate) fn new(handler: Weak<EventCallback>, conditions: Arc<Conditions>) -> Self {
        Self {
            handler,
            conditions,
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
        ConditionalHook::new(Weak::clone(&self.handler), Arc::new(conditions))
    }
}
