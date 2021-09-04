use super::button::{Button, ButtonAction};
use std::hash::Hash;
use std::sync::mpsc::{self, Sender, SyncSender};

/// Indicates whether to pass the generated event to the next program or not .
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventBlock {
    /// Do not pass the generated event to the next program.
    Block,

    /// Pass the generated event to the next program.
    Unblock,
}

impl Default for &EventBlock {
    fn default() -> Self {
        if cfg!(feature = "block-input-event") {
            &EventBlock::Block
        } else {
            &EventBlock::Unblock
        }
    }
}

impl Default for EventBlock {
    fn default() -> Self {
        *<&EventBlock>::default()
    }
}

/// Information about the generated event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ButtonEvent {
    /// Target of the generated event.
    pub target: Button,

    /// Action of the generated event.
    pub action: ButtonAction,
}

impl ButtonEvent {
    /// Creates a new `ButtonEvent<T, A>`.
    pub fn new(target: Button, action: ButtonAction) -> Self {
        Self { target, action }
    }
}

pub type MouseCursorEvent = (i32, i32);
pub type MouseWheelEvent = i32;

pub enum Event {
    Button(ButtonEvent),
    MouseWheel(MouseWheelEvent),
    MouseCursor(MouseCursorEvent),
}

pub(crate) struct EventMessage {
    pub(crate) event: Event,
    pub(crate) event_block_sender: Sender<EventBlock>,
}

#[derive(Clone, Debug)]
pub struct EventMessageSender(SyncSender<EventMessage>);

impl EventMessageSender {
    pub(crate) fn new(event_sender: SyncSender<EventMessage>) -> Self {
        Self(event_sender)
    }

    pub(crate) fn send(&self, event: Event) -> EventBlock {
        let (tx, rx) = mpsc::channel::<EventBlock>();
        let event_message = EventMessage {
            event,
            event_block_sender: tx,
        };
        self.0.send(event_message).unwrap();
        rx.recv().unwrap()
    }
}
