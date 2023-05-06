//! Handling hooks and hooked events.
//!
//! When an event is generated, [`EventReceiver::recv`] can be called to receive the event.
//! The received event is blocked (other programs are not yet notified).
//!
//! Calling [`NativeEventHandler::block`] will continue blocking and no notification will be made.
//! Alternatively, calling [`NativeEventHandler::dispatch`] will notify other programs of the event.
//! If neither is called, the event is notified.
//!
//! # Examples
//!
//! ```no_run
//! let rx = hookmap_core::install_hook().unwrap();
//! while let Ok((event, native_handler)) = rx.recv() {
//!     // **DANGEROUS!** block all events!
//!     native_handler.block();
//! }
//! ```

use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    sync::mpsc::{self, Receiver, RecvError, Sender},
};

use crate::{event::Event, sys};

/// Indicates whether to pass the generated event to the next program or not.
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

/// Decide whether to notify other programs of generated events.
#[derive(Debug)]
pub struct NativeEventHandler(Option<Sender<NativeEventOperation>>);

impl NativeEventHandler {
    fn new(tx: Sender<NativeEventOperation>) -> Self {
        Self(Some(tx))
    }

    /// Decides whether or not to notify by argument.
    pub fn handle(mut self, operation: NativeEventOperation) {
        self.0.take().unwrap().send(operation).unwrap();
    }

    // Notifies an event.
    pub fn dispatch(self) {
        self.handle(NativeEventOperation::Dispatch);
    }

    // Does not notify an event.
    pub fn block(self) {
        self.handle(NativeEventOperation::Block);
    }
}

pub(crate) struct NativeEventOperationReceiver(Receiver<NativeEventOperation>);

impl NativeEventOperationReceiver {
    pub(crate) fn recv(self) -> NativeEventOperation {
        match self.0.recv() {
            Ok(NativeEventOperation::Block) => NativeEventOperation::Block,
            _ => NativeEventOperation::Dispatch,
        }
    }
}

#[derive(Debug)]
pub(crate) struct EventSender(Sender<(Event, NativeEventHandler)>);

impl EventSender {
    pub(crate) fn send(&self, event: Event) -> NativeEventOperationReceiver {
        let (tx, rx) = mpsc::channel();
        self.0.send((event, NativeEventHandler::new(tx))).unwrap();
        NativeEventOperationReceiver(rx)
    }
}

/// Receives the event that occurred and the handler for the event.
///
/// When this drops, the hook is uninstalled.
#[derive(Debug)]
pub struct EventReceiver(Receiver<(Event, NativeEventHandler)>);

impl EventReceiver {
    /// Receives an event and handler.
    /// If hook is not installed, `Err` is returned.
    pub fn recv(&self) -> Result<(Event, NativeEventHandler), RecvError> {
        self.0.recv()
    }

    pub fn inner(&self) -> &Receiver<(Event, NativeEventHandler)> {
        &self.0
    }
}

/// An error returned from the [install_hook] function.
///
/// The [install_hook] can only fail if the hook is already installed.
#[derive(Debug, Clone, Copy)]
pub struct InstallHookError;

impl Display for InstallHookError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        "hook is already installed".fmt(f)
    }
}

impl Error for InstallHookError {}

/// An error returned from the [uninstall_hook] function.
///
/// The [uninstall_hook] can only fail if the hook is not installed.
#[derive(Debug, Clone, Copy)]
pub struct UninstallHookError;

impl Display for UninstallHookError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        "hook is not installed".fmt(f)
    }
}

impl Error for UninstallHookError {}

/// Installs a hook and returns a receiver to receive the generated event.
///
/// # Example
///
/// ```no_run
/// let rx = hookmap_core::install_hook().unwrap();
/// ```
///
pub fn install_hook() -> Result<EventReceiver, InstallHookError> {
    let (tx, rx) = mpsc::channel();
    let (tx, rx) = (EventSender(tx), EventReceiver(rx));
    sys::install(tx)?;
    Ok(rx)
}

/// Uninstalls a hook.
/// After this call, [`install_hook`] can be called again.
///
/// # Example
///
/// ```no_run
/// let rx = hookmap_core::install_hook().unwrap();
/// assert!(hookmap_core::uninstall_hook().is_ok());
///
/// assert!(rx.recv().is_err());
///
/// assert!(hookmap_core::install_hook().is_ok());
/// ```
///
pub fn uninstall_hook() -> Result<(), UninstallHookError> {
    sys::uninstall()
}

impl Drop for EventReceiver {
    fn drop(&mut self) {
        let _ = uninstall_hook();
    }
}
