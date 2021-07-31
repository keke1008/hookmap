use super::{ButtonRegister, MouseCursorRegister, MouseWheelRegister, SelectHandleTarget};
use crate::{
    handler::Handler,
    modifier::{ModifierButtonSet, ModifierSet},
    runtime::HookInstaller,
    Modifier,
};
use hookmap_core::{EventBlock, Key, Mouse};
use std::{cell::RefCell, rc::Rc, sync::Arc};

/// A struct that handles generated input events.
#[derive(Debug, Default)]
pub struct Hook {
    pub(crate) handler: Rc<Handler>,
    modifier_set: Arc<ModifierSet>,
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
    /// use hookmap::{Hook, Key, SelectHandleTarget};
    /// let hook = Hook::new();
    /// hook.bind_key(Key::A).on_press(|_| println!("The A key is pressed"));
    /// hook.handle_input(); // Blocking the current thread.
    /// ```
    ///
    pub fn handle_input(self) {
        let hook_installer = HookInstaller::from(self);
        hook_installer.install_hook();
    }
}

impl SelectHandleTarget for Hook {
    fn bind_key(&self, key: Key) -> ButtonRegister<Key> {
        ButtonRegister::new(
            Rc::downgrade(&self.handler.keyboard),
            Arc::clone(&self.modifier_set),
            key,
        )
    }

    fn bind_mouse(&self, mouse: Mouse) -> ButtonRegister<Mouse> {
        ButtonRegister::new(
            Rc::downgrade(&self.handler.mouse_button),
            Arc::clone(&self.modifier_set),
            mouse,
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

    fn modifier_key(&self, key: Key, event_block: EventBlock) -> Modifier {
        self.modifiers_list
            .borrow_mut()
            .add_keyboard(key, event_block);
        Modifier::new(
            Rc::downgrade(&self.handler),
            Arc::new(self.modifier_set.added_key(key)),
            Rc::downgrade(&self.modifiers_list),
        )
    }

    fn modifier_mouse_button(&self, mouse: Mouse, event_block: EventBlock) -> Modifier {
        self.modifiers_list
            .borrow_mut()
            .add_mouse(mouse, event_block);
        Modifier::new(
            Rc::downgrade(&self.handler),
            Arc::new(self.modifier_set.added_mouse_button(mouse)),
            Rc::downgrade(&self.modifiers_list),
        )
    }
}
