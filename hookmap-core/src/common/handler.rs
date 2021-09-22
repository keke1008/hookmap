use crate::common::event::{EventMessage, EventMessageSender};
use std::{
    sync::atomic::{AtomicBool, Ordering},
    sync::mpsc::{self, Receiver},
    thread,
};

pub trait HookInstaller {
    /// Handles keyboard and mouse event and blocks a thread.
    fn install(event_message_sender: EventMessageSender);
}

static HOOK_INSTALLED: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Default)]
pub struct HookHandler;

impl HookHandler
where
    Self: HookInstaller,
{
    pub fn install_hook() -> Receiver<EventMessage> {
        assert!(!HOOK_INSTALLED.swap(true, Ordering::SeqCst));
        let (tx, rx) = mpsc::sync_channel(1);
        thread::spawn(move || <Self as HookInstaller>::install(EventMessageSender::new(tx)));
        rx
    }
}
