use super::event::{
    ButtonEvent, Event, EventBlock, EventMessage, EventMessageSender, MouseCursorEvent,
    MouseWheelEvent,
};
use std::{
    fmt::Debug,
    sync::{mpsc, Mutex},
    thread,
};

pub trait EventCallback: Send + Sync {
    fn call(&mut self);
    fn get_event_block(&self) -> EventBlock;
}

pub type EventCallbackGenerator<E> = Box<dyn Send + FnMut(E) -> Box<dyn EventCallback>>;

/// An optional input event handler.
pub struct EventHandler<E: Send + Copy + 'static> {
    generator: Mutex<Option<EventCallbackGenerator<E>>>,
}

impl<E: Send + Copy + 'static> EventHandler<E> {
    /// Creates a new `EventHandler<E>` with `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap_core::{EventHandler, ButtonEvent};
    /// let handler = EventHandler::<ButtonEvent>::new();
    /// ```
    ///
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a callback function.
    pub fn register_handler<F>(&self, generator: F)
    where
        F: FnMut(E) -> Box<dyn EventCallback> + Send + 'static,
    {
        self.generator.lock().unwrap().insert(Box::new(generator));
    }

    /// Calls the handler in another thread if the handler is registered.
    pub fn emit(&self, event: E) -> EventBlock {
        if let Some(ref mut generator) = *self.generator.lock().unwrap() {
            let mut event_callback = (generator)(event);
            let event_block = event_callback.get_event_block();
            thread::spawn(move || event_callback.call());
            event_block
        } else {
            EventBlock::Unblock
        }
    }
}

impl<E: Send + Copy + 'static> std::fmt::Debug for EventHandler<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}<{}>",
            std::any::type_name::<Self>(),
            std::any::type_name::<E>(),
        )
    }
}

impl<E: Send + Copy + 'static> Default for EventHandler<E> {
    fn default() -> Self {
        Self {
            generator: Default::default(),
        }
    }
}

pub trait HookInstaller {
    /// Handles keyboard and mouse event and blocks a thread.
    fn install(event_message_sender: EventMessageSender);
}

/// A keyboard and mouse Event Handler.
#[derive(Debug, Default)]
pub struct InputHandler {
    pub button: EventHandler<ButtonEvent>,
    pub mouse_wheel: EventHandler<MouseWheelEvent>,
    pub mouse_cursor: EventHandler<MouseCursorEvent>,
}

impl InputHandler
where
    Self: HookInstaller,
{
    /// Creates a new instance fof InputHandler.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap_core::InputHandler;
    /// let input_hanlder = InputHandler::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles keyboard and mouse event and blocks a thread.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use hookmap_core::InputHandler;
    /// let input_handler = InputHandler::new();
    /// input_handler.handle_input();
    /// ```
    pub fn handle_input(self) {
        let (tx, rx) = mpsc::sync_channel::<EventMessage>(0);
        thread::spawn(move || loop {
            let message = rx.recv().unwrap();
            let event_block = match message.event {
                Event::Button(button_event) => self.button.emit(button_event),
                Event::MouseWheel(wheel_event) => self.mouse_wheel.emit(wheel_event),
                Event::MouseCursor(cursor_event) => self.mouse_cursor.emit(cursor_event),
            };
            message.event_block_sender.send(event_block).unwrap();
        });
        <Self as HookInstaller>::install(EventMessageSender::new(tx));
    }
}
