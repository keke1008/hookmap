use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::{self, Receiver, Sender},
    Arc, Mutex,
};

use hookmap_core::event::{ButtonEvent, NativeEventOperation};

use super::storage::InterruptionFetcher;

#[derive(Debug)]
struct InterruptionSender {
    active: Arc<AtomicBool>,
    native: NativeEventOperation,
    tx: Sender<ButtonEvent>,
}

#[derive(Debug, Default)]
struct SenderStack(Vec<InterruptionSender>);

impl SenderStack {
    fn fetch_block(&mut self, event: ButtonEvent) -> Option<NativeEventOperation> {
        for index in (0..self.0.len()).rev() {
            let tx = &self.0[index];
            if !tx.active.load(Ordering::Acquire) {
                continue;
            }
            match tx.tx.send(event) {
                Ok(_) => return Some(tx.native),
                Err(_) => self.0.remove(index),
            };
        }
        None
    }

    fn fetch_share(&mut self, event: ButtonEvent) -> NativeEventOperation {
        let mut native = NativeEventOperation::Dispatch;

        for index in (0..self.0.len()).rev() {
            let tx = &self.0[index];
            if !tx.active.load(Ordering::Acquire) {
                continue;
            }
            match tx.tx.send(event) {
                Ok(_) => native = native.or(tx.native),
                Err(_) => {
                    self.0.remove(index);
                }
            };
        }

        native
    }

    fn push(&mut self, native: NativeEventOperation) -> (Arc<AtomicBool>, Receiver<ButtonEvent>) {
        let active = Arc::default();
        let (tx, rx) = mpsc::channel();

        self.0.push(InterruptionSender {
            active: Arc::clone(&active),
            native,
            tx,
        });

        (active, rx)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum InterruptionKind {
    RawBlock,
    RawShare,
    HookBlock,
    HookShare,
}

#[derive(Debug, Default)]
pub(crate) struct InterruptionStorage {
    raw_block: SenderStack,
    raw_share: SenderStack,
    hook_block: SenderStack,
    hook_share: SenderStack,
}

impl InterruptionStorage {
    pub(crate) fn register(
        &mut self,
        kind: InterruptionKind,
        native: NativeEventOperation,
    ) -> (Arc<AtomicBool>, Receiver<ButtonEvent>) {
        use InterruptionKind::*;

        let stack = match kind {
            RawBlock => &mut self.raw_block,
            RawShare => &mut self.raw_share,
            HookBlock => &mut self.hook_block,
            HookShare => &mut self.hook_share,
        };

        stack.push(native)
    }
}

impl InterruptionFetcher for Arc<Mutex<InterruptionStorage>> {
    fn fetch_raw_hook(&mut self, event: ButtonEvent) -> (bool, NativeEventOperation) {
        let mut this = self.lock().unwrap();
        match this.raw_block.fetch_block(event) {
            Some(native) => (true, native),
            None => (false, this.raw_share.fetch_share(event)),
        }
    }

    fn fetch_hook(&mut self, event: ButtonEvent) -> NativeEventOperation {
        let mut this = self.lock().unwrap();
        match this.hook_block.fetch_block(event) {
            Some(native) => native,
            None => this.hook_share.fetch_share(event),
        }
    }
}
