use super::{KeyboardRegister, MouseRegister};
use crate::{
    handler::Handler,
    modifier::{ModifierEventBlock, ModifierSet},
};
use hookmap_core::{EventBlock, Key, MouseInput};
use std::{cell::RefCell, rc::Weak, sync::Arc};

/// A struct taht handler generated input events.
#[derive(Debug)]
pub struct Modifier {
    handler: Weak<RefCell<Handler>>,
    modifier: Arc<ModifierSet>,
    modifier_event_block: Weak<RefCell<ModifierEventBlock>>,
}

impl Modifier {
    pub(crate) fn new(
        handler: Weak<RefCell<Handler>>,
        modifier: Arc<ModifierSet>,
        modifier_event_block: Weak<RefCell<ModifierEventBlock>>,
    ) -> Self {
        Self {
            handler,
            modifier,
            modifier_event_block,
        }
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
        KeyboardRegister::new(Weak::clone(&self.handler), Arc::clone(&self.modifier), key)
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
            Weak::clone(&self.handler),
            Arc::clone(&self.modifier),
            mouse,
        )
    }

    /// Returns a new instance of `Modifier`.
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
            .upgrade()
            .unwrap()
            .borrow_mut()
            .keyboard
            .insert(key, event_block);
        Self {
            handler: Weak::clone(&self.handler),
            modifier: Arc::new(self.modifier.added_key(key)),
            modifier_event_block: Weak::clone(&self.modifier_event_block),
        }
    }

    /// Returns a new instance of `Modifier`.
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
            .upgrade()
            .unwrap()
            .borrow_mut()
            .mouse
            .insert(mouse_button, event_block);
        Self {
            handler: Weak::clone(&self.handler),
            modifier: Arc::new(self.modifier.added_mouse_button(mouse_button)),
            modifier_event_block: Weak::clone(&self.modifier_event_block),
        }
    }
}
