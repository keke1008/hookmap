mod hook;
mod modifier;
mod register;

pub use hook::Hook;
pub use modifier::Modifier;
pub use register::{ButtonRegister, MouseCursorRegister, MouseWheelRegister};

use hookmap_core::Button;

pub trait SelectHandleTarget {
    /// Returns a [`ButtonRegister`] for registering a hook to the button.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::{Hook, Button, SelectHandleTarget};
    /// let hook = Hook::new();
    /// hook.bind(Button::A)
    ///     .on_press(|_| println!("The A key has been pressed"));
    /// ```
    ///
    fn bind(&self, button: Button) -> ButtonRegister;

    /// Returns a [`MouseWheelRegister`] for registering a hook to the mouse wheel.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::{Hook, SelectHandleTarget};
    /// let hook = Hook::new();
    /// hook.bind_mouse_wheel()
    ///     .on_rotate(|e| println!("The mouse wheel rotated."));
    /// ```
    ///
    fn bind_mouse_wheel(&self) -> MouseWheelRegister;

    /// Returns a [`MouseCursorRegister`] for registering a hook to the mouse wheel.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::{Hook, SelectHandleTarget};
    /// let hook = Hook::new();
    /// hook.bind_mouse_cursor()
    ///     .on_move(|_| println!("The mouse cursor has moved"));
    /// ```
    ///
    fn bind_mouse_cursor(&self) -> MouseCursorRegister;

    /// Returns a new instance of [`Modifier`].
    /// The hooks assigned through this instance will be active only when the given key is pressed.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::{Hook, Button, SelectHandleTarget};
    /// let hook = Hook::new();
    /// let modifier_space = hook.modifier(Button::Space);
    /// modifier_space
    ///     .bind(Button::A)
    ///     .on_press(|_| println!("The A key is pressed while the Space key is pressed"));
    /// ```
    ///
    fn modifier(&self, button: Button) -> Modifier;
}
