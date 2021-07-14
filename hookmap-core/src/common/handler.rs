use super::{
    event::{Event, EventBlock},
    keyboard::{InstallKeyboardHook, Key, KeyboardAction},
    mouse::{InstallMouseHook, MouseAction, MouseInput},
};
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static INPUT_HANDLER: Lazy<InputHandler> = Lazy::new(InputHandler::new);

pub trait HandleInput {
    fn handle_input();
}

type EventCallback<T, A> = Box<dyn FnMut(Event<T, A>) -> EventBlock + Send>;

pub struct HandlerFunction<T, A> {
    handler: Mutex<Option<EventCallback<T, A>>>,
}

impl<T, A> HandlerFunction<T, A> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_handler<F>(&self, handler: F)
    where
        F: FnMut(Event<T, A>) -> EventBlock + Send + 'static,
    {
        *self.handler.lock().unwrap() = Some(Box::new(handler));
    }

    pub fn is_handler_registered(&self) -> bool {
        self.handler.lock().unwrap().is_some()
    }

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

#[derive(Debug)]
pub struct InputHandler {
    pub keyboard: HandlerFunction<Key, KeyboardAction>,
    pub mouse: HandlerFunction<MouseInput, MouseAction>,
}

impl InputHandler
where
    Self: InstallKeyboardHook + InstallMouseHook + HandleInput,
{
    pub fn new() -> Self {
        Self::default()
    }

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
