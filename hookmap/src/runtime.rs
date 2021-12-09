mod fetcher;
mod startup;

pub(crate) mod register;
pub(crate) mod storage;

pub(crate) use register::Register;
pub(crate) use startup::HookInstaller;

pub mod interceptor;

use hookmap_core::NativeEventOperation;

fn compute_native_event_operation(operations: &[NativeEventOperation]) -> NativeEventOperation {
    *operations
        .iter()
        .find(|&&operation| operation == NativeEventOperation::Block)
        .unwrap_or(&NativeEventOperation::Dispatch)
}
