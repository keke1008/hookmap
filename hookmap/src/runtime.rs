mod fetcher;
mod startup;

pub(crate) mod register;
pub(crate) mod storage;

pub(crate) use register::Register;
pub(crate) use startup::HookInstaller;

pub mod hook;
pub mod interruption;

use hookmap_core::EventBlock;

fn compute_event_block(event_blocks: &[EventBlock]) -> EventBlock {
    *event_blocks
        .iter()
        .find(|&&event_block| event_block == EventBlock::Block)
        .unwrap_or(&EventBlock::Unblock)
}
