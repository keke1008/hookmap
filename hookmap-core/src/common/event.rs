use super::button::{Button, ButtonAction};
use std::hash::Hash;
use std::sync::mpsc::{self, Sender, SyncSender};

/// Indicates whether to pass the generated event to the next program or not .
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NativeEventOperation {
    /// Do not pass the generated event to the next program.
    Block,

    /// Pass the generated event to the next program.
    Dispatch,
}

impl Default for &NativeEventOperation {
    fn default() -> Self {
        if cfg!(feature = "block-input-event") {
            &NativeEventOperation::Block
        } else {
            &NativeEventOperation::Dispatch
        }
    }
}

impl Default for NativeEventOperation {
    fn default() -> Self {
        *<&NativeEventOperation>::default()
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

pub struct EventMessage {
    pub event: Event,
    pub(crate) event_block_sender: Option<Sender<NativeEventOperation>>,
}

impl EventMessage {
    fn new(event: Event, event_block_sender: Sender<NativeEventOperation>) -> Self {
        Self {
            event,
            event_block_sender: Some(event_block_sender),
        }
    }

    /// Sends whether to block this event or not.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use hookmap_core::*;
    /// let mut message = HookHandler::install_hook().recv().unwrap();
    /// message.send_event_block(EventBlock::Block);
    /// ```
    ///
    pub fn send_event_block(&mut self, event_block: NativeEventOperation) {
        self.event_block_sender
            .take()
            .expect("EventBlock has already been sent.")
            .send(event_block)
            .unwrap();
    }

    /// Sends the default value of `EventBlock`.
    /// The feature flag `block-input-event` can set the default value of `EventBlock`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use hookmap_core::*;
    /// let mut message = HookHandler::install_hook().recv().unwrap();
    /// message.send_default_event_block();
    /// ```
    ///
    pub fn send_default_event_block(&mut self) {
        self.send_event_block(NativeEventOperation::default());
    }
}

impl Drop for EventMessage {
    fn drop(&mut self) {
        if self.event_block_sender.is_some() {
            self.send_default_event_block();
        }
    }
}

#[derive(Clone, Debug)]
pub struct EventMessageSender(SyncSender<EventMessage>);

impl EventMessageSender {
    pub(crate) fn new(event_sender: SyncSender<EventMessage>) -> Self {
        Self(event_sender)
    }

    pub(crate) fn send(&self, event: Event) -> NativeEventOperation {
        let (tx, rx) = mpsc::channel::<NativeEventOperation>();
        let event_message = EventMessage::new(event, tx);
        self.0.send(event_message).unwrap();
        rx.recv().unwrap()
    }
}
