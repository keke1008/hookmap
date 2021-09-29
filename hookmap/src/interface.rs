mod button_event_handler_entry;
mod conditional_hook;
mod hotkey;
mod hotkey_info;
mod mouse_event_handler_entry;

pub use button_event_handler_entry::ButtonEventHandlerEntry;
pub use conditional_hook::ConditionalHook;
pub use hotkey::Hotkey;
pub use mouse_event_handler_entry::{MouseCursorHotKeyEntry, MouseWheelHotkeyEntry};

use crate::button::ButtonSet;

/// Selecting the target of the hook.
pub trait SelectHandleTarget {
    /// Returns a [`ButtonEventHandlerEntry`] for registering a hook to the button.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::*;
    /// let hotkey = Hotkey::new();
    /// hotkey.bind(Button::A)
    ///     .on_press(|_| println!("The A key has been pressed"));
    /// ```
    ///
    fn bind(&self, button: impl Into<ButtonSet>) -> ButtonEventHandlerEntry;

    /// Returns a [`MouseWheelHotkeyEntry`] for registering a hook to the mouse wheel.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::*;
    /// let hotkey = Hotkey::new();
    /// hotkey.bind_mouse_wheel()
    ///     .on_rotate(|e| println!("The mouse wheel rotated."));
    /// ```
    ///
    fn bind_mouse_wheel(&self) -> MouseWheelHotkeyEntry;

    /// Returns a [`MouseCursorHotKeyEntry`] for registering a hook to the mouse wheel.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::*;
    /// let hotkey = Hotkey::new();
    /// hotkey.bind_mouse_cursor()
    ///     .on_move(|_| println!("The mouse cursor has moved"));
    /// ```
    ///
    fn bind_mouse_cursor(&self) -> MouseCursorHotKeyEntry;

    /// Add a modifier button to the hotkey to be registered.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::*;
    /// let hotkey = Hotkey::new();
    /// let modifier_shift = hotkey.add_modifiers((&[Button::LShift.into()], &[]));
    /// modifier_shift.bind(Button::A)
    ///     .on_press(|_| println!("Pressed the A key while holding down the Shift key."));
    /// ```
    fn add_modifiers(&self, modifiers: (&[ButtonSet], &[ButtonSet])) -> ConditionalHook;
}

/// Set whether the hook blocks events.
pub trait SetEventBlock {
    /// Blocks the input event when the hook to be registered is enable.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::*;
    /// let hotkey = Hotkey::new();
    /// hotkey.block()
    ///     .bind(Button::A)
    ///     .on_press(|e| println!("{:?}", e));
    /// ```
    ///
    fn block(&self) -> ConditionalHook;

    /// Do not block the input event when the hook to be registered is enable.
    ///
    /// If any other enabled hook blocks the event, this function will be ignored.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::*;
    /// let hotkey = Hotkey::new();
    /// hotkey.unblock()
    ///     .bind(Button::A)
    ///     .on_press(|e| println!("{:?}", e));
    /// ```
    ///
    fn unblock(&self) -> ConditionalHook;
}
