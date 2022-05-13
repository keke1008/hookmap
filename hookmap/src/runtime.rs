mod button_state;
mod event_broker;
pub mod interceptor;

use hookmap_core::event::{Event, NativeEventHandler, NativeEventOperation};

use self::button_state::RealButtonState;
use crate::hook::{ButtonState, Hook, HookStorage};

use std::thread;

#[derive(Debug)]
pub(crate) struct Runtime<T, S: ButtonState = RealButtonState>
where
    T: HookStorage + Send + 'static,
    <T as HookStorage>::ButtonHook: Send,
    <T as HookStorage>::MouseWheelHook: Send,
    <T as HookStorage>::MouseCursorHook: Send,
{
    storage: T,
    state: S,
}

impl<T> Runtime<T, RealButtonState>
where
    T: HookStorage + Send,
    <T as HookStorage>::ButtonHook: Send,
    <T as HookStorage>::MouseWheelHook: Send,
    <T as HookStorage>::MouseCursorHook: Send,
{
    pub(crate) fn new(storage: T) -> Self {
        Self::with_state(storage, RealButtonState)
    }
}

impl<T, S: ButtonState> Runtime<T, S>
where
    T: HookStorage + Send + 'static,
    <T as HookStorage>::ButtonHook: Send,
    <T as HookStorage>::MouseWheelHook: Send,
    <T as HookStorage>::MouseCursorHook: Send,
{
    pub(crate) fn with_state(storage: T, state: S) -> Self {
        Self { storage, state }
    }

    fn handle_event<F, E, H>(&self, fetch: F, event: E, native_handler: NativeEventHandler)
    where
        F: FnOnce(&T, E, &S) -> Vec<H>,
        E: Copy + Send + 'static,
        H: Hook<E> + Send + 'static,
    {
        let hooks = fetch(&self.storage, event, &self.state);
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
                    if interceptor::publish_event(event) == NativeEventOperation::Block {
                        native_handler.block();
                        continue;
                    }
                    self.handle_event(HookStorage::fetch_button_hook, event, native_handler);
                }
                Event::Wheel(event) => {
                    self.handle_event(HookStorage::fetch_mouse_wheel_hook, event, native_handler);
                }
                Event::Cursor(event) => {
                    self.handle_event(HookStorage::fetch_mouse_cursor_hook, event, native_handler);
                }
            }
        }
    }
}
