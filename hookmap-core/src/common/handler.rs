use crate::common::event::{self, EventConsumer, EventProvider};
use std::{
    sync::atomic::{AtomicBool, Ordering},
    thread,
};

pub trait HookInstaller {
    /// Handles keyboard and mouse event and blocks a thread.
    fn install(event_provider: EventProvider);
}

static HOOK_INSTALLED: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Default)]
pub struct HookHandler;

impl HookHandler
where
    Self: HookInstaller,
{
    pub fn install_hook() -> EventConsumer {
        assert!(!HOOK_INSTALLED.swap(true, Ordering::SeqCst));
        let (event_tx, event_rx) = event::connection();
        thread::spawn(move || <Self as HookInstaller>::install(event_tx));
        event_rx
    }
}
