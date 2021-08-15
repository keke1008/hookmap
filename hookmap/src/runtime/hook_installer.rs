use super::{interruption::INTERRUPTION_EVENT_SENDER, runtime_handler::RuntimeHandler};
use crate::{handler::SatisfiedHandler, Hook};
use hookmap_core::{ButtonAction, ButtonEvent, EventBlock, EventCallback, INPUT_HANDLER};
use once_cell::sync::OnceCell;
use std::{fmt::Debug, rc::Rc};

static RUNTIME_EVENT_HANDLER: OnceCell<RuntimeHandler> = OnceCell::new();

pub struct EventHandler<'a, E: Copy + Debug + PartialEq + Send + Sync + 'static> {
    handlers: SatisfiedHandler<'a, E>,
    event: E,
}

impl<'a, E: Copy + Debug + PartialEq + Send + Sync + 'static> EventHandler<'a, E> {
    fn new(handlers: SatisfiedHandler<'a, E>, event: E) -> Self {
        Self { handlers, event }
    }

    fn get_event_block(&self) -> EventBlock {
        let is_contains_block = self
            .handlers
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

impl<'a> EventCallback for EventHandler<'a, ButtonEvent> {
    fn get_event_block(&self) -> EventBlock {
        self.get_event_block()
    }

    fn call(&mut self) {
        INTERRUPTION_EVENT_SENDER
            .lock()
            .unwrap()
            .button
            .send_event(self.event);
        self.handlers.call();
    }
}

impl<'a> EventCallback for EventHandler<'a, i32> {
    fn get_event_block(&self) -> EventBlock {
        self.get_event_block()
    }

    fn call(&mut self) {
        INTERRUPTION_EVENT_SENDER
            .lock()
            .unwrap()
            .mouse_wheel
            .send_event(self.event);
        self.handlers.call();
    }
}

impl<'a> EventCallback for EventHandler<'a, (i32, i32)> {
    fn get_event_block(&self) -> EventBlock {
        self.get_event_block()
    }

    fn call(&mut self) {
        INTERRUPTION_EVENT_SENDER
            .lock()
            .unwrap()
            .mouse_cursor
            .send_event(self.event);
        self.handlers.call();
    }
}

pub(crate) struct HookInstaller {
    handler: RuntimeHandler,
}

impl HookInstaller {
    pub(crate) fn install_hook(self) {
        RUNTIME_EVENT_HANDLER.set(self.handler).unwrap();

        INPUT_HANDLER.button.register_handler(move |event| {
            let event_handler = RUNTIME_EVENT_HANDLER.get().unwrap();
            let handlers = match event.action {
                ButtonAction::Press => event_handler.button.on_press.get_satisfied(event),
                ButtonAction::Release => event_handler.button.on_release.get_satisfied(event),
            };
            Box::new(EventHandler::new(handlers, event))
        });
        INPUT_HANDLER.mouse_cursor.register_handler(move |event| {
            let satisfied_handlers = RUNTIME_EVENT_HANDLER
                .get()
                .unwrap()
                .mouse_cursor
                .get_satisfied(event);
            Box::new(EventHandler::new(satisfied_handlers, event))
        });
        INPUT_HANDLER.mouse_wheel.register_handler(move |event| {
            let satisfied_handlers = RUNTIME_EVENT_HANDLER
                .get()
                .unwrap()
                .mouse_wheel
                .get_satisfied(event);
            Box::new(EventHandler::new(satisfied_handlers, event))
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
