use super::{
    button::ToButtonWithState,
    cond::{Cond, Conditions},
    conditional_hook::ConditionalHook,
    register::{ButtonRegister, MouseCursorRegister, MouseWheelRegister},
    SelectHandleTarget, SetEventBlock,
};
use crate::{handler::Register, runtime::HookInstaller};
use hookmap_core::EventBlock;
use std::{fmt::Debug, rc::Rc, sync::Arc};

/// A struct for selecting the target of the hook.
///
/// # Example
///
/// ```
/// use hookmap::*;
/// let hook = Hook::new();
/// hook.bind(Button::A)
///     .on_press(|e| println!("{:?}", e));
/// ```
///
#[derive(Default)]
pub struct Hook {
    pub(crate) register: Rc<Register>,
    conditions: Arc<Conditions>,
}

impl Hook {
    /// Creates a new instance of `Hook`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles input events and blocks the current thread.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use hookmap::{Hook, Button, SelectHandleTarget};
    /// let hook = Hook::new();
    /// hook.bind(Button::A).on_press(|_| println!("The A key is pressed"));
    /// hook.handle_input(); // Blocking the current thread.
    /// ```
    ///
    pub fn handle_input(self) {
        let hook_installer = HookInstaller::from(self);
        hook_installer.install_hook();
    }
}

impl SelectHandleTarget for Hook {
    fn bind<B: std::borrow::Borrow<B> + ToButtonWithState + Clone>(
        &self,
        button: B,
    ) -> ButtonRegister {
        ButtonRegister::new(
            Rc::downgrade(&self.register),
            Arc::clone(&self.conditions),
            button,
            EventBlock::default(),
        )
    }

    fn bind_mouse_wheel(&self) -> MouseWheelRegister {
        MouseWheelRegister::new(
            Rc::downgrade(&self.register),
            Arc::clone(&self.conditions),
            EventBlock::default(),
        )
    }

    fn bind_mouse_cursor(&self) -> MouseCursorRegister {
        MouseCursorRegister::new(
            Rc::downgrade(&self.register),
            Arc::clone(&self.conditions),
            EventBlock::default(),
        )
    }

    fn cond(&self, cond: impl std::borrow::Borrow<Cond>) -> ConditionalHook {
        let mut conditions = (*self.conditions).clone();
        conditions.add(cond.borrow().clone());
        ConditionalHook::new(
            Rc::downgrade(&self.register),
            Arc::new(conditions),
            EventBlock::default(),
        )
    }
}

impl SetEventBlock for Hook {
    fn block(&self) -> ConditionalHook {
        let register = Rc::downgrade(&self.register);
        let conditions = (*self.conditions).clone();
        ConditionalHook::new(register, Arc::new(conditions), EventBlock::Block)
    }

    fn unblock(&self) -> ConditionalHook {
        let register = Rc::downgrade(&self.register);
        let conditions = (*self.conditions).clone();
        ConditionalHook::new(register, Arc::new(conditions), EventBlock::Unblock)
    }
}

impl Debug for Hook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Hook")
            .field("event_block", &EventBlock::default())
            .finish()
    }
}
