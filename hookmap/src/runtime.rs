mod fetcher;
mod hook_installer;

pub(crate) mod register;
pub(crate) mod storage;

pub(crate) use hook_installer::HookInstaller;
pub(crate) use register::Register;

pub mod hook;
pub mod interruption;

use hookmap_core::EventBlock;

fn compute_event_block(event_blocks: &[EventBlock]) -> EventBlock {
    *event_blocks
        .iter()
        .find(|&&event_block| event_block == EventBlock::Block)
        .unwrap_or(&EventBlock::Unblock)
}
