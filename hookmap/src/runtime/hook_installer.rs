use super::{
    alone_modifier::AloneModifierMap, interruption::EVENT_SENDER, runtime_handler::RuntimeHandler,
};
use crate::{handler::SatisfiedHandler, Hook};
use hookmap_core::{ButtonAction, ButtonEvent, EventBlock, EventCallback, INPUT_HANDLER};
use once_cell::sync::{Lazy, OnceCell};
use std::{fmt::Debug, rc::Rc, sync::Mutex};

static RUNTIME_EVENT_HANDLER: OnceCell<RuntimeHandler> = OnceCell::new();

static ALONE_MODIFIER: Lazy<Mutex<AloneModifierMap>> = Lazy::new(Mutex::default);

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
        EVENT_SENDER.lock().unwrap().button.send_event(self.event);
        self.handlers.call();
    }
}

impl<'a> EventCallback for EventHandler<'a, i32> {
    fn get_event_block(&self) -> EventBlock {
        self.get_event_block()
    }

    fn call(&mut self) {
        EVENT_SENDER
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
        EVENT_SENDER
            .lock()
            .unwrap()
            .mouse_cursor
            .send_event(self.event);
        self.handlers.call();
    }
}

pub(crate) struct HookInstaller {
    handler: RuntimeHandler,
    alone_modifier: AloneModifierMap,
}

impl HookInstaller {
    pub(crate) fn install_hook(self) {
        RUNTIME_EVENT_HANDLER.set(self.handler).unwrap();
        *ALONE_MODIFIER.lock().unwrap() = self.alone_modifier;

        INPUT_HANDLER.button.register_handler(move |event| {
            let event_handler = RUNTIME_EVENT_HANDLER.get().unwrap();
            let mut alone_modifier = ALONE_MODIFIER.lock().unwrap();
            let handlers = match event.action {
                ButtonAction::Press => {
                    alone_modifier.emit_press_event(event.target);
                    event_handler.button.on_press.get_satisfied(event)
                }
                ButtonAction::Release => {
                    let mut handlers = event_handler.button.on_release.get_satisfied(event);
                    if alone_modifier.is_alone(event.target) {
                        handlers.extend(event_handler.button.on_release_alone.get_satisfied(event));
                    }
                    handlers
                }
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
        let modifiers_list = Rc::try_unwrap(hook.modifiers_list).unwrap().into_inner();
        let alone_modifier = AloneModifierMap::from(modifiers_list);
        Self {
            handler,
            alone_modifier,
        }
    }
}
