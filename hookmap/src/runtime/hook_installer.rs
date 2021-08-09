use super::{
    alone_modifier::AloneModifierMap, interruption::EVENT_SENDER, runtime_handler::RuntimeHandler,
};
use crate::Hook;
use hookmap_core::{ButtonAction, ButtonEvent, INPUT_HANDLER};
use std::{
    rc::Rc,
    sync::{Arc, Mutex},
};

#[derive(Debug)]
pub(crate) struct HookInstaller {
    handler: Mutex<RuntimeHandler>,
    alone_modifier: Mutex<AloneModifierMap>,
}

impl HookInstaller {
    pub(crate) fn install_hook(self) {
        let this = Arc::new(self);
        INPUT_HANDLER
            .button
            .register_handler(Arc::clone(&this).generate_button_handler());
        INPUT_HANDLER
            .mouse_wheel
            .register_handler(Arc::clone(&this).generate_mouse_wheel_handler());
        INPUT_HANDLER
            .mouse_cursor
            .register_handler(this.mouse_cursor_handler());
        INPUT_HANDLER.handle_input();
    }

    fn generate_button_handler(self: Arc<Self>) -> impl Fn(ButtonEvent) {
        move |event: ButtonEvent| {
            EVENT_SENDER.lock().unwrap().button.send_event(event);
            let handler = &mut self.handler.lock().unwrap().button;
            let alone_modifiers = &mut self.alone_modifier.lock().unwrap();
            handler
                .on_press_or_release
                .call_available(event.target, event.action);
            match event.action {
                ButtonAction::Press => {
                    handler.on_press.call_available(event.target, ());
                    alone_modifiers.emit_press_event(event.target);
                }
                ButtonAction::Release => {
                    handler.on_release.call_available(event.target, ());
                    if alone_modifiers.is_alone(event.target) {
                        handler.on_release_alone.call_available(event.target, ());
                    }
                }
            }
        }
    }

    fn generate_mouse_wheel_handler(self: Arc<Self>) -> impl Fn(i32) {
        move |event| {
            EVENT_SENDER.lock().unwrap().mouse_wheel.send_event(event);
            self.handler
                .lock()
                .unwrap()
                .mouse_wheel
                .call_available(event);
        }
    }

    fn mouse_cursor_handler(self: Arc<Self>) -> impl Fn((i32, i32)) {
        move |event| {
            EVENT_SENDER.lock().unwrap().mouse_cursor.send_event(event);
            self.handler
                .lock()
                .unwrap()
                .mouse_cursor
                .call_available(event);
        }
    }
}

impl From<Hook> for HookInstaller {
    fn from(hook: Hook) -> Self {
        let handler = Rc::try_unwrap(hook.handler).unwrap();
        let handler = RuntimeHandler::from(handler);
        let modifiers_list = Rc::try_unwrap(hook.modifiers_list).unwrap().into_inner();
        let alone_modifier = AloneModifierMap::from(modifiers_list);
        Self {
            handler: Mutex::new(handler),
            alone_modifier: Mutex::new(alone_modifier),
        }
    }
}
