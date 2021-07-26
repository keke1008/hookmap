mod hook;
mod modifier;
mod register;

pub use hook::Hook;
pub use modifier::Modifier;
pub use register::{ButtonRegister, MouseCursorRegister, MouseWheelRegister};

use hookmap_core::{EventBlock, Key, Mouse};

pub trait SelectHandleTarget {
    /// Returns a [`KeyboardRegister`] for registering a hook to the key.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::{Hook, Key};
    /// let hook = Hook::new();
    /// hook.bind_key(Key::A).on_press(|_| println!("The A key is pressed"));
    /// ```
    ///
    fn bind_key(&self, key: Key) -> ButtonRegister<Key>;

    /// Returns a [`ButtonRegister`] for registering a hook to the mouse button.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::{Hook, MouseInput};
    /// let hook = Hook::new();
    /// hook.bind_mouse(MouseInput::Wheel).on_rotate(|_| println!("The mouse wheel rotated"));
    /// ```
    ///
    fn bind_mouse(&self, mouse: Mouse) -> ButtonRegister<Mouse>;

    /// Returns a [`MouseWheelRegister`] for registering a hook to the mouse wheel.
    fn bind_mouse_wheel(&self) -> MouseWheelRegister;

    /// Returns a [`MouseCursorRegister`] for registering a hook to the mouse wheel.
    fn bind_mouse_cursor(&self) -> MouseCursorRegister;

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
    fn modifier_key(&self, key: Key, event_block: EventBlock) -> Modifier;

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
    fn modifier_mouse_button(&self, mouse: Mouse, event_block: EventBlock) -> Modifier;
}
