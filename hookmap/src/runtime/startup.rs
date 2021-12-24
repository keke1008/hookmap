use super::{
    fetcher::{FetchResult, Fetchers, MouseFetcher},
    interceptor::event_sender,
    storage::Storage,
};
use crate::interface::Hotkey;
use hookmap_core::ButtonAction;
use hookmap_core::{common::event::UndispatchedEvent, Event, HookHandler, NativeEventOperation};
use std::{fmt::Debug, rc::Rc, thread};

pub(crate) struct HookInstaller {
    storage: Storage,
}

impl HookInstaller {
    fn handle_mouse_event<T: 'static + Debug + Copy + Send>(
        fetcher: &MouseFetcher<T>,
        event_message: UndispatchedEvent,
        event: T,
    ) {
        let FetchResult {
            actions,
            native_event_operation,
        } = fetcher.fetch();
        event_message.operate(native_event_operation);
        thread::spawn(move || actions.iter().for_each(|action| action.call(event)));
    }

    pub(crate) fn install_hook(self) {
        let Fetchers {
            on_press_fetcher,
            on_release_fetcher,
            mouse_cursor_fetcher,
            mouse_wheel_fetcher,
        } = self.storage.into();

        let event_receiver = HookHandler::install_hook();
        loop {
            let event_message = event_receiver.recv();
            match event_message.event {
                Event::Button(event) => {
                    if event_sender::send(event) == NativeEventOperation::Block {
                        event_message.block();
                    } else {
                        let actions = match event.action {
                            ButtonAction::Press => {
                                let result = on_press_fetcher.fetch(&event);
                                event_message.operate(result.native_event_operation);
                                result.actions
                            }
                            ButtonAction::Release => {
                                // If the release event is not blocked, the button will remain pressed.
                                event_message.dispatch();
                                on_release_fetcher.fetch(&event).actions
                            }
                        };
                        thread::spawn(move || actions.iter().for_each(|action| action.call(event)));
                    }
                }
                Event::MouseCursor(event) => {
                    HookInstaller::handle_mouse_event(&mouse_cursor_fetcher, event_message, event);
                }
                Event::MouseWheel(event) => {
                    HookInstaller::handle_mouse_event(&mouse_wheel_fetcher, event_message, event);
                }
            }
        }
    }
}

impl From<Hotkey> for HookInstaller {
    fn from(hook: Hotkey) -> Self {
        Self {
            storage: Rc::try_unwrap(hook.register)
                .unwrap()
                .into_inner()
                .into_inner(),
        }
    }
}
