use std::cell::RefCell;
use std::sync::Arc;

use super::{
    button_arg::{ButtonArg, ButtonArgElementTag},
    hook::{HookProcess, HotkeyCondition, HotkeyHook, HotkeyProcess, MouseHook, RemapHook},
    modifiers::Modifiers,
    storage::HotkeyStorage,
};
use crate::{
    button::Button,
    event::{ButtonEvent, MouseCursorEvent, MouseWheelEvent, NativeEventOperation},
};

#[derive(Debug, Default, Clone)]
pub(super) struct Context {
    pub(super) modifiers: Option<Arc<Modifiers>>,
    pub(super) native_event_operation: NativeEventOperation,
}

#[derive(Default)]
pub(super) struct HotkeyEntry {
    storage: RefCell<HotkeyStorage>,
}

impl HotkeyEntry {
    pub fn into_inner(self) -> HotkeyStorage {
        self.storage.into_inner()
    }

    pub(super) fn remap(&self, targets: ButtonArg, behavior: Button, context: Context) {
        let mut storage = self.storage.borrow_mut();
        let hook = Arc::new(RemapHook::new(context.modifiers.into(), behavior));
        for target in targets.iter() {
            match target.tag {
                ButtonArgElementTag::Inversion => panic!(),
                ButtonArgElementTag::Direct => {
                    storage.register_remap(target.value, Arc::clone(&hook));
                }
            }
        }
    }

    pub(super) fn on_press(
        &self,
        targets: ButtonArg,
        process: HookProcess<ButtonEvent>,
        context: Context,
    ) {
        let mut storage = self.storage.borrow_mut();
        let hook = Arc::new(HotkeyHook::new(
            context.modifiers.into(),
            HotkeyProcess::Callback(process),
            context.native_event_operation,
        ));
        for target in targets.iter() {
            match target.tag {
                ButtonArgElementTag::Direct => {
                    storage.register_hotkey_on_press(target.value, Arc::clone(&hook));
                }
                ButtonArgElementTag::Inversion => {
                    storage.register_hotkey_on_release(target.value, Arc::clone(&hook));
                }
            }
        }
    }

    pub(super) fn on_release(
        &self,
        targets: ButtonArg,
        process: HookProcess<ButtonEvent>,
        context: Context,
    ) {
        let mut storage = self.storage.borrow_mut();
        if context.modifiers.is_none() {
            let hook = Arc::new(HotkeyHook::new(
                HotkeyCondition::Any,
                HotkeyProcess::Callback(process),
                context.native_event_operation,
            ));

            for target in targets.iter() {
                match target.tag {
                    ButtonArgElementTag::Direct => {
                        storage.register_hotkey_on_release(target.value, Arc::clone(&hook));
                    }
                    ButtonArgElementTag::Inversion => {
                        storage.register_hotkey_on_press(target.value, Arc::clone(&hook));
                    }
                }
            }
            return;
        }
        let modifiers = context.modifiers.unwrap();
        for target in targets.iter() {
            let is_active = Arc::default();
            let inactivation_hook = Arc::new(HotkeyHook::new(
                HotkeyCondition::Activation(Arc::clone(&is_active)),
                HotkeyProcess::Callback(Arc::clone(&process)),
                context.native_event_operation,
            ));
            let is_active = Arc::clone(&is_active);
            let activation_hook = Arc::new(HotkeyHook::new(
                HotkeyCondition::Modifier(Arc::clone(&modifiers)),
                HotkeyProcess::Activate(Arc::clone(&is_active)),
                NativeEventOperation::Dispatch,
            ));

            match target.tag {
                ButtonArgElementTag::Direct => {
                    storage.register_hotkey_on_press(target.value, Arc::clone(&activation_hook));
                    storage
                        .register_hotkey_on_release(target.value, Arc::clone(&inactivation_hook));
                }
                ButtonArgElementTag::Inversion => {
                    storage.register_hotkey_on_release(target.value, Arc::clone(&activation_hook));
                    storage.register_hotkey_on_press(target.value, Arc::clone(&inactivation_hook));
                }
            }
            for target in &modifiers.pressed {
                storage.register_hotkey_on_release(*target, Arc::clone(&inactivation_hook));
            }
            for target in &modifiers.released {
                storage.register_hotkey_on_press(*target, Arc::clone(&inactivation_hook));
            }
        }
    }

    pub(super) fn mouse_wheel(&self, process: HookProcess<MouseWheelEvent>, context: Context) {
        let hook = Arc::new(MouseHook::new(
            context.modifiers.into(),
            process,
            context.native_event_operation,
        ));
        self.storage.borrow_mut().register_mouse_wheel_hotkey(hook);
    }

    pub(super) fn mouse_cursor(&self, process: HookProcess<MouseCursorEvent>, context: Context) {
        let hook = Arc::new(MouseHook::new(
            context.modifiers.into(),
            process,
            context.native_event_operation,
        ));
        self.storage.borrow_mut().register_mouse_cursor_hotkey(hook);
    }

    pub(super) fn disable(&self, targets: ButtonArg, context: Context) {
        let mut storage = self.storage.borrow_mut();
        let hook = Arc::new(HotkeyHook::new(
            context.modifiers.into(),
            HotkeyProcess::Noop,
            context.native_event_operation,
        ));
        for target in targets.iter() {
            storage.register_hotkey_on_press(target.value, Arc::clone(&hook));
            storage.register_hotkey_on_release(target.value, Arc::clone(&hook));
        }
    }
}
