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

use hookmap_core::{ButtonEvent, ButtonKind, EventBlock};
use once_cell::sync::Lazy;
use std::{
    fmt::Debug,
    mem,
    sync::{
        mpsc::{self, Sender},
        Mutex,
    },
};

pub(super) static INTERRUPTION_EVENT: Lazy<EventSender> = Lazy::new(EventSender::default);

#[derive(Debug)]
pub(super) struct EventSenderVec<I: Debug> {
    block: Vec<Sender<I>>,
    unblock: Vec<Sender<I>>,
}

impl<E: Debug + Copy> EventSenderVec<E> {
    pub(super) fn send_event(&mut self, event: E) {
        if self.block.is_empty() {
            mem::take(&mut self.unblock)
                .iter()
                .for_each(|tx| tx.send(event).unwrap());
        } else {
            self.block.remove(0).send(event).unwrap();
        }
    }

    fn push(&mut self, tx: Sender<E>, event_block: EventBlock) {
        match event_block {
            EventBlock::Block => self.block.push(tx),
            EventBlock::Unblock => self.unblock.push(tx),
        }
    }

    fn get_event_block(&self) -> EventBlock {
        if self.block.is_empty() {
            EventBlock::Unblock
        } else {
            EventBlock::Block
        }
    }
}

impl<I: Debug> Default for EventSenderVec<I> {
    fn default() -> Self {
        Self {
            block: Vec::default(),
            unblock: Vec::default(),
        }
    }
}

#[derive(Debug, Default)]
pub(super) struct EventSender {
    pub(super) keyboard: Mutex<EventSenderVec<ButtonEvent>>,
    pub(super) mouse_button: Mutex<EventSenderVec<ButtonEvent>>,
    pub(super) mouse_cursor: Mutex<EventSenderVec<(i32, i32)>>,
    pub(super) mouse_wheel: Mutex<EventSenderVec<i32>>,
}

impl EventSender {
    pub(super) fn get_event_block(&self, event: ButtonEvent) -> EventBlock {
        match event.target.kind() {
            ButtonKind::Key => self.keyboard.lock().unwrap().get_event_block(),
            ButtonKind::Mouse => self.mouse_button.lock().unwrap().get_event_block(),
        }
    }

    pub(super) fn send_button_event(&self, event: ButtonEvent) {
        match event.target.kind() {
            ButtonKind::Key => self.keyboard.lock().unwrap().send_event(event),
            ButtonKind::Mouse => self.mouse_button.lock().unwrap().send_event(event),
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Interruption(EventBlock);

impl Interruption {
    pub fn block() -> Self {
        Self(EventBlock::Block)
    }

    pub fn unblock() -> Self {
        Self(EventBlock::Unblock)
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
    pub fn keyboard(&self) -> EventReceiver<ButtonEvent> {
        EventReceiver::new(&INTERRUPTION_EVENT.keyboard, self.0)
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
    pub fn mouse_button(&self) -> EventReceiver<ButtonEvent> {
        EventReceiver::new(&INTERRUPTION_EVENT.mouse_button, self.0)
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
    pub fn mouse_cursor(&self) -> EventReceiver<(i32, i32)> {
        EventReceiver::new(&INTERRUPTION_EVENT.mouse_cursor, self.0)
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
    pub fn mouse_wheel(&self) -> EventReceiver<i32> {
        EventReceiver::new(&INTERRUPTION_EVENT.mouse_wheel, self.0)
    }
}

pub struct EventReceiver<'a, E: Debug + Copy>(&'a Mutex<EventSenderVec<E>>, EventBlock);

impl<'a, E: Debug + Copy> EventReceiver<'a, E> {
    fn new(event_sender_vec: &'a Mutex<EventSenderVec<E>>, event_block: EventBlock) -> Self {
        Self(event_sender_vec, event_block)
    }

    pub fn recv(&self) -> E {
        self.iter().next().unwrap()
    }

    pub fn iter(&self) -> impl Iterator<Item = E> + 'a {
        Iter::new(self.0, self.1)
    }
}

struct Iter<'a, E: Debug + Copy>(&'a Mutex<EventSenderVec<E>>, EventBlock);

impl<'a, E: Debug + Copy> Iter<'a, E> {
    fn new(event_sender_vec: &'a Mutex<EventSenderVec<E>>, event_block: EventBlock) -> Self {
        Self(event_sender_vec, event_block)
    }
}

impl<E: Debug + Copy> Iterator for Iter<'_, E> {
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        let (tx, rx) = mpsc::channel();
        self.0.lock().unwrap().push(tx, self.1);
        Some(rx.recv().unwrap())
    }
}
