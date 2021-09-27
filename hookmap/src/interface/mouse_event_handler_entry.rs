use super::hotkey_info::ConditionalHotkeyInfo;
use crate::hotkey::Action;
use crate::runtime::Register;
use hookmap_core::{EventBlock, MouseCursorEvent, MouseWheelEvent};
use std::cell::RefCell;
use std::rc::Weak;

/// A struct for registering handlers for the mouse cursor.
pub struct MouseCursorHotKeyEntry {
    register: Weak<RefCell<Register>>,
    partial_event_handler: ConditionalHotkeyInfo,
}

impl MouseCursorHotKeyEntry {
    pub(super) fn new(
        handler_register: Weak<RefCell<Register>>,
        partial_event_handler: ConditionalHotkeyInfo,
    ) -> Self {
        Self {
            register: handler_register,
            partial_event_handler,
        }
    }

    fn register_handler(
        &self,
        action: Action<MouseCursorEvent>,
        partial_event_handler: ConditionalHotkeyInfo,
    ) {
        let handler = partial_event_handler.build_mouse_event_handler(action);
        self.register
            .upgrade()
            .unwrap()
            .borrow_mut()
            .register_cursor_event_handler(handler);
    }

    /// Registers a handler called when the mouse cursor is moved.
    ///
    /// # Arguments
    ///
    /// * `callback` - A function that takes a absolute postion of the mouse cursor as an argument.
    ///
    /// # Example
    /// ```
    /// use hookmap::*;
    /// let hotkey = Hotkey::new();
    /// hotkey.bind_mouse_cursor().on_move(|event| {
    ///     println!("Current mouse cursor position(x, y): {:?}", event);
    /// });
    /// ```
    pub fn on_move<F>(&self, callback: F)
    where
        F: Fn(MouseCursorEvent) + Send + Sync + 'static,
    {
        self.register_handler(callback.into(), self.partial_event_handler.clone());
    }

    /// Disables and blocks mouse move events.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::*;
    /// let hotkey = Hotkey::new();
    /// hotkey.bind_mouse_cursor().disable();
    /// ```
    ///
    pub fn disable(&self) {
        let mut partial_event_handler = self.partial_event_handler.clone();
        partial_event_handler.event_block = EventBlock::Block;
        self.register_handler((|_| {}).into(), partial_event_handler);
    }
}

/// A struct for registering handlers for the mouse wheel.
pub struct MouseWheelHotkeyEntry {
    register: Weak<RefCell<Register>>,
    partial_event_handler: ConditionalHotkeyInfo,
}

impl MouseWheelHotkeyEntry {
    pub(super) fn new(
        handler: Weak<RefCell<Register>>,
        partial_event_handler: ConditionalHotkeyInfo,
    ) -> Self {
        Self {
            register: handler,
            partial_event_handler,
        }
    }

    fn register_handler(
        &self,
        action: Action<MouseWheelEvent>,
        partial_event_handler: ConditionalHotkeyInfo,
    ) {
        self.register
            .upgrade()
            .unwrap()
            .borrow_mut()
            .register_wheel_event_event_handler(
                partial_event_handler.build_mouse_event_handler(action),
            );
    }

    /// Registers a handler called when the mouse wheel is rotated.
    ///
    /// # Arguments
    ///
    /// * `callback` - A function that takes a rotation speed of the mouse
    /// wheel as an argument.
    ///
    /// # Example
    /// ```
    /// use hookmap::*;
    /// let hotkey = Hotkey::new();
    /// hotkey.bind_mouse_wheel().on_rotate(|event| {
    ///     println!("Mouse wheel rotation speed: {}", event);
    /// });
    /// ```
    ///
    pub fn on_rotate<F>(&self, callback: F)
    where
        F: Fn(MouseWheelEvent) + Send + Sync + 'static,
    {
        self.register_handler(callback.into(), self.partial_event_handler.clone());
    }

    /// Disables and blocks mouse wheel events.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::*;
    /// let hotkey = Hotkey::new();
    /// hotkey.bind_mouse_wheel().disable();
    /// ```
    ///
    pub fn disable(&self) {
        let mut partial_event_handler = self.partial_event_handler.clone();
        partial_event_handler.event_block = EventBlock::Block;
        self.register_handler((|_| {}).into(), partial_event_handler);
    }
}
