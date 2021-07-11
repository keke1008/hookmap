use std::sync::mpsc::Sender;

/// Indicates whether to pass the generated event to the next program or not .
#[derive(Debug, Clone, Copy)]
pub enum EventBlock {
    /// Do not pass the generated event to the next program.
    Block,

    /// Pass the generated event to the next program.
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

/// Information about the generated event.
/// When dropped, this will send whether to pass the event to the next program or not
/// to the thread where the hook is installed.
#[derive(Debug)]
pub struct EventDetail<T, A> {
    /// Target of the generated event.
    pub target: T,

    /// Action of the generated event.
    pub action: A,

    /// When dropped, this will send an `EventBlock` to the thread where the hook is installed.
    block_input_tx: BlockInputTx,
}

impl<T, A> EventDetail<T, A> {
    /// Creates a new `Event<T, A>`.
    pub fn new(target: T, action: A, block_input_tx: Sender<EventBlock>) -> Self {
        Self {
            target,
            action,
            block_input_tx: BlockInputTx::new(block_input_tx),
        }
    }

    /// Returns a mutable reference to the value that will be sent to the thread where the hook is
    /// installed.
    ///
    /// If you want to block the event, assign `EventBlock::Block`,
    /// otherwise, assign `EventBlock::Unblock`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::common::EventBlock;
    ///
    /// fn block_event(event: &mut Event) {
    ///     event.block_input_mut() = EventBlock::Block;
    /// }
    ///
    /// fn unblock_event(event: &mut Event) {
    ///     event.block_input_mnt() = EventBlock::Unblock;
    /// }
    /// ```
    pub fn block_input_mut(&mut self) -> &mut EventBlock {
        self.block_input_tx.block_input_mut()
    }
}
