use super::cond::Conditions;
use crate::handler::{Callback, Handler, Register as HandlerRegister};
use hookmap_core::{EventBlock, MouseCursorEvent, MouseWheelEvent};
use std::{fmt::Debug, rc::Weak, sync::Arc};

/// A struct for registering handlers for the mouse cursor.
pub struct MouseCursorEventHandlerEntry {
    handler_register: Weak<HandlerRegister>,
    conditions: Arc<Conditions>,
    event_block: EventBlock,
}

impl MouseCursorEventHandlerEntry {
    pub(crate) fn new(
        handler_register: Weak<HandlerRegister>,
        conditions: Arc<Conditions>,
        event_block: EventBlock,
    ) -> Self {
        Self {
            handler_register,
            conditions,
            event_block,
        }
    }

    fn register_handler(&self, callback: Callback<MouseCursorEvent>, event_block: EventBlock) {
        let handler = Handler::new(callback, Arc::clone(&self.conditions), event_block);
        self.handler_register
            .upgrade()
            .unwrap()
            .register_mouse_cursor(Arc::new(handler));
    }

    /// Registers a handler called when the mouse cursor is moved.
    ///
    /// # Arguments
    ///
    /// * `callback` - A function that takes a absolute postion of the mouse cursor as an argument.
    ///
    /// # Example
    /// ```
    /// use hookmap::{Hook, SelectHandleTarget};
    /// let hook = Hook::new();
    /// hook.bind_mouse_cursor().on_move(|event| {
    ///     println!("Current mouse cursor position(x, y): {:?}", event);
    /// });
    /// ```
    pub fn on_move<F>(&self, callback: F)
    where
        F: Fn(MouseCursorEvent) + Send + Sync + 'static,
    {
        self.register_handler(Arc::new(callback), self.event_block);
    }

    /// Disables and blocks mouse move events.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::*;
    /// let hook = Hook::new();
    /// hook.bind_mouse_cursor().disable();
    /// ```
    ///
    pub fn disable(&self) {
        self.register_handler(Arc::new(|_| {}), EventBlock::Block);
    }
}

impl Debug for MouseCursorEventHandlerEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MouseCursorRegister")
            .field("conditions", &*self.conditions)
            .field("event_block", &self.event_block)
            .finish()
    }
}

/// A struct for registering handlers for the mouse wheel.
pub struct MouseWheelEventHandlerEntry {
    handler_register: Weak<HandlerRegister>,
    conditions: Arc<Conditions>,
    event_block: EventBlock,
}

impl MouseWheelEventHandlerEntry {
    pub(crate) fn new(
        handler: Weak<HandlerRegister>,
        conditions: Arc<Conditions>,
        event_block: EventBlock,
    ) -> Self {
        Self {
            handler_register: handler,
            conditions,
            event_block,
        }
    }

    fn register_handler(&self, callback: Callback<MouseWheelEvent>, event_block: EventBlock) {
        let handler = Handler::new(callback, Arc::clone(&self.conditions), event_block);
        self.handler_register
            .upgrade()
            .unwrap()
            .register_mouse_wheel(Arc::new(handler));
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
    /// use hookmap::{Hook, SelectHandleTarget};
    /// let hook = Hook::new();
    /// hook.bind_mouse_wheel().on_rotate(|event| {
    ///     println!("Mouse wheel rotation speed: {}", event);
    /// });
    /// ```
    ///
    pub fn on_rotate<F>(&self, callback: F)
    where
        F: Fn(MouseWheelEvent) + Send + Sync + 'static,
    {
        self.register_handler(Arc::new(callback), self.event_block);
    }

    /// Disables and blocks mouse wheel events.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::*;
    /// let hook = Hook::new();
    /// hook.bind_mouse_wheel().disable();
    /// ```
    ///
    pub fn disable(&self) {
        self.register_handler(Arc::new(|_| {}), EventBlock::Block);
    }
}

impl Debug for MouseWheelEventHandlerEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MouseWheelRegister")
            .field("conditions", &*self.conditions)
            .field("event_block", &self.event_block)
            .finish()
    }
}
