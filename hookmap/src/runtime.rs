pub mod interceptor;

use crate::hook::{Hook, HookStorage};
use hookmap_core::{common::event::UndispatchedEvent, HookHandler, NativeEventOperation};
use std::thread;

fn compute_native_event_operation(operations: &[NativeEventOperation]) -> NativeEventOperation {
    *operations
        .iter()
        .find(|&&operation| operation == NativeEventOperation::Block)
        .unwrap_or(&NativeEventOperation::Dispatch)
}

#[derive(Debug)]
pub(crate) struct Runtime<T>
where
    T: HookStorage + Send + 'static,
    <T as HookStorage>::ButtonHook: Send,
    <T as HookStorage>::MouseWheelHook: Send,
    <T as HookStorage>::MouseCursorHook: Send,
{
    storage: T,
}

impl<T> Runtime<T>
where
    T: HookStorage + Send,
    <T as HookStorage>::ButtonHook: Send,
    <T as HookStorage>::MouseWheelHook: Send,
    <T as HookStorage>::MouseCursorHook: Send,
{
    pub(crate) fn new(storage: T) -> Self {
        Self { storage }
    }

    fn handle_event<E, U>(
        &self,
        fetch: impl FnOnce(&T, E) -> Vec<U>,
        message: UndispatchedEvent,
        event: E,
    ) where
        E: Copy + Send + 'static,
        U: Hook<E> + Send + 'static,
    {
        let hooks = fetch(&self.storage, event);
        let has_block_operation = hooks
            .iter()
            .map(|hook| hook.native_event_operation())
            .any(|operation| operation == NativeEventOperation::Block);
        let operation = if has_block_operation {
            NativeEventOperation::Block
        } else {
            NativeEventOperation::Dispatch
        };
        message.operate(operation);
        thread::spawn(move || hooks.iter().for_each(|hook| hook.run(event)));
    }

    pub(crate) fn start(&self) {
        let event_receiver = HookHandler::install_hook();

        loop {
            let message = event_receiver.recv();
            match message.event {
                hookmap_core::Event::Button(event) => {
                    self.handle_event(HookStorage::fetch_button_hook, message, event);
                }
                hookmap_core::Event::MouseWheel(event) => {
                    self.handle_event(HookStorage::fetch_mouse_wheel_hook, message, event)
                }
                hookmap_core::Event::MouseCursor(event) => {
                    self.handle_event(HookStorage::fetch_mouse_cursor_hook, message, event)
                }
            }
        }
    }
}
