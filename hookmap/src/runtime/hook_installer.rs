use super::interruption::INTERRUPTION_EVENT;
use crate::{
    handler::{ButtonFetcher, Handler, MouseFetcher, Storage},
    interface::Hook,
};
use hookmap_core::{
    ButtonAction, ButtonEvent, EventBlock, EventCallback, InputHandler, MouseCursorEvent,
    MouseWheelEvent,
};
use std::{fmt::Debug, rc::Rc, sync::Arc};

pub(crate) enum EventHandler<E: Copy + Debug + PartialEq + Send + Sync + 'static> {
    Interruption,
    Normal(Vec<Arc<Handler<E>>>, E),
}

impl<E: Copy + Debug + PartialEq + Send + Sync + 'static> EventCallback for EventHandler<E> {
    fn get_event_block(&self) -> EventBlock {
        match self {
            Self::Interruption => EventBlock::Block,
            Self::Normal(handlers, _) => {
                let is_contains_block = handlers
                    .iter()
                    .any(|handler| handler.event_block == EventBlock::Block);
                if is_contains_block {
                    EventBlock::Block
                } else {
                    EventBlock::Unblock
                }
            }
        }
    }

    fn call(&mut self) {
        if let Self::Normal(handlers, event) = self {
            handlers
                .iter()
                .for_each(|handler| (handler.callback)(*event));
        }
    }
}

pub(crate) struct HookInstaller {
    storage: Storage,
}

impl HookInstaller {
    pub(crate) fn install_hook(self) {
        let on_press_fetcher = ButtonFetcher::new(self.storage.button_on_press);
        let on_release_fetcher = ButtonFetcher::new(self.storage.button_on_release);
        let input_handler = InputHandler::new();
        input_handler.button.register_handler(move |event| {
            if INTERRUPTION_EVENT.send_button_event(event) == EventBlock::Block {
                Box::new(EventHandler::<ButtonEvent>::Interruption)
            } else {
                let handlers = match event.action {
                    ButtonAction::Press => on_press_fetcher.fetch(&event.target),
                    ButtonAction::Release => on_release_fetcher.fetch(&event.target),
                };
                Box::new(EventHandler::Normal(handlers, event))
            }
        });
        let mouse_cursor_fetcher = MouseFetcher::new(self.storage.mouse_cursor);
        input_handler.mouse_cursor.register_handler(move |event| {
            let interruption_event_block = INTERRUPTION_EVENT
                .mouse_cursor
                .lock()
                .unwrap()
                .send_event(event);
            if interruption_event_block == EventBlock::Block {
                Box::new(EventHandler::<MouseCursorEvent>::Interruption)
            } else {
                Box::new(EventHandler::Normal(mouse_cursor_fetcher.fetch(), event))
            }
        });
        let mouse_wheel_fetcher = MouseFetcher::new(self.storage.mouse_wheel);
        input_handler.mouse_wheel.register_handler(move |event| {
            let event_block = INTERRUPTION_EVENT
                .mouse_wheel
                .lock()
                .unwrap()
                .send_event(event);
            if event_block == EventBlock::Block {
                Box::new(EventHandler::<MouseWheelEvent>::Interruption)
            } else {
                Box::new(EventHandler::Normal(mouse_wheel_fetcher.fetch(), event))
            }
        });
        input_handler.handle_input();
    }
}

impl From<Hook> for HookInstaller {
    fn from(hook: Hook) -> Self {
        Self {
            storage: Rc::try_unwrap(hook.register).unwrap().into_inner(),
        }
    }
}
