use hookmap_core::{ButtonEvent, ButtonKind, EventBlock, MouseCursorEvent, MouseWheelEvent};
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
    pub(super) fn send_event(&mut self, event: E) -> EventBlock {
        if self.block.is_empty() {
            mem::take(&mut self.unblock)
                .iter()
                .for_each(|tx| tx.send(event).unwrap());
            EventBlock::Unblock
        } else {
            self.block.remove(0).send(event).unwrap();
            EventBlock::Block
        }
    }

    fn push(&mut self, tx: Sender<E>, event_block: EventBlock) {
        match event_block {
            EventBlock::Block => self.block.push(tx),
            EventBlock::Unblock => self.unblock.push(tx),
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
    pub(super) mouse_cursor: Mutex<EventSenderVec<MouseCursorEvent>>,
    pub(super) mouse_wheel: Mutex<EventSenderVec<MouseWheelEvent>>,
}

impl EventSender {
    pub(super) fn send_button_event(&self, event: ButtonEvent) -> EventBlock {
        match event.target.kind() {
            ButtonKind::Key => self.keyboard.lock().unwrap().send_event(event),
            ButtonKind::Mouse => self.mouse_button.lock().unwrap().send_event(event),
        }
    }
}

/// Blocks the thread and receive input.
///
/// # Examples
///
/// ```no_run
/// use hookmap::*;
/// let hotkey = Hotkey::new();
/// hotkey.bind(Button::A).on_press(|_| {
///     let event = Interruption::unblock().keyboard().recv();
///     println!("button: {:?}", event.target);
///     println!("action: {:?}", event.action);
/// });
/// ```
///
#[derive(Debug, Default, Clone, Copy)]
pub struct Interruption(EventBlock);

impl Interruption {
    /// Creates a new instance of `Interruption` and block events.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use hookmap::*;
    /// let event = Interruption::block().keyboard().recv();
    /// ```
    ///
    pub fn block() -> Self {
        Self(EventBlock::Block)
    }

    /// Creates a new instance of `Interruption` and do not events.
    /// # Examples
    ///
    /// ```no_run
    /// use hookmap::*;
    /// let event = Interruption::unblock().keyboard().recv();
    /// ```
    ///
    pub fn unblock() -> Self {
        Self(EventBlock::Unblock)
    }

    /// Creates a new [`EventReceiver`] for keyboard events.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use hookmap::*;
    /// let event = Interruption::unblock().keyboard().recv();
    /// println!("key:    {:?}", event.target);
    /// println!("action: {:?}", event.action);
    /// ```
    ///
    pub fn keyboard(&self) -> EventReceiver<ButtonEvent> {
        EventReceiver::new(&INTERRUPTION_EVENT.keyboard, self.0)
    }

    /// Creates a new [`EventReceiver`] for mouse button events.
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use hookmap::*;
    /// let event = Interruption::unblock().mouse_button().recv();
    /// println!("button: {:?}", event.target);
    /// println!("action: {:?}", event.action);
    /// ```
    ///
    pub fn mouse_button(&self) -> EventReceiver<ButtonEvent> {
        EventReceiver::new(&INTERRUPTION_EVENT.mouse_button, self.0)
    }

    /// Creates a new [`EventReceiver`] for mouse cursor events.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use hookmap::*;
    /// let position = Interruption::unblock().mouse_cursor().recv();
    /// println!("x: {}, y: {}", position.0, position.0);
    /// ```
    ///
    pub fn mouse_cursor(&self) -> EventReceiver<MouseCursorEvent> {
        EventReceiver::new(&INTERRUPTION_EVENT.mouse_cursor, self.0)
    }

    /// Creates a new [`EventReceiver`] for mouse wheel events.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use hookmap::*;
    /// let speed = Interruption::unblock().mouse_wheel().recv();
    /// println!("speed: {}", speed);
    /// ```
    ///
    pub fn mouse_wheel(&self) -> EventReceiver<MouseWheelEvent> {
        EventReceiver::new(&INTERRUPTION_EVENT.mouse_wheel, self.0)
    }
}

pub struct EventReceiver<'a, E: Debug + Copy>(&'a Mutex<EventSenderVec<E>>, EventBlock);

impl<'a, E: Debug + Copy> EventReceiver<'a, E> {
    fn new(event_sender_vec: &'a Mutex<EventSenderVec<E>>, event_block: EventBlock) -> Self {
        Self(event_sender_vec, event_block)
    }

    /// Waits for the event.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use hookmap::*;
    /// let event = Interruption::unblock().keyboard().recv();
    /// ```
    ///
    pub fn recv(&self) -> E {
        self.iter().next().unwrap()
    }

    /// Creates a Iterator of the events.
    ///
    /// # Examples
    /// ```no_run
    /// use hookmap::*;
    /// Interruption::unblock()
    ///     .keyboard()
    ///     .iter()
    ///     .filter(|e| e.target == Button::A)
    ///     .for_each(|e| println!("{:?}", e));
    /// ```
    ///
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
