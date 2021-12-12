use super::button::{Button, ButtonAction};
use std::sync::mpsc::{self, Receiver, Sender, SyncSender};

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

pub struct UndispatchedEvent {
    pub event: Event,
    pub(crate) native_event_operation_sender: Sender<NativeEventOperation>,
}

impl UndispatchedEvent {
    fn new(event: Event, native_event_operation_sender: Sender<NativeEventOperation>) -> Self {
        Self {
            event,
            native_event_operation_sender,
        }
    }

    /// Dispatches or Blocks events to the OS.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use hookmap_core::*;
    /// let mut event = HookHandler::install_hook().recv().unwrap();
    /// event.operate(NativeEventOperation::Block);
    /// ```
    pub fn operate(self, operation: NativeEventOperation) {
        self.native_event_operation_sender.send(operation).unwrap();
    }

    /// Dispatches this event to the OS.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use hookmap_core::*;
    /// let mut event = HookHandler::install_hook().recv().unwrap();
    /// event.dispatch();
    /// ```
    pub fn dispatch(self) {
        self.operate(NativeEventOperation::Dispatch);
    }

    /// Blocks events from being dispatched to the OS.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use hookmap_core::*;
    /// let mut event = HookHandler::install_hook().recv().unwrap();
    /// event.block();
    /// ```
    pub fn block(self) {
        self.operate(NativeEventOperation::Block);
    }
}

impl Drop for UndispatchedEvent {
    fn drop(&mut self) {
        self.native_event_operation_sender
            .send(NativeEventOperation::default())
            .unwrap();
    }
}

pub(crate) fn connection() -> (EventProvider, EventConsumer) {
    const BOUND: usize = 1;
    let (event_tx, event_rx) = mpsc::sync_channel(BOUND);
    (EventProvider::new(event_tx), EventConsumer::new(event_rx))
}

#[derive(Clone, Debug)]
pub(crate) struct EventProvider {
    event_tx: SyncSender<UndispatchedEvent>,
}

impl EventProvider {
    fn new(event_tx: SyncSender<UndispatchedEvent>) -> Self {
        Self { event_tx }
    }

    pub(crate) fn send(&self, event: Event) -> NativeEventOperation {
        let (operation_tx, operation_rx) = mpsc::channel();
        let undispatched_event = UndispatchedEvent::new(event, operation_tx);
        self.event_tx.send(undispatched_event).unwrap();
        operation_rx.recv().unwrap()
    }
}

#[derive(Debug)]
pub struct EventConsumer {
    event_rx: Receiver<UndispatchedEvent>,
}

impl EventConsumer {
    fn new(event_rx: Receiver<UndispatchedEvent>) -> Self {
        Self { event_rx }
    }

    pub fn recv(&self) -> UndispatchedEvent {
        self.event_rx.recv().unwrap()
    }
}
