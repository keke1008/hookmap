mod fetcher;
mod startup;

pub(crate) mod register;
pub(crate) mod storage;

pub(crate) use register::Register;
pub(crate) use startup::HookInstaller;

pub mod interceptor;

use hookmap_core::NativeEventOperation;

fn compute_event_block(event_blocks: &[NativeEventOperation]) -> NativeEventOperation {
    *event_blocks
        .iter()
        .find(|&&event_block| event_block == NativeEventOperation::Block)
        .unwrap_or(&NativeEventOperation::Dispatch)
}
