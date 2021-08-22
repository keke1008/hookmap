use super::super::{cond::Conditions, SetEventBlock};
use crate::handler::{Handler, Register as HandlerRegister};
use hookmap_core::{EventBlock, MouseCursorEvent, MouseWheelEvent};
use std::{rc::Weak, sync::Arc};

/// A struct for registering handlers for the mouse cursor.
pub struct MouseCursorRegister {
    handler_register: Weak<HandlerRegister>,
    conditions: Arc<Conditions>,
    event_block: EventBlock,
}

impl MouseCursorRegister {
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
        let handler = Handler::new(
            Arc::new(callback),
            Arc::clone(&self.conditions),
            self.event_block,
        );
        self.handler_register
            .upgrade()
            .unwrap()
            .register_mouse_cursor(Arc::new(handler));
    }
}

impl SetEventBlock for MouseCursorRegister {
    fn block(mut self) -> Self {
        self.event_block = EventBlock::Block;
        self
    }

    fn unblock(mut self) -> Self {
        self.event_block = EventBlock::Unblock;
        self
    }
}

/// A struct for registering handlers for the mouse wheel.
pub struct MouseWheelRegister {
    handler_register: Weak<HandlerRegister>,
    conditions: Arc<Conditions>,
    event_block: EventBlock,
}

impl MouseWheelRegister {
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
        let handler = Handler::new(
            Arc::new(callback),
            Arc::clone(&self.conditions),
            self.event_block,
        );
        self.handler_register
            .upgrade()
            .unwrap()
            .register_mouse_wheel(Arc::new(handler));
    }
}

impl SetEventBlock for MouseWheelRegister {
    fn block(mut self) -> Self {
        self.event_block = EventBlock::Block;
        self
    }

    fn unblock(mut self) -> Self {
        self.event_block = EventBlock::Unblock;
        self
    }
}
