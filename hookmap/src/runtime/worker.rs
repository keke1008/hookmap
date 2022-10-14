use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

use hookmap_core::event::{ButtonEvent, CursorEvent, WheelEvent};

use crate::layer::LayerState;

use super::hook::LayerEvent;
use super::hook::{HookAction, IntoInheritedEvent};

#[derive(Debug)]
pub(super) struct Action<E: IntoInheritedEvent + Copy, A = E> {
    event: E,
    actions: Vec<Arc<HookAction<A>>>,
}

impl<E: IntoInheritedEvent + Copy, A> Action<E, A> {
    pub(super) fn new(event: E, actions: Vec<Arc<HookAction<A>>>) -> Self {
        Self { event, actions }
    }
}

impl<E: IntoInheritedEvent + Copy> Action<E> {
    fn run(&self, state: &LayerState, tx: &Sender<LayerEvent>) {
        for action in &self.actions {
            action.run(self.event, state, tx)
        }
    }
}

impl Action<Option<ButtonEvent>, ButtonEvent> {
    fn run(&self, state: &LayerState, tx: &Sender<LayerEvent>) {
        for action in &self.actions {
            action.run_optional(self.event, state, tx);
        }
    }
}

#[derive(Debug)]
pub(super) enum Message {
    Button(Action<ButtonEvent>),
    Optional(Action<Option<ButtonEvent>, ButtonEvent>),
    Cursor(Action<CursorEvent>),
    Wheel(Action<WheelEvent>),
}

#[derive(Debug)]
pub(super) struct Worker {
    handle: JoinHandle<()>,
}

impl Worker {
    pub(super) fn new(
        state: Arc<Mutex<LayerState>>,
        layer_tx: Sender<LayerEvent>,
    ) -> (Sender<Message>, Self) {
        let (tx, rx) = mpsc::channel();
        let handle = thread::spawn(move || {
            for msg in rx.iter() {
                let state = state.lock().unwrap();
                match msg {
                    Message::Button(action) => action.run(&state, &layer_tx),
                    Message::Optional(action) => action.run(&state, &layer_tx),
                    Message::Cursor(action) => action.run(&state, &layer_tx),
                    Message::Wheel(action) => action.run(&state, &layer_tx),
                }
            }
        });

        (tx, Worker { handle })
    }

    pub(super) fn join(self) {
        self.handle.join().unwrap();
    }
}
