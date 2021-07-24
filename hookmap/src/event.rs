use hookmap_core::EventBlock;
use std::{
    fmt::Debug,
    sync::mpsc::{self, Receiver, Sender},
};

/// An enum that represents the state of a button.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Button {
    Press,
    Release,
}

#[derive(Debug)]
struct BlockInput {
    event_block: EventBlock,
    event_block_tx: Option<Sender<EventBlock>>,
}

impl BlockInput {
    fn new(tx: Sender<EventBlock>) -> Self {
        Self {
            event_block: EventBlock::Unblock,
            event_block_tx: Some(tx),
        }
    }

    fn send_event_block(&mut self) {
        if let Some(tx) = self.event_block_tx.take() {
            tx.send(self.event_block).unwrap();
        }
    }
}

impl Drop for BlockInput {
    fn drop(&mut self) {
        self.send_event_block();
    }
}

#[derive(Debug)]
/// A struct struct that represents information about a generated and controls whether the event is
/// blocked or not.
pub struct EventInfo<I: Debug> {
    pub info: I,
    block_input: BlockInput,
}

impl<I: Debug> EventInfo<I> {
    /// Blocks a generated event.
    pub fn block_event(&mut self) {
        self.block_input.event_block = EventBlock::Block;
    }

    /// Do not block a generated event.
    pub fn unblock_event(&mut self) {
        self.block_input.event_block = EventBlock::Unblock;
    }

    pub(crate) fn channel(info: I) -> (Self, Receiver<EventBlock>) {
        let (tx, rx) = mpsc::channel();
        let event_detail = EventInfo {
            block_input: BlockInput::new(tx),
            info,
        };
        (event_detail, rx)
    }
}