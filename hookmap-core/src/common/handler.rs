use super::event::ButtonEvent;
use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
    thread,
};

type EventCallback<E> = Arc<dyn Fn(E) + Send + Sync>;

/// An optional input event handler.
pub struct HandlerFunction<E> {
    handler: Mutex<Option<EventCallback<E>>>,
}

impl<E> HandlerFunction<E> {
    /// Creates a new `HandlerFunction<E>` with `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap_core::{HandlerFunction, ButtonEvent};
    /// let handler = HandlerFunction::<ButtonEvent>::new();
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
    /// use hookmap_core::{EventBlock, HandlerFunction, ButtonEvent};
    ///
    /// let mut handler = HandlerFunction::<ButtonEvent>::new();
    /// handler.register_handler(|e| {
    ///     println!("Event target: {:?}", e.target);
    ///     println!("Event action: {:?}", e.action);
    /// });
    /// ```
    ///
    pub fn register_handler<F>(&self, handler: F)
    where
        F: Fn(E) + Send + Sync + 'static,
    {
        self.handler.lock().unwrap().insert(Arc::new(handler));
    }

    /// Returns `true` if the `HandlerFunction` registers a callback function.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap_core::{EventBlock, HandlerFunction, ButtonEvent};
    ///
    /// let mut handler = HandlerFunction::<ButtonEvent>::new();
    /// assert!(!handler.is_handler_registered());
    ///
    /// handler.register_handler(|_| {});
    /// ```
    ///
    pub fn is_handler_registered(&self) -> bool {
        self.handler.lock().unwrap().is_some()
    }

    /// Calls the handler in another thread if the handler is registered.
    ///
    /// # Examples
    /// ```
    /// use hookmap_core::{ButtonAction, ButtonEvent, EventBlock, HandlerFunction, Button};
    ///
    /// let mut handler = HandlerFunction::<ButtonEvent>::new();
    /// handler.register_handler(|_| {});
    /// handler.emit(ButtonEvent::new(Button::A, ButtonAction::Press));
    /// ```
    ///
    pub fn emit(&self, event: E)
    where
        E: Send + 'static,
    {
        if let Some(handler) = self.handler.lock().unwrap().as_ref() {
            let handler = Arc::clone(handler);
            thread::spawn(move || (handler)(event));
        }
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
        Self {
            handler: Default::default(),
        }
    }
}

pub trait HookInstaller {
    /// Installs hooks in the way of each platform.
    fn install();

    /// Installs hooks and blocks a thread.
    fn handle_input();
}

/// A keyboard and mouse Event Handler.
///
/// FFI requires static variables, so instead of creating a new instance, use [`INPUT_HANDLER`].
#[derive(Debug, Default)]
pub struct InputHandler {
    pub button: HandlerFunction<ButtonEvent>,
    pub mouse_wheel: HandlerFunction<i32>,
    pub mouse_cursor: HandlerFunction<(i32, i32)>,
}

impl InputHandler
where
    Self: HookInstaller,
{
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
        <Self as HookInstaller>::install();
        <Self as HookInstaller>::handle_input();
    }
}
