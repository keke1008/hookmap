//! Blocks the thread and receive input.
//!
//! # Examples
//!
//! ```no_run
//! use hookmap::*;
//! let hook = Hook::new();
//! hook.bind(Button::A).on_press(|_| {
//!     let event = interruption::button_event();
//!     println!("button: {:?}", event.target);
//!     println!("action: {:?}", event.action);
//! });
//! ```
//!

use hookmap_core::{ButtonEvent, ButtonKind};
use once_cell::sync::Lazy;
use std::{
    fmt::Debug,
    mem,
    sync::{
        mpsc::{self, Sender},
        Mutex,
    },
};

pub(super) static EVENT_SENDER: Lazy<Mutex<EventSender>> = Lazy::new(Mutex::default);

#[derive(Debug)]
pub(super) struct EventSenderVec<I: Debug>(Vec<Sender<I>>);

impl<E: Debug + Copy> EventSenderVec<E> {
    pub(super) fn send_event(&mut self, event: E) {
        let sender = mem::take(&mut self.0);
        sender.iter().for_each(|tx| tx.send(event).unwrap());
    }

    fn push(&mut self, tx: Sender<E>) {
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
    pub(super) button: EventSenderVec<ButtonEvent>,
    pub(super) mouse_cursor: EventSenderVec<(i32, i32)>,
    pub(super) mouse_wheel: EventSenderVec<i32>,
}

/// Waits for the button event.
///
/// # Examples
///
/// ```no_run
/// use hookmap::*;
/// let event = interruption::button_event();
/// println!("button: {:?}", event.target);
/// println!("action: {:?}", event.action);
/// ```
pub fn button_event() -> ButtonEvent {
    let (tx, rx) = mpsc::channel();
    EVENT_SENDER.lock().unwrap().button.push(tx);
    rx.recv().unwrap()
}

/// Waits for the keyboard event.
///
///
/// # Examples
///
/// ```no_run
/// use hookmap::*;
/// let event = interruption::keyboard_event();
/// println!("key:    {:?}", event.target);
/// println!("action: {:?}", event.action);
/// ```
///
pub fn keyboard_event() -> ButtonEvent {
    loop {
        let event = self::button_event();
        if event.target.kind() == ButtonKind::Key {
            return event;
        }
    }
}

/// Waits for the mouse button event.
///
///
/// # Examples
///
/// ```no_run
/// use hookmap::*;
/// let event = interruption::mouse_button_event();
/// println!("button: {:?}", event.target);
/// println!("action: {:?}", event.action);
/// ```
///
pub fn mouse_button_event() -> ButtonEvent {
    loop {
        let event = self::button_event();
        if event.target.kind() == ButtonKind::Mouse {
            return event;
        }
    }
}

/// Waits for the mouse cursor movement event.
///
/// # Examples
///
/// ```no_run
/// use hookmap::*;
/// let position = interruption::mouse_cursor_event();
/// println!("x: {}, y: {}", position.0, position.0);
/// ```
///
pub fn mouse_cursor_event() -> (i32, i32) {
    let (tx, rx) = mpsc::channel();
    EVENT_SENDER.lock().unwrap().mouse_cursor.push(tx);
    rx.recv().unwrap()
}

/// Waits for the mouse wheel rotation event.
///
/// # Examples
///
/// ```no_run
/// use hookmap::*;
/// let speed = interruption::mouse_wheel_event();
/// println!("speed: {}", speed);
/// ```
///
pub fn mouse_wheel_event() -> i32 {
    let (tx, rx) = mpsc::channel();
    EVENT_SENDER.lock().unwrap().mouse_wheel.push(tx);
    rx.recv().unwrap()
}
