use std::cell::RefCell;
use std::sync::Arc;

use super::{
    button_arg::{ButtonArg, ButtonArgElementTag},
    hook::{
        Condition, HookProcess, HotkeyCondition, HotkeyHook, HotkeyProcess, MouseHook, RemapHook,
    },
    modifier_keys::ModifierKeys,
    storage::HotkeyStorage,
};
use crate::{
    button::Button,
    event::{ButtonEvent, MouseCursorEvent, MouseWheelEvent, NativeEventOperation},
};

#[derive(Debug, Default, Clone)]
pub(super) struct Context {
    pub(super) modifier_keys: Option<Arc<ModifierKeys>>,
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
        let hook = Arc::new(RemapHook::new(
            context
                .modifier_keys
                .map(Condition::Modifier)
                .unwrap_or(Condition::Any),
            behavior,
        ));
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
            context
                .modifier_keys
                .map(HotkeyCondition::Modifier)
                .unwrap_or(HotkeyCondition::Any),
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
        if context.modifier_keys.is_none() {
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
        let modifier_keys = context.modifier_keys.unwrap();
        for target in targets.iter() {
            let is_active = Arc::default();
            let inactivation_hook = Arc::new(HotkeyHook::new(
                HotkeyCondition::Activation(Arc::clone(&is_active)),
                HotkeyProcess::Callback(Arc::clone(&process)),
                context.native_event_operation,
            ));
            let is_active = Arc::clone(&is_active);
            let activation_hook = Arc::new(HotkeyHook::new(
                HotkeyCondition::Modifier(Arc::clone(&modifier_keys)),
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
            for target in &modifier_keys.pressed {
                storage.register_hotkey_on_release(*target, Arc::clone(&inactivation_hook));
            }
            for target in &modifier_keys.released {
                storage.register_hotkey_on_press(*target, Arc::clone(&inactivation_hook));
            }
        }
    }

    pub(super) fn mouse_wheel(&self, process: HookProcess<MouseWheelEvent>, context: Context) {
        let hook = Arc::new(MouseHook::new(
            context
                .modifier_keys
                .map(Condition::Modifier)
                .unwrap_or(Condition::Any),
            process,
            context.native_event_operation,
        ));
        self.storage.borrow_mut().register_mouse_wheel_hotkey(hook);
    }

    pub(super) fn mouse_cursor(&self, process: HookProcess<MouseCursorEvent>, context: Context) {
        let hook = Arc::new(MouseHook::new(
            context
                .modifier_keys
                .map(Condition::Modifier)
                .unwrap_or(Condition::Any),
            process,
            context.native_event_operation,
        ));
        self.storage.borrow_mut().register_mouse_cursor_hotkey(hook);
    }

    pub(super) fn disable(&self, targets: ButtonArg, context: Context) {
        let mut storage = self.storage.borrow_mut();
        let hook = Arc::new(HotkeyHook::new(
            context
                .modifier_keys
                .map(HotkeyCondition::Modifier)
                .unwrap_or(HotkeyCondition::Any),
            HotkeyProcess::Noop,
            context.native_event_operation,
        ));
        for target in targets.iter() {
            storage.register_hotkey_on_press(target.value, Arc::clone(&hook));
            storage.register_hotkey_on_release(target.value, Arc::clone(&hook));
        }
    }
}
