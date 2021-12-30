use super::modifier_keys::ModifierKeys;
use crate::{
    button::{ButtonInput, ButtonSet},
    hook::Hook,
};
use hookmap_core::{ButtonAction, ButtonEvent, NativeEventOperation};
use std::sync::Arc;

pub(super) type HookProcess<E> = Arc<dyn Fn(E) + Send + Sync>;

#[derive(Clone)]
pub(super) struct HotkeyHook {
    modifier_keys: Arc<ModifierKeys>,
    process: HookProcess<ButtonEvent>,
    native_event_operation: NativeEventOperation,
}

impl HotkeyHook {
    pub(super) fn new(
        modifier_keys: Arc<ModifierKeys>,
        process: HookProcess<ButtonEvent>,
        native_event_operation: NativeEventOperation,
    ) -> Self {
        HotkeyHook {
            modifier_keys,
            process,
            native_event_operation,
        }
    }
}

#[derive(Clone)]
pub(super) struct RemapHook {
    modifier_keys: Arc<ModifierKeys>,
    button: ButtonSet,
}

impl RemapHook {
    pub(super) fn new(modifier_keys: Arc<ModifierKeys>, button: ButtonSet) -> Self {
        RemapHook {
            modifier_keys,
            button,
        }
    }
}

pub(super) trait ExecutableHook {
    fn is_executable(&self) -> bool;
}
impl ExecutableHook for HotkeyHook {
    fn is_executable(&self) -> bool {
        self.modifier_keys.meets_conditions()
    }
}
impl ExecutableHook for RemapHook {
    fn is_executable(&self) -> bool {
        self.modifier_keys.meets_conditions()
    }
}

#[derive(Clone)]
pub(super) enum ButtonHook {
    Hotkey(HotkeyHook),
    Remap(RemapHook),
}

impl Hook<ButtonEvent> for ButtonHook {
    fn native_event_operation(&self) -> NativeEventOperation {
        match self {
            ButtonHook::Hotkey(hook) => hook.native_event_operation,
            _ => NativeEventOperation::Block,
        }
    }

    fn run(&self, event: ButtonEvent) {
        match self {
            ButtonHook::Hotkey(hook) => (hook.process)(event),
            ButtonHook::Remap(hook) => match event.action {
                ButtonAction::Press => hook.button.press(),
                ButtonAction::Release => hook.button.release(),
            },
        }
    }
}
impl From<HotkeyHook> for ButtonHook {
    fn from(hook: HotkeyHook) -> Self {
        ButtonHook::Hotkey(hook)
    }
}
impl From<RemapHook> for ButtonHook {
    fn from(remap: RemapHook) -> Self {
        ButtonHook::Remap(remap)
    }
}

#[derive(Clone)]
pub(super) struct MouseHook<E> {
    modifier_keys: Arc<ModifierKeys>,
    process: HookProcess<E>,
    native_event_operation: NativeEventOperation,
}

impl<E> MouseHook<E> {
    pub(super) fn new(
        modifier_keys: Arc<ModifierKeys>,
        process: HookProcess<E>,
        native_event_operation: NativeEventOperation,
    ) -> Self {
        MouseHook {
            modifier_keys,
            process,
            native_event_operation,
        }
    }
}

impl<E> Hook<E> for MouseHook<E> {
    fn native_event_operation(&self) -> NativeEventOperation {
        self.native_event_operation
    }

    fn run(&self, event: E) {
        (self.process)(event);
    }
}

impl<E> ExecutableHook for MouseHook<E> {
    fn is_executable(&self) -> bool {
        self.modifier_keys.meets_conditions()
    }
}
