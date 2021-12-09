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
    pub(crate) native_event_operation_sender: Option<Sender<NativeEventOperation>>,
}

impl EventMessage {
    fn new(event: Event, native_event_operation_sender: Sender<NativeEventOperation>) -> Self {
        Self {
            event,
            native_event_operation_sender: Some(native_event_operation_sender),
        }
    }

    /// Sends whether to block this event or not.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use hookmap_core::*;
    /// let mut message = HookHandler::install_hook().recv().unwrap();
    /// message.send_native_event_operation(NativeEventOperation::Block);
    /// ```
    ///
    pub fn send_native_event_operation(&mut self, operation: NativeEventOperation) {
        self.native_event_operation_sender
            .take()
            .expect("The native event operation has already been sent.")
            .send(operation)
            .unwrap();
    }

    /// Sends the default value of `NativeEventOperation`.
    /// The feature flag `block-input-event` can set the default value of `NativeEventOperation`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use hookmap_core::*;
    /// let mut message = HookHandler::install_hook().recv().unwrap();
    /// message.send_default_native_event_operation();
    /// ```
    ///
    pub fn send_default_native_event_operation(&mut self) {
        self.send_native_event_operation(NativeEventOperation::default());
    }
}

impl Drop for EventMessage {
    fn drop(&mut self) {
        if self.native_event_operation_sender.is_some() {
            self.send_default_native_event_operation();
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
