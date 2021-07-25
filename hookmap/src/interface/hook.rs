use super::{KeyboardRegister, Modifier, MouseRegister};
use crate::{
    handler::Handler,
    modifier::{ModifierEventBlock, ModifierSet},
    runtime::HookInstaller,
};
use hookmap_core::{EventBlock, Key, MouseInput};
use std::{cell::RefCell, rc::Rc, sync::Arc};

/// A struct that handles generated input events.
#[derive(Debug, Default)]
pub struct Hook {
    pub(crate) handler: Rc<RefCell<Handler>>,
    modifier: Arc<ModifierSet>,
    pub(crate) modifier_event_block: Rc<RefCell<ModifierEventBlock>>,
}

impl Hook {
    /// Creates a new instance of `Hook`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a [`KeyboardRegister`] for assigning a function to the key passed as an arguments.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::{Hook, Key};
    /// let hook = Hook::new();
    /// hook.bind_key(Key::A).on_press(|_| println!("The A key is pressed"));
    /// ```
    ///
    pub fn bind_key(&self, key: Key) -> KeyboardRegister {
        KeyboardRegister::new(
            Rc::downgrade(&self.handler),
            Arc::clone(&self.modifier),
            key,
        )
    }

    /// Returns a [`MouseRegister`] for assigning a function to the mouse as an argument.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::{Hook, MouseInput};
    /// let hook = Hook::new();
    /// hook.bind_mouse(MouseInput::Wheel).on_rotate(|_| println!("The mouse wheel rotated"));
    /// ```
    ///
    pub fn bind_mouse(&self, mouse: MouseInput) -> MouseRegister {
        MouseRegister::new(
            Rc::downgrade(&self.handler),
            Arc::clone(&self.modifier),
            mouse,
        )
    }

    /// Returns a new instance of [`Modifier`].
    /// The hooks assigned through this instance will be active only when the given key is pressed.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::{Hook, Key, EventBlock};
    /// let hook = Hook::new();
    /// let modifier_space = hook.modifier_key(Key::Space, EventBlock::Unblock);
    /// modifier_space
    ///     .bind_key(Key::A)
    ///     .on_press(|_| println!("The A key is pressed while the Space key is pressed"));
    /// ```
    ///
    pub fn modifier_key(&self, key: Key, event_block: EventBlock) -> Modifier {
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

    /// Returns a new instance of [`Modifier`].
    /// The hooks assigned through this instance will be active only when the given mouse button is pressed.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::{Hook, Key, MouseInput, EventBlock};
    /// let hook = Hook::new();
    /// let modifier_left = hook.modifier_mouse_button(MouseInput::LButton, EventBlock::Unblock);
    /// modifier_left
    ///     .bind_key(Key::A)
    ///     .on_press(|_| println!("The A key is pressed while the left mouse button is pressed"));
    /// ```
    ///
    pub fn modifier_mouse_button(
        &self,
        mouse_button: MouseInput,
        event_block: EventBlock,
    ) -> Modifier {
        self.modifier_event_block
            .borrow_mut()
            .mouse
            .insert(mouse_button, event_block);
        Modifier::new(
            Rc::downgrade(&self.handler),
            Arc::new(self.modifier.added_mouse_button(mouse_button)),
            Rc::downgrade(&self.modifier_event_block),
        )
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
