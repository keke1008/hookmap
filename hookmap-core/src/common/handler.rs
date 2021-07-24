use super::{
    event::{Event, EventBlock},
    keyboard::{InstallKeyboardHook, Key, KeyboardAction},
    mouse::{InstallMouseHook, MouseAction, MouseInput},
};
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static INPUT_HANDLER: Lazy<InputHandler> = Lazy::new(InputHandler::new);

/// Installs a hook in the way of each platform.
/// This needs to implement for `InputHandler`.
pub trait HandleInput {
    /// Installs a hook and blocks a thread.
    fn handle_input();
}

type EventCallback<T, A> = Box<dyn FnMut(Event<T, A>) -> EventBlock + Send>;

/// An optional input event handler.
pub struct HandlerFunction<T, A> {
    handler: Option<EventCallback<T, A>>,
}

impl<T, A> HandlerFunction<T, A> {
    /// Creates a new `HandlerFunction<T, A>` with `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap_core::{keyboard::{Key, KeyboardAction}, EventBlock, HandlerFunction};
    /// let handler = HandlerFunction::<Key, KeyboardAction>::new();
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
    /// use hookmap_core::{keyboard::{Key, KeyboardAction}, EventBlock, HandlerFunction};
    ///
    /// let handler = HandlerFunction::<Key, KeyboardAction>::new();
    /// handler.register_handler(|e| {
    ///     println!("Event target: {:?}", e.target);
    ///     println!("Event action: {:?}", e.action);
    ///     EventBlock::Unblock
    /// });
    /// ```
    ///
    pub fn register_handler<F>(&mut self, handler: F)
    where
        F: FnMut(Event<T, A>) -> EventBlock + Send + 'static,
    {
        self.handler = Some(Box::new(handler));
    }

    /// Returns `true` if the `HandlerFunction` registers a callback function.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap_core::{keyboard::{Key, KeyboardAction}, Event, EventBlock, HandlerFunction};
    ///
    /// let handler = HandlerFunction::<Key, KeyboardAction>::new();
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
    /// use hookmap_core::{keyboard::{Key, KeyboardAction}, Event, EventBlock, HandlerFunction};
    ///
    /// let handler = HandlerFunction::<Key, KeyboardAction>::new();
    /// handler.register_handler(|_| EventBlock::Block);
    /// let event_block = handler.emit(Event::new(Key::A, KeyboardAction::Press));
    /// assert_eq!(event_block, EventBlock::Block);
    /// ```
    ///
    pub fn emit(&mut self, event: Event<T, A>) -> EventBlock {
        (self.handler.as_mut().unwrap())(event)
    }
}

impl<T, A> Default for HandlerFunction<T, A> {
    fn default() -> Self {
        Self { handler: None }
    }
}

impl<T, A> std::fmt::Debug for HandlerFunction<T, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}<{}, {}>",
            std::any::type_name::<Self>(),
            std::any::type_name::<T>(),
            std::any::type_name::<A>()
        )
    }
}

/// A keyboard and mouse Event Handler.
///
/// FFI requires static variables, so instead of creating a new instance, use `hookmap_core::INPUT_HANDLER`.
#[derive(Debug, Default)]
pub struct InputHandler {
    pub keyboard: Mutex<HandlerFunction<Key, KeyboardAction>>,
    pub mouse: Mutex<HandlerFunction<MouseInput, MouseAction>>,
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
        let registered_keyboard_handler = self.keyboard.lock().unwrap().is_handler_registered();
        let registered_mouse_handler = self.mouse.lock().unwrap().is_handler_registered();

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
