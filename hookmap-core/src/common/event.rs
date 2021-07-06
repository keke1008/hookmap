use std::sync::mpsc::Sender;

#[derive(Debug, Clone, Copy)]
pub enum BlockInput {
    Block,
    Unblock,
}

#[derive(Debug)]
struct BlockInputTx {
    block_input: BlockInput,
    tx: Sender<BlockInput>,
}

impl BlockInputTx {
    fn new(tx: Sender<BlockInput>) -> Self {
        Self {
            tx,
            block_input: BlockInput::Unblock,
        }
    }

    fn block_input_mut(&mut self) -> &mut BlockInput {
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
    pub fn new(target: T, action: A, block_input_tx: Sender<BlockInput>) -> Self {
        Self {
            target,
            action,
            block_input_tx: BlockInputTx::new(block_input_tx),
        }
    }

    pub fn block_input_mut(&mut self) -> &mut BlockInput {
        self.block_input_tx.block_input_mut()
    }
}
