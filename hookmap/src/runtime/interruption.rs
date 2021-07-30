use hookmap_core::{ButtonAction, KeyboardEvent, MouseEvent};
use once_cell::sync::Lazy;
use std::{
    fmt::Debug,
    mem,
    sync::{
        mpsc::{self, Sender},
        Mutex,
    },
    thread,
};

pub(super) static EVENT_SENDER: Lazy<Mutex<EventSender>> = Lazy::new(Mutex::default);

#[derive(Debug)]
pub(super) struct EventSenderVec<I: Debug>(Vec<Sender<I>>);

impl<I: Debug + Copy> EventSenderVec<I> {
    pub(super) fn send_event(&mut self, info: I) {
        let sender = mem::take(&mut self.0);
        sender.iter().for_each(|tx| tx.send(info).unwrap());
    }

    fn push(&mut self, tx: Sender<I>) {
        self.0.push(tx);
    }
}

impl<I: Debug> Default for EventSenderVec<I> {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[derive(Debug, Default)]
pub(super) struct EventSender {
    pub(super) keyboard: EventSenderVec<KeyboardEvent>,
    pub(super) mouse_button: EventSenderVec<MouseEvent>,
    pub(super) mouse_cursor: EventSenderVec<(i32, i32)>,
    pub(super) mouse_wheel: EventSenderVec<i32>,
}

/// Blocks the thread and receive input.
///
/// To avoid blocking the hook thread, call [`Interruption::spawn`] instead of creating a new instance.
///
/// # Examples
///
/// ```
/// use hookmap::*;
/// let hook = Hook::new();
/// hook.bind_key(Key::A).on_press(|_|{
///     Interruption::spawn(|interruption| {
///         let event = interruption.keyboard_event();
///         println!("key:    {:?}", event.target);
///         println!("action: {:?}", event.action);
///     });
/// });
/// ```
///
pub struct Interruption {
    _private: (),
}

impl Interruption {
    /// Exacute `callbach` asynchronously.
    ///
    /// An instance of `Interruption` is given as an argument to `callback`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::*;
    /// Interruption::spawn(|interruption| {
    ///     let event = interruption.keyboard_event();
    ///     println!("key:    {:?}", event.target);
    ///     println!("action: {:?}", event.action);
    /// });
    /// ```
    ///
    pub fn spawn<F>(callback: F)
    where
        F: FnOnce(Interruption) + Send + 'static,
    {
        thread::spawn(move || callback(Interruption { _private: () }));
    }

    /// Waits for the keyboard event.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::*;
    /// Interruption::spawn(|interruption| {
    ///     let event = interruption.keyboard_event();
    ///     println!("key:    {:?}", event.target);
    ///     println!("action: {:?}", event.action);
    /// });
    /// ```
    ///
    pub fn keyboard_event(&self) -> KeyboardEvent {
        let (tx, rx) = mpsc::channel();
        EVENT_SENDER.lock().unwrap().keyboard.push(tx);
        rx.recv().unwrap()
    }

    /// Waits for the keyboard event with the specified action.
    ///
    /// ```
    /// use hookmap::*;
    /// Interruption::spawn(|interruption| {
    ///     let event = interruption.keyboard_event_with_action(ButtonAction::Press);
    ///     assert_eq!(event.action, ButtonAction::Press);
    /// });
    /// ```
    pub fn keyboard_event_with_action(&self, action: ButtonAction) -> KeyboardEvent {
        loop {
            let event = self.keyboard_event();
            if event.action == action {
                return event;
            }
        }
    }

    /// Waits for the mouse button event.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::*;
    /// Interruption::spawn(|interruption| {
    ///     let event = interruption.mouse_button_event();
    ///     println!("button: {:?}", event.target);
    ///     println!("action: {:?}", event.action);
    /// });
    /// ```
    ///
    pub fn mouse_button_event(&self) -> MouseEvent {
        let (tx, rx) = mpsc::channel();
        EVENT_SENDER.lock().unwrap().mouse_button.push(tx);
        rx.recv().unwrap()
    }

    /// Waits for the mouse button event with the specified action.
    ///
    /// ```
    /// use hookmap::*;
    /// Interruption::spawn(|interruption| {
    ///     let event = interruption.mouse_button_event_with_action(ButtonAction::Press);
    ///     assert_eq!(event.action, ButtonAction::Press);
    /// });
    /// ```
    pub fn mouse_button_event_with_action(&self, action: ButtonAction) -> MouseEvent {
        loop {
            let event = self.mouse_button_event();
            if event.action == action {
                return event;
            }
        }
    }

    /// Waits for the mouse cursor movement event.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::*;
    /// Interruption::spawn(|interruption| {
    ///     let position = interruption.mouse_cursor_event();
    ///     println!("x: {}, y: {}", position.0, position.0);
    /// });
    /// ```
    ///
    pub fn mouse_cursor_event(&self) -> (i32, i32) {
        let (tx, rx) = mpsc::channel();
        EVENT_SENDER.lock().unwrap().mouse_cursor.push(tx);
        rx.recv().unwrap()
    }

    /// Waits for the mouse wheel rotation event.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::*;
    /// Interruption::spawn(|interruption| {
    ///     let speed = interruption.mouse_wheel_event();
    ///     println!("speed: {}", speed);
    /// });
    /// ```
    ///
    pub fn mouse_wheel_event(&self) -> i32 {
        let (tx, rx) = mpsc::channel();
        EVENT_SENDER.lock().unwrap().mouse_wheel.push(tx);
        rx.recv().unwrap()
    }
}
