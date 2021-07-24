use super::hook_installer::HookInstaller;
use crate::{
    handler::Handler,
    modifier::{Modifier, ModifierEventBlock},
    register::{KeyboardRegister, MouseRegister},
};
use hookmap_core::{EventBlock, Key, MouseInput};
use std::sync::{Arc, Mutex};

/// A struct that handles generated input events.
#[derive(Debug, Default)]
pub struct Hook {
    pub(crate) handler: Arc<Mutex<Handler>>,
    modifier: Modifier,
    pub(crate) modifier_event_block: Arc<Mutex<ModifierEventBlock>>,
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
        KeyboardRegister::new(key, self.modifier.clone(), Arc::clone(&self.handler))
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
        MouseRegister::new(mouse, self.modifier.clone(), Arc::clone(&self.handler))
    }

    /// Returns a new instance of `Hook`.
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
    pub fn modifier_key(&self, key: Key, event_block: EventBlock) -> Self {
        self.modifier_event_block
            .lock()
            .unwrap()
            .keyboard
            .insert(key, event_block);
        Self {
            modifier: self.modifier.added_key(key),
            handler: Arc::clone(&self.handler),
            modifier_event_block: Arc::clone(&self.modifier_event_block),
        }
    }

    /// Returns a new instance of `Hook`.
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
    pub fn modifier_mouse_button(&self, mouse_button: MouseInput, event_block: EventBlock) -> Self {
        self.modifier_event_block
            .lock()
            .unwrap()
            .mouse
            .insert(mouse_button, event_block);
        Self {
            modifier: self.modifier.added_mouse_button(mouse_button),
            handler: Arc::clone(&self.handler),
            modifier_event_block: Arc::clone(&self.modifier_event_block),
        }
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
