use std::sync::mpsc::Sender;

#[derive(Debug, Clone, Copy)]
pub enum EventBlock {
    Block,
    Unblock,
}

#[derive(Debug)]
struct BlockInputTx {
    block_input: EventBlock,
    tx: Sender<EventBlock>,
}

impl BlockInputTx {
    fn new(tx: Sender<EventBlock>) -> Self {
        Self {
            tx,
            block_input: EventBlock::Unblock,
        }
    }

    fn block_input_mut(&mut self) -> &mut EventBlock {
        &mut self.block_input
    }
}

impl Drop for BlockInputTx {
    fn drop(&mut self) {
        self.tx.send(self.block_input).unwrap();
    }
}

#[derive(Debug)]
pub struct EventDetail<T, A> {
    pub target: T,
    pub action: A,
    block_input_tx: BlockInputTx,
}

impl<T, A> EventDetail<T, A> {
    pub fn new(target: T, action: A, block_input_tx: Sender<EventBlock>) -> Self {
        Self {
            target,
            action,
            block_input_tx: BlockInputTx::new(block_input_tx),
        }
    }

    pub fn block_input_mut(&mut self) -> &mut EventBlock {
        self.block_input_tx.block_input_mut()
    }
}
