use super::{
    button::ToButtonWithState,
    cond::{Cond, Conditions},
    conditional_hook::ConditionalHook,
    register::{ButtonRegister, MouseCursorRegister, MouseWheelRegister},
    SelectHandleTarget, SetEventBlock,
};
use crate::{handler::EventCallback, runtime::HookInstaller};
use hookmap_core::EventBlock;
use std::{rc::Rc, sync::Arc};

/// A struct for selecting the target of the hook.
///
/// # Example
///
/// ```
/// use hookmap::*;
/// let hook = Hook::new();
/// hook.bind(&Button::A)
///     .on_press(|e| println!("{:?}", e));
/// ```
///
#[derive(Debug, Default)]
pub struct Hook {
    pub(crate) handler: Rc<EventCallback>,
    conditions: Arc<Conditions>,
    event_block: EventBlock,
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
    /// hook.bind(&Button::A).on_press(|_| println!("The A key is pressed"));
    /// hook.handle_input(); // Blocking the current thread.
    /// ```
    ///
    pub fn handle_input(self) {
        let hook_installer = HookInstaller::from(self);
        hook_installer.install_hook();
    }
}

impl SelectHandleTarget for Hook {
    fn bind<B: ToButtonWithState + Clone>(&self, button: &B) -> ButtonRegister {
        ButtonRegister::new(
            Rc::downgrade(&self.handler.button),
            Arc::clone(&self.conditions),
            button.clone(),
            self.event_block,
        )
    }

    fn bind_mouse_wheel(&self) -> MouseWheelRegister {
        MouseWheelRegister::new(
            Rc::downgrade(&self.handler.mouse_wheel),
            Arc::clone(&self.conditions),
            self.event_block,
        )
    }

    fn bind_mouse_cursor(&self) -> MouseCursorRegister {
        MouseCursorRegister::new(
            Rc::downgrade(&self.handler.mouse_cursor),
            Arc::clone(&self.conditions),
            self.event_block,
        )
    }

    fn cond(&self, cond: &Cond) -> ConditionalHook {
        let mut conditions = (*self.conditions).clone();
        conditions.add(cond.clone());
        ConditionalHook::new(Rc::downgrade(&self.handler), Arc::new(conditions))
    }
}

impl SetEventBlock for Hook {
    fn block(mut self) -> Self {
        self.event_block = EventBlock::Block;
        self
    }

    fn unblock(mut self) -> Self {
        self.event_block = EventBlock::Unblock;
        self
    }
}
