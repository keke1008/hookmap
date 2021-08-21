use super::{interruption::INTERRUPTION_EVENT, runtime_handler::RuntimeHandler};
use crate::{handler::SatisfiedHandler, Hook};
use hookmap_core::{
    ButtonAction, ButtonEvent, EventBlock, EventCallback, MouseCursorEvent, MouseWheelEvent,
    INPUT_HANDLER,
};
use once_cell::sync::OnceCell;
use std::{fmt::Debug, rc::Rc};

static RUNTIME_EVENT_HANDLER: OnceCell<RuntimeHandler> = OnceCell::new();

pub(crate) enum EventHandler<'a, E: Copy + Debug + PartialEq + Send + Sync + 'static> {
    Interruption,
    Normal(SatisfiedHandler<'a, E>),
}

impl<'a, E: Copy + Debug + PartialEq + Send + Sync + 'static> EventCallback
    for EventHandler<'a, E>
{
    fn get_event_block(&self) -> EventBlock {
        match self {
            Self::Interruption => EventBlock::Block,
            Self::Normal(handlers) => {
                let is_contains_block = handlers
                    .get_event_blocks()
                    .iter()
                    .any(|event_block| event_block == &EventBlock::Block);
                if is_contains_block {
                    EventBlock::Block
                } else {
                    EventBlock::Unblock
                }
            }
        }
    }

    fn call(&mut self) {
        if let Self::Normal(handlers) = self {
            handlers.call()
        }
    }
}

pub(crate) struct HookInstaller {
    handler: RuntimeHandler,
}

impl HookInstaller {
    pub(crate) fn install_hook(self) {
        RUNTIME_EVENT_HANDLER.set(self.handler).unwrap();

        INPUT_HANDLER.button.register_handler(move |event| {
            if INTERRUPTION_EVENT.send_button_event(event) == EventBlock::Block {
                Box::new(EventHandler::<ButtonEvent>::Interruption)
            } else {
                let event_handler = RUNTIME_EVENT_HANDLER.get().unwrap();
                let handlers = match event.action {
                    ButtonAction::Press => event_handler.button.on_press.get_satisfied(event),
                    ButtonAction::Release => event_handler.button.on_release.get_satisfied(event),
                };
                Box::new(EventHandler::Normal(handlers))
            }
        });
        INPUT_HANDLER.mouse_cursor.register_handler(move |event| {
            let interruption_event_block = INTERRUPTION_EVENT
                .mouse_cursor
                .lock()
                .unwrap()
                .send_event(event);
            if interruption_event_block == EventBlock::Block {
                Box::new(EventHandler::<MouseCursorEvent>::Interruption)
            } else {
                let satisfied_handlers = RUNTIME_EVENT_HANDLER
                    .get()
                    .unwrap()
                    .mouse_cursor
                    .get_satisfied(event);
                Box::new(EventHandler::Normal(satisfied_handlers))
            }
        });
        INPUT_HANDLER.mouse_wheel.register_handler(move |event| {
            let event_block = INTERRUPTION_EVENT
                .mouse_wheel
                .lock()
                .unwrap()
                .send_event(event);
            if event_block == EventBlock::Block {
                Box::new(EventHandler::<MouseWheelEvent>::Interruption)
            } else {
                let satisfied_handlers = RUNTIME_EVENT_HANDLER
                    .get()
                    .unwrap()
                    .mouse_wheel
                    .get_satisfied(event);
                Box::new(EventHandler::Normal(satisfied_handlers))
            }
        });
        INPUT_HANDLER.handle_input();
    }
}

impl From<Hook> for HookInstaller {
    fn from(hook: Hook) -> Self {
        let handler = Rc::try_unwrap(hook.handler).unwrap();
        let handler = RuntimeHandler::from(handler);
        Self { handler }
    }
}
