use super::{
    alone_modifier::{AloneModifierList, AloneModifierMap},
    interruption::EVENT_SENDER,
    modifier_event_block::ModifierEventBlock,
    runtime_handler::RuntimeHandler,
};
use crate::{handler::ButtonHandler, Hook};

use hookmap_core::{
    ButtonAction, ButtonEvent, EventBlock, KeyboardEvent, MouseEvent, INPUT_HANDLER,
};
use std::{
    hash::Hash,
    rc::Rc,
    sync::{Arc, Mutex},
};

#[derive(Debug)]
pub(crate) struct HookInstaller {
    handler: Mutex<RuntimeHandler>,
    modifier_event_block: ModifierEventBlock,
    alone_modifier: Mutex<AloneModifierList>,
}

impl HookInstaller {
    pub(crate) fn install_hook(self) {
        let this = Arc::new(self);
        INPUT_HANDLER
            .keyboard
            .write()
            .unwrap()
            .register_handler(Arc::clone(&this).keyboard_handler());
        INPUT_HANDLER
            .mouse_button
            .write()
            .unwrap()
            .register_handler(Arc::clone(&this).mouse_button_handler());
        INPUT_HANDLER
            .mouse_wheel
            .write()
            .unwrap()
            .register_handler(Arc::clone(&this).mouse_wheel_handler());
        INPUT_HANDLER
            .mouse_cursor
            .write()
            .unwrap()
            .register_handler(this.mouse_cursor_handler());
        INPUT_HANDLER.handle_input();
    }

    fn keyboard_handler(self: Arc<Self>) -> impl Fn(KeyboardEvent) -> EventBlock {
        move |event: KeyboardEvent| {
            EVENT_SENDER.lock().unwrap().keyboard.send_event(event);
            let handler = &mut self.handler.lock().unwrap().keyboard;
            let alone_modifiers = &mut self.alone_modifier.lock().unwrap().keyboard_alone_modifiers;
            let event_blocks = Self::call_handler(handler, alone_modifiers, &event);
            if let Some(event_block) = self.modifier_event_block.keyboard.get(&event.target) {
                *event_block
            } else {
                Self::determine_event_block(event_blocks)
            }
        }
    }

    fn mouse_button_handler(self: Arc<Self>) -> impl Fn(MouseEvent) -> EventBlock {
        move |event: MouseEvent| {
            EVENT_SENDER.lock().unwrap().mouse_button.send_event(event);
            let handler = &mut self.handler.lock().unwrap().mouse_button;
            let alone_modifiers = &mut self.alone_modifier.lock().unwrap().mouse_alone_modifiers;
            let event_blocks = Self::call_handler(handler, alone_modifiers, &event);
            if let Some(event_block) = self.modifier_event_block.mouse.get(&event.target) {
                *event_block
            } else {
                Self::determine_event_block(event_blocks)
            }
        }
    }

    fn call_handler<T: Hash + Eq + Copy>(
        handler: &mut ButtonHandler<T>,
        alone_modifier: &mut AloneModifierMap<T>,
        event: &ButtonEvent<T>,
    ) -> Vec<EventBlock> {
        let mut event_blocks = handler
            .on_press_or_release
            .call_available(event.target, event.action);
        match event.action {
            ButtonAction::Press => {
                event_blocks.append(&mut handler.on_press.call_available(event.target, ()));
                alone_modifier.press_event(event.target);
            }
            ButtonAction::Release => {
                event_blocks.append(&mut handler.on_release.call_available(event.target, ()));
                if alone_modifier.is_alone(event.target) {
                    event_blocks
                        .append(&mut handler.on_release_alone.call_available(event.target, ()));
                }
            }
        }
        event_blocks
    }

    fn mouse_wheel_handler(self: Arc<Self>) -> impl Fn(i32) -> EventBlock {
        move |event| {
            EVENT_SENDER.lock().unwrap().mouse_wheel.send_event(event);
            let event_blocks = self
                .handler
                .lock()
                .unwrap()
                .mouse_wheel
                .call_available(event);
            Self::determine_event_block(event_blocks)
        }
    }

    fn mouse_cursor_handler(self: Arc<Self>) -> impl Fn((i32, i32)) -> EventBlock {
        move |event| {
            EVENT_SENDER.lock().unwrap().mouse_cursor.send_event(event);
            let event_blocks = self
                .handler
                .lock()
                .unwrap()
                .mouse_cursor
                .call_available(event);
            Self::determine_event_block(event_blocks)
        }
    }

    fn determine_event_block(event_blocks: Vec<EventBlock>) -> EventBlock {
        if event_blocks.into_iter().any(|e| e == EventBlock::Block) {
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
        let modifiers_list = Rc::try_unwrap(hook.modifiers_list).unwrap().into_inner();
        let modifier_event_block = ModifierEventBlock::from(modifiers_list.clone());
        let alone_modifier = AloneModifierList::from(modifiers_list);
        Self {
            handler: Mutex::new(handler),
            modifier_event_block,
            alone_modifier: Mutex::new(alone_modifier),
        }
    }
}
