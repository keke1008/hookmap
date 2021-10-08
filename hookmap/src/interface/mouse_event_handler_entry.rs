use crate::{
    hotkey::{Action, ModifierKeys, MouseEventHandler},
    runtime::Register,
};
use hookmap_core::{EventBlock, MouseCursorEvent, MouseWheelEvent};
use std::{cell::RefCell, rc::Weak, sync::Arc};
use typed_builder::TypedBuilder;

/// A struct for registering handlers for the mouse cursor.
#[derive(TypedBuilder)]
pub struct MouseCursorHotKeyEntry {
    register: Weak<RefCell<Register>>,

    #[builder(default)]
    modifier_keys: Arc<ModifierKeys>,

    #[builder(default)]
    event_block: EventBlock,
}

impl MouseCursorHotKeyEntry {
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
        let handler = MouseEventHandler::builder()
            .modifier_keys(Arc::clone(&self.modifier_keys))
            .event_block(self.event_block)
            .action(callback.into())
            .build();
        self.register
            .upgrade()
            .unwrap()
            .borrow_mut()
            .register_cursor_event_handler(handler);
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
        let handler = MouseEventHandler::builder()
            .modifier_keys(Arc::clone(&self.modifier_keys))
            .event_block(EventBlock::Block)
            .action(Action::Noop)
            .build();
        self.register
            .upgrade()
            .unwrap()
            .borrow_mut()
            .register_cursor_event_handler(handler);
    }
}

/// A struct for registering handlers for the mouse wheel.
#[derive(TypedBuilder)]
pub struct MouseWheelHotkeyEntry {
    register: Weak<RefCell<Register>>,

    #[builder(default)]
    modifier_keys: Arc<ModifierKeys>,

    #[builder(default)]
    event_block: EventBlock,
}

impl MouseWheelHotkeyEntry {
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
        let handler = MouseEventHandler::builder()
            .modifier_keys(Arc::clone(&self.modifier_keys))
            .event_block(self.event_block)
            .action(callback.into())
            .build();
        self.register
            .upgrade()
            .unwrap()
            .borrow_mut()
            .register_wheel_event_event_handler(handler);
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
        let handler = MouseEventHandler::builder()
            .modifier_keys(Arc::clone(&self.modifier_keys))
            .event_block(EventBlock::Block)
            .action(Action::Noop)
            .build();
        self.register
            .upgrade()
            .unwrap()
            .borrow_mut()
            .register_wheel_event_event_handler(handler);
    }
}
