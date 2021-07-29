use hookmap_core::EventBlock;
use std::{
    fmt::Debug,
    sync::mpsc::{self, Sender},
};

#[derive(Debug)]
struct BlockInput {
    event_block: EventBlock,
    event_block_tx: Option<Sender<EventBlock>>,
}

impl BlockInput {
    fn new(tx: Option<Sender<EventBlock>>) -> Self {
        let event_block = if cfg!(feature = "block-input-event") {
            EventBlock::Block
        } else {
            EventBlock::Unblock
        };
        Self {
            event_block,
            event_block_tx: tx,
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
/// A struct struct that represents information about a generated event and controls whether the event is
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

    pub(crate) fn new(info: I) -> Self {
        EventInfo {
            block_input: BlockInput::new(None),
            info,
        }
    }

    pub(crate) fn send_with(mut self, callback: impl FnOnce(EventInfo<I>)) -> EventBlock {
        let (tx, rx) = mpsc::channel();
        self.block_input.event_block_tx = Some(tx);
        (callback)(self);
        rx.recv().unwrap()
    }
}
