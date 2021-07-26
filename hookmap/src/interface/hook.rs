use super::{ButtonRegister, MouseCursorRegister, MouseWheelRegister, SelectHandleTarget};
use crate::{
    handler::Handler,
    modifier::{ModifierEventBlock, ModifierSet},
    runtime::HookInstaller,
    Modifier,
};
use hookmap_core::{EventBlock, Key, Mouse};
use std::{cell::RefCell, rc::Rc, sync::Arc};

/// A struct that handles generated input events.
#[derive(Debug, Default)]
pub struct Hook {
    pub(crate) handler: Rc<Handler>,
    modifier: Arc<ModifierSet>,
    pub(crate) modifier_event_block: Rc<RefCell<ModifierEventBlock>>,
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
    /// use hookmap::{Hook, Key};
    /// let hook = Hook::new();
    /// hook.bind_key(Key::A).on_press(|_| println!("The A key is pressed"));
    /// hook.handle_input(); // Blocking the current thread.
    /// ```
    ///
    pub fn handle_input(self) {
        let hook_installer: HookInstaller = self.into();
        hook_installer.install_hook();
    }
}

impl SelectHandleTarget for Hook {
    fn bind_key(&self, key: Key) -> ButtonRegister<Key> {
        ButtonRegister::new(
            Rc::downgrade(&self.handler.keyboard),
            Arc::clone(&self.modifier),
            key,
        )
    }

    fn bind_mouse(&self, mouse: Mouse) -> ButtonRegister<Mouse> {
        ButtonRegister::new(
            Rc::downgrade(&self.handler.mouse_button),
            Arc::clone(&self.modifier),
            mouse,
        )
    }

    fn bind_mouse_wheel(&self) -> MouseWheelRegister {
        MouseWheelRegister::new(
            Rc::downgrade(&self.handler.mouse_wheel),
            Arc::clone(&self.modifier),
        )
    }

    fn bind_mouse_cursor(&self) -> MouseCursorRegister {
        MouseCursorRegister::new(
            Rc::downgrade(&self.handler.mouse_cursor),
            Arc::clone(&self.modifier),
        )
    }

    fn modifier_key(&self, key: Key, event_block: EventBlock) -> Modifier {
        self.modifier_event_block
            .borrow_mut()
            .keyboard
            .insert(key, event_block);
        Modifier::new(
            Rc::downgrade(&self.handler),
            Arc::new(self.modifier.added_key(key)),
            Rc::downgrade(&self.modifier_event_block),
        )
    }

    fn modifier_mouse_button(&self, mouse: Mouse, event_block: EventBlock) -> Modifier {
        self.modifier_event_block
            .borrow_mut()
            .mouse
            .insert(mouse, event_block);
        Modifier::new(
            Rc::downgrade(&self.handler),
            Arc::new(self.modifier.added_mouse_button(mouse)),
            Rc::downgrade(&self.modifier_event_block),
        )
    }
}
