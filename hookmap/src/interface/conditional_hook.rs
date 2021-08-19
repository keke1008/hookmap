use super::{
    button::ToButtonWithState,
    cond::{Cond, Conditions},
    register::{ButtonRegister, MouseCursorRegister, MouseWheelRegister},
    SelectHandleTarget, SetEventBlock,
};
use crate::handler::EventCallback;
use hookmap_core::EventBlock;
use std::{
    rc::{Rc, Weak},
    sync::Arc,
};

/// A struct for selecting the target of the conditional hook.
///
/// # Example
///
/// ```
/// use hookmap::*;
/// let hook = Hook::new();
/// let mod_ctrl = hook.cond(Cond::pressed(Button::Ctrl));
/// mod_ctrl.bind(Button::H).like(Button::LeftArrow);
/// ```
///
#[derive(Debug)]
pub struct ConditionalHook {
    handler: Weak<EventCallback>,
    conditions: Arc<Conditions>,
    event_block: EventBlock,
}

impl ConditionalHook {
    /// Creates a new instance of `ConditionalHook`.
    pub(crate) fn new(handler: Weak<EventCallback>, conditions: Arc<Conditions>) -> Self {
        Self {
            handler,
            conditions,
            event_block: EventBlock::default(),
        }
    }
}

impl SelectHandleTarget for ConditionalHook {
    fn bind<B: ToButtonWithState + Clone>(&self, button: &B) -> ButtonRegister {
        ButtonRegister::new(
            Rc::downgrade(&self.handler.upgrade().unwrap().button),
            Arc::clone(&self.conditions),
            button.clone(),
            self.event_block,
        )
    }

    fn bind_mouse_wheel(&self) -> MouseWheelRegister {
        MouseWheelRegister::new(
            Rc::downgrade(&self.handler.upgrade().unwrap().mouse_wheel),
            Arc::clone(&self.conditions),
            self.event_block,
        )
    }

    fn bind_mouse_cursor(&self) -> MouseCursorRegister {
        MouseCursorRegister::new(
            Rc::downgrade(&self.handler.upgrade().unwrap().mouse_cursor),
            Arc::clone(&self.conditions),
            self.event_block,
        )
    }

    fn cond(&self, cond: &Cond) -> ConditionalHook {
        let mut conditions = (*self.conditions).clone();
        conditions.add(cond.clone());
        ConditionalHook::new(Weak::clone(&self.handler), Arc::new(conditions))
    }
}

impl SetEventBlock for ConditionalHook {
    fn block(mut self) -> Self {
        self.event_block = EventBlock::Block;
        self
    }

    fn unblock(mut self) -> Self {
        self.event_block = EventBlock::Unblock;
        self
    }
}
