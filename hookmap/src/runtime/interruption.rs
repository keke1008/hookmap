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

pub(super) static INTERRUPTION_EVENT: Lazy<Mutex<EventSender>> = Lazy::new(Mutex::default);

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
    pub(super) keyboard: EventSenderVec<ButtonEvent>,
    pub(super) mouse_button: EventSenderVec<ButtonEvent>,
    pub(super) mouse_cursor: EventSenderVec<(i32, i32)>,
    pub(super) mouse_wheel: EventSenderVec<i32>,
}

impl EventSender {
    pub(super) fn get_event_block(&self, event: ButtonEvent) -> EventBlock {
        match event.target.kind() {
            ButtonKind::Key => self.keyboard.get_event_block(),
            ButtonKind::Mouse => self.mouse_button.get_event_block(),
        }
    }

    pub(super) fn send_button_event(&mut self, event: ButtonEvent) {
        match event.target.kind() {
            ButtonKind::Key => self.keyboard.send_event(event),
            ButtonKind::Mouse => self.mouse_button.send_event(event),
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
    pub fn keyboard_event(&self) -> ButtonEvent {
        let (tx, rx) = mpsc::channel();
        INTERRUPTION_EVENT.lock().unwrap().keyboard.push(tx, self.0);
        rx.recv().unwrap()
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
    pub fn mouse_button_event(&self) -> ButtonEvent {
        let (tx, rx) = mpsc::channel();
        INTERRUPTION_EVENT
            .lock()
            .unwrap()
            .mouse_button
            .push(tx, self.0);
        rx.recv().unwrap()
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
    pub fn mouse_cursor_event(&self) -> (i32, i32) {
        let (tx, rx) = mpsc::channel();
        INTERRUPTION_EVENT
            .lock()
            .unwrap()
            .mouse_cursor
            .push(tx, self.0);
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
    pub fn mouse_wheel_event(&self) -> i32 {
        let (tx, rx) = mpsc::channel();
        INTERRUPTION_EVENT
            .lock()
            .unwrap()
            .mouse_wheel
            .push(tx, self.0);
        rx.recv().unwrap()
    }
}
