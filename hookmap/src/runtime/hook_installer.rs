use super::runtime_handler::RuntimeHandler;
use crate::{handler::ButtonHandler, modifier::ModifierEventBlock, Hook};
use hookmap_core::{ButtonAction, Event, EventBlock, KeyboardEvent, MouseEvent, INPUT_HANDLER};
use std::{
    hash::Hash,
    rc::Rc,
    sync::{Arc, Mutex},
};

#[derive(Debug)]
pub(crate) struct HookInstaller {
    handler: Mutex<RuntimeHandler>,
    modifier_event_block: ModifierEventBlock,
}

impl HookInstaller {
    pub(crate) fn install_hook(self) {
        let this = Arc::new(self);
        INPUT_HANDLER
            .keyboard
            .lock()
            .unwrap()
            .register_handler(Arc::clone(&this).keyboard_handler());
        INPUT_HANDLER
            .mouse_button
            .lock()
            .unwrap()
            .register_handler(Arc::clone(&this).mouse_button_handler());
        INPUT_HANDLER
            .mouse_wheel
            .lock()
            .unwrap()
            .register_handler(Arc::clone(&this).mouse_wheel_handler());
        INPUT_HANDLER
            .mouse_cursor
            .lock()
            .unwrap()
            .register_handler(this.mouse_cursor_handler());
        INPUT_HANDLER.handle_input();
    }

    fn keyboard_handler(self: Arc<Self>) -> impl FnMut(KeyboardEvent) -> EventBlock {
        move |event: KeyboardEvent| {
            let handler = &mut self.handler.lock().unwrap().keyboard;
            let event_block = Self::call_handler(handler, &event);
            if let Some(event_block) = self.modifier_event_block.keyboard.get(&event.target) {
                *event_block
            } else {
                Self::determine_event_block(event_block)
            }
        }
    }

    fn mouse_button_handler(self: Arc<Self>) -> impl FnMut(MouseEvent) -> EventBlock {
        move |event: MouseEvent| {
            let handler = &mut self.handler.lock().unwrap().mouse_button;
            let event_block = Self::call_handler(handler, &event);
            if let Some(event_block) = self.modifier_event_block.mouse.get(&event.target) {
                *event_block
            } else {
                Self::determine_event_block(event_block)
            }
        }
    }

    fn call_handler<T: Hash + Eq + Copy>(
        handler: &mut ButtonHandler<T>,
        event: &Event<T>,
    ) -> Vec<EventBlock> {
        let mut event_block = handler
            .on_press_or_release
            .call_available(event.target, event.action);
        match event.action {
            ButtonAction::Press => {
                event_block.append(&mut handler.on_press.call_available(event.target, ()));
            }
            ButtonAction::Release => {
                event_block.append(&mut handler.on_release.call_available(event.target, ()));
            }
        }
        event_block
    }

    fn mouse_wheel_handler(self: Arc<Self>) -> impl FnMut(i32) -> EventBlock {
        move |event_info| {
            let event_block = self
                .handler
                .lock()
                .unwrap()
                .mouse_wheel
                .call_available(event_info);
            Self::determine_event_block(event_block)
        }
    }

    fn mouse_cursor_handler(self: Arc<Self>) -> impl FnMut((i32, i32)) -> EventBlock {
        move |event_info| {
            let event_block = self
                .handler
                .lock()
                .unwrap()
                .mouse_cursor
                .call_available(event_info);
            Self::determine_event_block(event_block)
        }
    }

    fn determine_event_block(event_block: Vec<EventBlock>) -> EventBlock {
        if event_block.into_iter().any(|e| e == EventBlock::Block) {
            EventBlock::Block
        } else {
            EventBlock::Unblock
        }
    }
}

impl From<Hook> for HookInstaller {
    fn from(hook: Hook) -> Self {
        let handler = Rc::try_unwrap(hook.handler).unwrap();
        let handler = RuntimeHandler::from(handler);
        let modifier_event_block = Rc::try_unwrap(hook.modifier_event_block)
            .unwrap()
            .into_inner();
        Self {
            handler: Mutex::new(handler),
            modifier_event_block,
        }
    }
}
