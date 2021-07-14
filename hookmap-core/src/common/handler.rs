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

/// An optional thread-safe input event handler.
pub struct HandlerFunction<T, A> {
    handler: Mutex<Option<EventCallback<T, A>>>,
}

impl<T, A> HandlerFunction<T, A> {
    /// Creates a new `HandlerFunction<T, A>`.
    ///
    /// A value of `Mutex` in the field `handler` is initialized with `None`.
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
    /// # Panics
    ///
    /// Panics if a mutex lock fails.
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
    pub fn register_handler<F>(&self, handler: F)
    where
        F: FnMut(Event<T, A>) -> EventBlock + Send + 'static,
    {
        *self.handler.lock().unwrap() = Some(Box::new(handler));
    }

    /// Returns `true` if the `HandlerFunction` registers a callback function.
    ///
    /// # Panics
    ///
    /// Panics if a mutex lock fails.
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
        self.handler.lock().unwrap().is_some()
    }

    /// Calls a registered handler and returns value returned by the handler.
    ///
    /// # Panics
    ///
    /// Panics if the handler has not yet been registered or a mutex fails to lock.
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
    pub fn emit(&self, event: Event<T, A>) -> EventBlock {
        (self.handler.lock().unwrap().as_mut().unwrap())(event)
    }
}

impl<T, A> Default for HandlerFunction<T, A> {
    fn default() -> Self {
        Self {
            handler: Mutex::new(None),
        }
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
#[derive(Debug)]
pub struct InputHandler {
    pub keyboard: HandlerFunction<Key, KeyboardAction>,
    pub mouse: HandlerFunction<MouseInput, MouseAction>,
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
    /// # Examples
    ///
    /// ```no_run
    /// use hookmap_core::INPUT_HANDLER;
    /// INPUT_HANDLER.handle_input();
    /// ```
    pub fn handle_input(&self) {
        let registered_keyboard_handler = self.keyboard.is_handler_registered();
        let registered_mouse_handler = self.mouse.is_handler_registered();

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

impl Default for InputHandler {
    fn default() -> Self {
        Self {
            keyboard: HandlerFunction::new(),
            mouse: HandlerFunction::new(),
        }
    }
}
