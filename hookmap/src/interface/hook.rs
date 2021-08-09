use super::{ButtonRegister, MouseCursorRegister, MouseWheelRegister, SelectHandleTarget};
use crate::{handler::Handler, modifier::ModifierButtonSet, runtime::HookInstaller, Modifier};
use hookmap_core::Button;
use std::{cell::RefCell, rc::Rc, sync::Arc};

/// A struct that handles generated input events.
#[derive(Debug, Default)]
pub struct Hook {
    pub(crate) handler: Rc<Handler>,
    modifier_set: Arc<ModifierButtonSet>,
    pub(crate) modifiers_list: Rc<RefCell<ModifierButtonSet>>,
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
    fn bind(&self, button: Button) -> ButtonRegister {
        ButtonRegister::new(
            Rc::downgrade(&self.handler.button),
            Arc::clone(&self.modifier_set),
            button,
        )
    }

    fn bind_mouse_wheel(&self) -> MouseWheelRegister {
        MouseWheelRegister::new(
            Rc::downgrade(&self.handler.mouse_wheel),
            Arc::clone(&self.modifier_set),
        )
    }

    fn bind_mouse_cursor(&self) -> MouseCursorRegister {
        MouseCursorRegister::new(
            Rc::downgrade(&self.handler.mouse_cursor),
            Arc::clone(&self.modifier_set),
        )
    }

    fn modifier(&self, button: Button) -> Modifier {
        self.modifiers_list.borrow_mut().add(button);
        let mut modifier_set = (*self.modifier_set).clone();
        modifier_set.add(button);
        Modifier::new(
            Rc::downgrade(&self.handler),
            Arc::new(modifier_set),
            Rc::downgrade(&self.modifiers_list),
        )
    }
}
