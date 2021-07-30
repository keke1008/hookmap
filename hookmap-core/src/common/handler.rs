use super::{event::EventBlock, keyboard::InstallKeyboardHook, mouse::InstallMouseHook};
use crate::{KeyboardEvent, MouseEvent};
use once_cell::sync::Lazy;
use std::{fmt::Debug, sync::RwLock};

pub static INPUT_HANDLER: Lazy<InputHandler> = Lazy::new(InputHandler::new);

/// Installs a hook in the way of each platform.
/// This needs to implement for `InputHandler`.
pub trait HandleInput {
    /// Installs a hook and blocks a thread.
    fn handle_input();
}

type EventCallback<E> = Box<dyn Fn(E) -> EventBlock + Send + Sync>;

/// An optional input event handler.
pub struct HandlerFunction<E> {
    handler: Option<EventCallback<E>>,
}

impl<E> HandlerFunction<E> {
    /// Creates a new `HandlerFunction<T, A>` with `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap_core::{HandlerFunction, KeyboardEvent};
    /// let handler = HandlerFunction::<KeyboardEvent>::new();
    /// ```
    ///
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a callback function.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap_core::{EventBlock, HandlerFunction, KeyboardEvent};
    ///
    /// let mut handler = HandlerFunction::<KeyboardEvent>::new();
    /// handler.register_handler(|e| {
    ///     println!("Event target: {:?}", e.target);
    ///     println!("Event action: {:?}", e.action);
    ///     EventBlock::Unblock
    /// });
    /// ```
    ///
    pub fn register_handler<F>(&mut self, handler: F)
    where
        F: Fn(E) -> EventBlock + Send + Sync + 'static,
    {
        self.handler = Some(Box::new(handler));
    }

    /// Returns `true` if the `HandlerFunction` registers a callback function.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap_core::{EventBlock, HandlerFunction, KeyboardEvent};
    ///
    /// let mut handler = HandlerFunction::<KeyboardEvent>::new();
    /// assert!(!handler.is_handler_registered());
    ///
    /// handler.register_handler(|_| EventBlock::Unblock);
    /// assert!(handler.is_handler_registered());
    /// ```
    ///
    pub fn is_handler_registered(&self) -> bool {
        self.handler.is_some()
    }

    /// Calls a registered handler and returns value returned by the handler.
    ///
    /// # Panics
    ///
    /// Panics if the handler has not yet been registered.
    ///
    /// # Examples
    /// ```
    /// use hookmap_core::{ButtonAction, ButtonEvent, EventBlock, HandlerFunction, Key, KeyboardEvent};
    ///
    /// let mut handler = HandlerFunction::<KeyboardEvent>::new();
    /// handler.register_handler(|_| EventBlock::Block);
    /// let event_block = handler.emit(ButtonEvent::new(Key::A, ButtonAction::Press));
    /// assert_eq!(event_block, EventBlock::Block);
    /// ```
    ///
    pub fn emit(&self, event: E) -> EventBlock {
        (self.handler.as_ref().unwrap())(event)
    }
}

impl<E> std::fmt::Debug for HandlerFunction<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}<{}>",
            std::any::type_name::<Self>(),
            std::any::type_name::<E>(),
        )
    }
}

impl<E> Default for HandlerFunction<E> {
    fn default() -> Self {
        Self { handler: None }
    }
}

/// A keyboard and mouse Event Handler.
///
/// FFI requires static variables, so instead of creating a new instance, use [`INPUT_HANDLER`].
#[derive(Debug, Default)]
pub struct InputHandler {
    pub keyboard: RwLock<HandlerFunction<KeyboardEvent>>,
    pub mouse_button: RwLock<HandlerFunction<MouseEvent>>,
    pub mouse_wheel: RwLock<HandlerFunction<i32>>,
    pub mouse_cursor: RwLock<HandlerFunction<(i32, i32)>>,
}

impl InputHandler
where
    Self: InstallKeyboardHook + InstallMouseHook + HandleInput,
{
    /// Creates a new `InputHandler`.
    ///
    /// Instead of calling this function, use `hookmap_core::INPUT_HANDLER`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use hookmap_core::InputHandler;
    /// let handler = InputHandler::new();
    /// ```
    ///
    fn new() -> Self {
        Self::default()
    }

    /// Handles keyboard and mouse event and blocks a thread.
    ///
    /// # Panics
    ///
    /// Panics if a mutex lock fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use hookmap_core::INPUT_HANDLER;
    /// INPUT_HANDLER.handle_input();
    /// ```
    pub fn handle_input(&self) {
        let registered_keyboard_handler = self.keyboard.read().unwrap().is_handler_registered();
        let registered_mouse_handler = self.mouse_button.read().unwrap().is_handler_registered()
            || self.mouse_wheel.read().unwrap().is_handler_registered()
            || self.mouse_cursor.read().unwrap().is_handler_registered();

        if registered_keyboard_handler {
            <Self as InstallKeyboardHook>::install();
        }
        if registered_mouse_handler {
            <Self as InstallMouseHook>::install();
        }
        if registered_keyboard_handler || registered_mouse_handler {
            <Self as HandleInput>::handle_input();
        }
    }
}
