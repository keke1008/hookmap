use super::{
    button::ToButtonSet,
    cond::{Cond, Conditions},
    register::{ButtonRegister, MouseCursorRegister, MouseWheelRegister},
    SelectHandleTarget, SetEventBlock,
};
use crate::handler::Register;
use hookmap_core::EventBlock;
use std::{
    borrow::Borrow,
    fmt::Debug,
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
/// let mod_ctrl = hook.cond(Cond::pressed(Button::LCtrl));
/// mod_ctrl.bind(Button::H).like(Button::LeftArrow);
/// ```
///
pub struct ConditionalHook {
    handler: Weak<Register>,
    conditions: Arc<Conditions>,
    event_block: EventBlock,
}

impl ConditionalHook {
    /// Creates a new instance of `ConditionalHook`.
    pub(crate) fn new(
        handler: Weak<Register>,
        conditions: Arc<Conditions>,
        event_block: EventBlock,
    ) -> Self {
        Self {
            handler,
            conditions,
            event_block,
        }
    }
}

impl SelectHandleTarget for ConditionalHook {
    fn bind<B: Borrow<B> + ToButtonSet + Clone>(&self, button: B) -> ButtonRegister {
        ButtonRegister::new(
            Rc::downgrade(&self.handler.upgrade().unwrap()),
            Arc::clone(&self.conditions),
            button,
            self.event_block,
        )
    }

    fn bind_mouse_wheel(&self) -> MouseWheelRegister {
        MouseWheelRegister::new(
            Rc::downgrade(&self.handler.upgrade().unwrap()),
            Arc::clone(&self.conditions),
            self.event_block,
        )
    }

    fn bind_mouse_cursor(&self) -> MouseCursorRegister {
        MouseCursorRegister::new(
            Rc::downgrade(&self.handler.upgrade().unwrap()),
            Arc::clone(&self.conditions),
            self.event_block,
        )
    }

    fn cond(&self, cond: impl Borrow<Cond>) -> ConditionalHook {
        let mut conditions = (*self.conditions).clone();
        conditions.add(cond.borrow().clone());
        ConditionalHook::new(
            Weak::clone(&self.handler),
            Arc::new(conditions),
            self.event_block,
        )
    }
}

impl SetEventBlock for ConditionalHook {
    fn block(&self) -> Self {
        let conditions = (*self.conditions).clone();
        ConditionalHook::new(
            Weak::clone(&self.handler),
            Arc::new(conditions),
            EventBlock::Block,
        )
    }

    fn unblock(&self) -> Self {
        let conditions = (*self.conditions).clone();
        ConditionalHook::new(
            Weak::clone(&self.handler),
            Arc::new(conditions),
            EventBlock::Unblock,
        )
    }
}

impl Debug for ConditionalHook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConditionalHook")
            .field("conditions", &*self.conditions)
            .field("event_block", &self.event_block)
            .finish()
    }
}
