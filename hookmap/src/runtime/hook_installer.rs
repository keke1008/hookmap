use super::fetcher::{ButtonFetcher, FetchResult, MouseFetcher};
use super::interruption::{EventSenderVec, INTERRUPTION_EVENT};
use super::storage::Storage;
use crate::interface::Hook;
use hookmap_core::{common::event::EventMessage, Event, EventBlock, HookHandler};
use std::{fmt::Debug, rc::Rc, sync::Mutex};

pub(crate) struct HookInstaller {
    storage: Storage,
}

impl HookInstaller {
    fn handle_mouse_event<T: Debug + Copy>(
        interruption: &Mutex<EventSenderVec<T>>,
        fetcher: &MouseFetcher<T>,
        event_message: &mut EventMessage,
        event: T,
    ) {
        let event_block = interruption.lock().unwrap().send_event(event);
        if event_block == EventBlock::Block {
            event_message.send_event_block(EventBlock::Block)
        } else {
            let FetchResult {
                actions,
                event_block,
            } = fetcher.fetch();
            event_message.send_event_block(event_block);
            actions.iter().for_each(|action| (action.0)(event));
        }
    }

    pub(crate) fn install_hook(self) {
        let button_fetcher = ButtonFetcher::new(self.storage.button);
        let mouse_cursor_fetcher = MouseFetcher::new(self.storage.mouse_cursor);
        let mouse_wheel_fetcher = MouseFetcher::new(self.storage.mouse_wheel);

        let event_receiver = HookHandler::install_hook();
        while let Ok(mut event_message) = event_receiver.recv() {
            match event_message.event {
                Event::Button(event) => {
                    if INTERRUPTION_EVENT.send_button_event(event) == EventBlock::Block {
                        event_message.send_event_block(EventBlock::Block);
                    } else {
                        let FetchResult {
                            actions,
                            event_block,
                        } = button_fetcher.fetch(&event);
                        event_message.send_event_block(event_block);
                        actions.iter().for_each(|action| action.0(event));
                    }
                }
                Event::MouseCursor(event) => {
                    HookInstaller::handle_mouse_event(
                        &INTERRUPTION_EVENT.mouse_cursor,
                        &mouse_cursor_fetcher,
                        &mut event_message,
                        event,
                    );
                }
                Event::MouseWheel(event) => {
                    HookInstaller::handle_mouse_event(
                        &INTERRUPTION_EVENT.mouse_wheel,
                        &mouse_wheel_fetcher,
                        &mut event_message,
                        event,
                    );
                }
            }
        }
    }
}

impl From<Hook> for HookInstaller {
    fn from(hook: Hook) -> Self {
        Self {
            storage: Rc::try_unwrap(hook.register)
                .unwrap()
                .into_inner()
                .into_inner(),
        }
    }
}
