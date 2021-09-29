use super::fetcher::{ButtonFetcher, FetchResult, MouseFetcher};
use super::interruption::{EventSenderVec, INTERRUPTION_EVENT};
use super::storage::Storage;
use crate::interface::Hotkey;
use hookmap_core::ButtonAction;
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
            actions.iter().for_each(|action| action.call(event));
        }
    }

    pub(crate) fn install_hook(self) {
        let on_press_fetcher = ButtonFetcher::new(self.storage.on_press);
        let on_release_fetcher = ButtonFetcher::new(self.storage.on_release);
        let mouse_cursor_fetcher = MouseFetcher::new(self.storage.mouse_cursor);
        let mouse_wheel_fetcher = MouseFetcher::new(self.storage.mouse_wheel);

        let event_receiver = HookHandler::install_hook();
        while let Ok(mut event_message) = event_receiver.recv() {
            match event_message.event {
                Event::Button(event) => {
                    if INTERRUPTION_EVENT.send_button_event(event) == EventBlock::Block {
                        event_message.send_event_block(EventBlock::Block);
                    } else {
                        let actions = match event.action {
                            ButtonAction::Press => {
                                let result = on_press_fetcher.fetch(&event);
                                event_message.send_event_block(result.event_block);
                                result.actions
                            }
                            ButtonAction::Release => {
                                // If the release event is not blocked, the button will remain pressed.
                                event_message.send_event_block(EventBlock::Unblock);
                                on_release_fetcher.fetch(&event).actions
                            }
                        };
                        actions.iter().for_each(|action| action.call(event));
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
