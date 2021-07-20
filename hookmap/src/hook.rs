use crate::{
    event::Button,
    handler::Handler,
    modifier::Modifier,
    register::{KeyboardRegister, MouseRegister},
};
use hookmap_core::{
    keyboard::{Key, KeyboardAction},
    mouse::{MouseAction, MouseInput},
    EventBlock, INPUT_HANDLER,
};
use std::sync::{Arc, Mutex};

#[derive(Debug, Default)]
pub struct Hook {
    handler: Arc<Mutex<Handler>>,
    modifier: Modifier,
}

impl Hook {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bind_key(&self, key: Key) -> KeyboardRegister {
        KeyboardRegister::new(key, self.modifier.clone(), Arc::clone(&self.handler))
    }

    pub fn bind_mouse(&self, mouse: MouseInput) -> MouseRegister {
        MouseRegister::new(mouse, self.modifier.clone(), Arc::clone(&self.handler))
    }

    pub fn modifier_key(&self, key: Key) -> Self {
        Self {
            modifier: self.modifier.added_key(key),
            handler: Arc::clone(&self.handler),
        }
    }

    pub fn modifier_mouse_button(&self, mouse_button: MouseInput) -> Self {
        Self {
            modifier: self.modifier.added_mouse_button(mouse_button),
            handler: Arc::clone(&self.handler),
        }
    }

    pub fn handle_input(self) {
        let handler = Arc::clone(&self.handler);
        INPUT_HANDLER.keyboard.register_handler(move |event| {
            let handler = &mut handler.lock().unwrap().keyboard;
            let key = event.target;
            let event_block: Vec<EventBlock> = match event.action {
                KeyboardAction::Press => {
                    let mut event_block = handler
                        .on_press_or_release
                        .call_available(key, Button::Press);
                    event_block.append(&mut handler.on_press.call_available(key, ()));
                    event_block
                }
                KeyboardAction::Release => {
                    let mut event_block = handler
                        .on_press_or_release
                        .call_available(key, Button::Release);
                    event_block.append(&mut handler.on_release.call_available(key, ()));
                    event_block
                }
            };

            if event_block.into_iter().any(|e| e == EventBlock::Block) {
                EventBlock::Block
            } else {
                EventBlock::Unblock
            }
        });

        let handler = Arc::clone(&self.handler);
        INPUT_HANDLER.mouse.register_handler(move |event| {
            let handler = &mut handler.lock().unwrap().mouse;
            let mouse = event.target;
            let event_block = match event.action {
                MouseAction::Press => {
                    let mut event_block = handler
                        .on_press_or_release
                        .call_available(mouse, Button::Press);
                    event_block.append(&mut handler.on_press.call_available(mouse, ()));
                    event_block
                }
                MouseAction::Release => {
                    let mut event_block = handler
                        .on_press_or_release
                        .call_available(mouse, Button::Release);
                    event_block.append(&mut handler.on_release.call_available(mouse, ()));
                    event_block
                }
                MouseAction::Wheel(value) => handler.wheel.call_available(value),
                MouseAction::Move(value) => handler.cursor.call_available(value),
            };

            if event_block.into_iter().any(|e| e == EventBlock::Block) {
                EventBlock::Block
            } else {
                EventBlock::Unblock
            }
        });

        INPUT_HANDLER.handle_input();
    }
}
