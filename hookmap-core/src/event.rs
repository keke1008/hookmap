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
        &NativeEventOperation::Dispatch
    }
}

impl Default for NativeEventOperation {
    fn default() -> Self {
        *<&NativeEventOperation>::default()
    }
}

/// Information about the generated event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Event {
    Button(ButtonEvent),
    MouseWheel(MouseWheelEvent),
    MouseCursor(MouseCursorEvent),
}

#[derive(Debug)]
pub struct NativeEventHandler {
    tx: Option<Sender<NativeEventOperation>>,
}

impl NativeEventHandler {
    fn new(tx: Sender<NativeEventOperation>) -> Self {
        Self { tx: Some(tx) }
    }

    pub fn handle(mut self, operation: NativeEventOperation) {
        self.tx.take().unwrap().send(operation).unwrap();
    }

    pub fn dispatch(self) {
        self.handle(NativeEventOperation::Dispatch);
    }

    pub fn block(self) {
        self.handle(NativeEventOperation::Block);
    }
}

impl Drop for NativeEventHandler {
    fn drop(&mut self) {
        if let Some(tx) = self.tx.take() {
            tx.send(NativeEventOperation::default()).unwrap();
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct EventSender {
    tx: SyncSender<(Event, NativeEventHandler)>,
}

impl EventSender {
    pub(crate) fn new(tx: SyncSender<(Event, NativeEventHandler)>) -> Self {
        Self { tx }
    }

    pub(crate) fn send(&self, event: Event) -> NativeEventOperation {
        let (tx, rx) = mpsc::channel();
        let sent_data = (event, NativeEventHandler::new(tx));

        self.tx.send(sent_data).unwrap();
        rx.recv().unwrap()
    }
}

pub type EventReceiver = Receiver<(Event, NativeEventHandler)>;

pub(crate) fn channel() -> (EventSender, EventReceiver) {
    const BOUND: usize = 1;
    let (tx, rx) = mpsc::sync_channel(BOUND);
    (EventSender::new(tx), rx)
}
