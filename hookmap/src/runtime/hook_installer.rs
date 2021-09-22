use super::interruption::{EventSenderVec, INTERRUPTION_EVENT};
use crate::{
    handler::{ButtonFetcher, Handler, MouseFetcher, Storage},
    interface::Hook,
};
use hookmap_core::{common::event::EventMessage, ButtonAction, Event, EventBlock, HookHandler};
use std::{
    fmt::Debug,
    rc::Rc,
    sync::{Arc, Mutex},
};

pub(crate) struct HookInstaller {
    storage: Storage,
}

impl HookInstaller {
    fn get_event_block<T: Debug>(handlers: &[Arc<Handler<T>>]) -> EventBlock {
        let is_contains_block = handlers
            .iter()
            .any(|handler| handler.event_block == EventBlock::Block);
        if is_contains_block {
            EventBlock::Block
        } else {
            EventBlock::Unblock
        }
    }

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
            let handlers = fetcher.fetch();
            event_message.send_event_block(HookInstaller::get_event_block(&handlers));
            handlers
                .iter()
                .for_each(|handler| (handler.callback)(event));
        }
    }

    pub(crate) fn install_hook(self) {
        let on_press_fetcher = ButtonFetcher::new(self.storage.button_on_press);
        let on_release_fetcher = ButtonFetcher::new(self.storage.button_on_release);
        let mouse_cursor_fetcher = MouseFetcher::new(self.storage.mouse_cursor);
        let mouse_wheel_fetcher = MouseFetcher::new(self.storage.mouse_wheel);

        let event_receiver = HookHandler::install_hook();
        while let Ok(mut event_message) = event_receiver.recv() {
            match event_message.event {
                Event::Button(event) => {
                    if INTERRUPTION_EVENT.send_button_event(event) == EventBlock::Block {
                        event_message.send_event_block(EventBlock::Block);
                    } else {
                        let handlers = match event.action {
                            ButtonAction::Press => on_press_fetcher.fetch(&event.target),
                            ButtonAction::Release => on_release_fetcher.fetch(&event.target),
                        };
                        event_message.send_event_block(HookInstaller::get_event_block(&handlers));
                        handlers
                            .iter()
                            .for_each(|handler| (handler.callback)(event));
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
            storage: Rc::try_unwrap(hook.register).unwrap().into_inner(),
        }
    }
}
