use super::{
    button::DownCastableButtonState,
    cond::{Cond, Conditions},
    conditional_hook::ConditionalHook,
    register::{ButtonRegister, MouseCursorRegister, MouseWheelRegister},
    SelectHandleTarget,
};
use crate::{handler::EventCallback, runtime::HookInstaller};
use std::{rc::Rc, sync::Arc};

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
#[derive(Debug, Default)]
pub struct Hook {
    pub(crate) handler: Rc<EventCallback>,
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
    fn bind(&self, button: impl DownCastableButtonState) -> ButtonRegister {
        ButtonRegister::new(
            Rc::downgrade(&self.handler.button),
            Arc::clone(&self.conditions),
            button,
        )
    }

    fn bind_mouse_wheel(&self) -> MouseWheelRegister {
        MouseWheelRegister::new(
            Rc::downgrade(&self.handler.mouse_wheel),
            Arc::clone(&self.conditions),
        )
    }

    fn bind_mouse_cursor(&self) -> MouseCursorRegister {
        MouseCursorRegister::new(
            Rc::downgrade(&self.handler.mouse_cursor),
            Arc::clone(&self.conditions),
        )
    }

    fn cond(&self, cond: Cond) -> ConditionalHook {
        let mut conditions = (*self.conditions).clone();
        conditions.add(cond);
        ConditionalHook::new(Rc::downgrade(&self.handler), Arc::new(conditions))
    }
}
