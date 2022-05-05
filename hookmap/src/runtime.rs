pub mod interceptor;

use crate::hook::{Hook, HookStorage};
use hookmap_core::event::{Event, NativeEventHandler, NativeEventOperation};
use std::thread;

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
        event: E,
        native_handler: NativeEventHandler,
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
        native_handler.handle(operation);
        thread::spawn(move || hooks.iter().for_each(|hook| hook.run(event)));
    }

    pub(crate) fn start(&self) {
        let event_receiver = hookmap_core::install_hook();

        while let Ok((event, native_handler)) = event_receiver.recv() {
            match event {
                Event::Button(event) => {
                    if interceptor::event_sender::send(event) == NativeEventOperation::Block {
                        native_handler.block();
                        return;
                    }
                    self.handle_event(HookStorage::fetch_button_hook, event, native_handler);
                }
                Event::MouseWheel(event) => {
                    self.handle_event(HookStorage::fetch_mouse_wheel_hook, event, native_handler);
                }
                Event::MouseCursor(event) => {
                    self.handle_event(HookStorage::fetch_mouse_cursor_hook, event, native_handler);
                }
            }
        }
    }
}
