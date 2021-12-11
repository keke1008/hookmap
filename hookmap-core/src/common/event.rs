use super::button::{Button, ButtonAction};
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

#[derive(Clone, Debug)]
pub struct EventMessageSender(SyncSender<UndispatchedEvent>);

impl EventMessageSender {
    pub(crate) fn new(event_sender: SyncSender<UndispatchedEvent>) -> Self {
        Self(event_sender)
    }

    pub(crate) fn send(&self, event: Event) -> NativeEventOperation {
        let (tx, rx) = mpsc::channel::<NativeEventOperation>();
        let undispatched_event = UndispatchedEvent::new(event, tx);
        self.0.send(undispatched_event).unwrap();
        rx.recv().unwrap()
    }
}
