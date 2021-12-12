use crate::{
    common::event::{self, EventConsumer},
    sys,
};
use std::{
    sync::atomic::{AtomicBool, Ordering},
    thread,
};

static HOOK_INSTALLED: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Default)]
pub struct HookHandler;

impl HookHandler {
    pub fn install_hook() -> EventConsumer {
        assert!(!HOOK_INSTALLED.swap(true, Ordering::SeqCst));
        let (event_tx, event_rx) = event::connection();
        thread::spawn(move || sys::install_hook(event_tx));
        event_rx
    }
}
