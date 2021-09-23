mod button_event_handler_entry;
mod conditional_hook;
mod hook;
mod mouse_event_handler_entry;

pub use button_event_handler_entry::ButtonEventHandlerEntry;
pub use conditional_hook::ConditionalHook;
pub use hook::Hook;
pub use mouse_event_handler_entry::{MouseCursorHotKeyEntry, MouseWheelHotkeyEntry};

use crate::{button::ToButtonSet, hotkey::ConditionUnit};
use std::borrow::Borrow;

/// Selecting the target of the hook.
pub trait SelectHandleTarget {
    /// Returns a [`ButtonEventHandlerEntry`] for registering a hook to the button.
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
    fn bind<B: Borrow<B> + ToButtonSet + Clone>(&self, button: B) -> ButtonEventHandlerEntry;

    /// Returns a [`MouseWheelEventHandlerEntry`] for registering a hook to the mouse wheel.
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
    fn bind_mouse_wheel(&self) -> MouseWheelHotkeyEntry;

    /// Returns a [`MouseCursorEventHandlerEntry`] for registering a hook to the mouse wheel.
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
    fn bind_mouse_cursor(&self) -> MouseCursorHotKeyEntry;

    /// Returns a new instance of [`ConditionalHook`].
    /// The hooks assigned through this instance will be activated only when the given conditions are met.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::*;
    /// let hook = Hook::new();
    /// let modifier_space = hook.cond(ConditionUnit::Pressed(Button::Space));
    /// modifier_space
    ///     .bind(Button::A)
    ///     .on_press(|_| println!("The A key is pressed while the Space key is pressed"));
    /// ```
    ///
    fn cond<T: ToButtonSet>(&self, cond: ConditionUnit<T>) -> ConditionalHook;
}

/// Set whether the hook blocks events.
pub trait SetEventBlock {
    /// Blocks the input event when the hook to be registered is enable.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::*;
    /// let hook = Hook::new();
    /// hook.block()
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
    /// let hook = Hook::new();
    /// hook.unblock()
    ///     .bind(Button::A)
    ///     .on_press(|e| println!("{:?}", e));
    /// ```
    ///
    fn unblock(&self) -> ConditionalHook;
}
