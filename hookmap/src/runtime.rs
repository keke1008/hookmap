use crate::{handler::Handler, modifier::ModifierEventBlock, Button, Hook};
use hookmap_core::{
    EventBlock, KeyboardAction, KeyboardEvent, MouseAction, MouseEvent, INPUT_HANDLER,
};
use std::{
    rc::Rc,
    sync::{Arc, Mutex},
};

#[derive(Debug)]
pub(super) struct HookInstaller {
    handler: Mutex<Handler>,
    modifier_event_block: ModifierEventBlock,
}

impl HookInstaller {
    pub(super) fn install_hook(self) {
        let this = Arc::new(self);
        INPUT_HANDLER
            .keyboard
            .lock()
            .unwrap()
            .register_handler(Arc::clone(&this).keyboard_handler());
        INPUT_HANDLER
            .mouse
            .lock()
            .unwrap()
            .register_handler(this.mouse_handler());
        INPUT_HANDLER.handle_input();
    }

    fn keyboard_handler(self: Arc<Self>) -> impl FnMut(KeyboardEvent) -> EventBlock {
        move |event: KeyboardEvent| {
            let event_block = self.keyboard_event_block(&event);
            if let Some(event_block) = self.modifier_event_block.keyboard.get(&event.target) {
                *event_block
            } else if event_block.into_iter().any(|e| e == EventBlock::Block) {
                EventBlock::Block
            } else {
                EventBlock::Unblock
            }
        }
    }

    fn mouse_handler(self: Arc<Self>) -> impl FnMut(MouseEvent) -> EventBlock {
        move |event: MouseEvent| {
            let event_block = self.mouse_event_block(&event);

            if let Some(event_block) = self.modifier_event_block.mouse.get(&event.target) {
                *event_block
            } else if event_block.into_iter().any(|e| e == EventBlock::Block) {
                EventBlock::Block
            } else {
                EventBlock::Unblock
            }
        }
    }

    fn keyboard_event_block(&self, event: &KeyboardEvent) -> Vec<EventBlock> {
        let handler = &mut self.handler.lock().unwrap().keyboard;
        match event.action {
            KeyboardAction::Press => {
                let mut event_block = handler
                    .on_press_or_release
                    .call_available(event.target, Button::Press);
                event_block.append(&mut handler.on_press.call_available(event.target, ()));
                event_block
            }
            KeyboardAction::Release => {
                let mut event_block = handler
                    .on_press_or_release
                    .call_available(event.target, Button::Release);
                event_block.append(&mut handler.on_release.call_available(event.target, ()));
                event_block
            }
        }
    }

    fn mouse_event_block(&self, event: &MouseEvent) -> Vec<EventBlock> {
        let handler = &mut self.handler.lock().unwrap().mouse;
        match event.action {
            MouseAction::Press => {
                let mut event_block = handler
                    .on_press_or_release
                    .call_available(event.target, Button::Press);
                event_block.append(&mut handler.on_press.call_available(event.target, ()));
                event_block
            }
            MouseAction::Release => {
                let mut event_block = handler
                    .on_press_or_release
                    .call_available(event.target, Button::Release);
                event_block.append(&mut handler.on_release.call_available(event.target, ()));
                event_block
            }
            MouseAction::Wheel(value) => handler.wheel.call_available(value),
            MouseAction::Move(value) => handler.cursor.call_available(value),
        }
    }
}

impl From<Hook> for HookInstaller {
    fn from(hook: Hook) -> Self {
        let handler = Rc::try_unwrap(hook.handler).unwrap().into_inner();
        let modifier_event_block = Rc::try_unwrap(hook.modifier_event_block)
            .unwrap()
            .into_inner();
        Self {
            handler: Mutex::new(handler),
            modifier_event_block,
        }
    }
}
