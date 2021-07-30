use hookmap_core::{KeyboardEvent, MouseEvent};
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

pub fn get_keyboard_event() -> KeyboardEvent {
    let (tx, rx) = mpsc::channel();
    EVENT_SENDER.lock().unwrap().keyboard.push(tx);
    rx.recv().unwrap()
}

pub fn get_mouse_button_event() -> MouseEvent {
    let (tx, rx) = mpsc::channel();
    EVENT_SENDER.lock().unwrap().mouse_button.push(tx);
    rx.recv().unwrap()
}

pub fn get_mouse_cursor_event() -> (i32, i32) {
    let (tx, rx) = mpsc::channel();
    EVENT_SENDER.lock().unwrap().mouse_cursor.push(tx);
    rx.recv().unwrap()
}

pub fn get_mouse_wheel_event() -> i32 {
    let (tx, rx) = mpsc::channel();
    EVENT_SENDER.lock().unwrap().mouse_wheel.push(tx);
    rx.recv().unwrap()
}
