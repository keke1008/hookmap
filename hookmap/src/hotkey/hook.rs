use super::modifier_keys::ModifierKeys;
use crate::{button::ButtonInput, hook::Hook};
use hookmap_core::{Button, ButtonAction, ButtonEvent, NativeEventOperation};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

pub(super) type HookProcess<E> = Arc<dyn Fn(E) + Send + Sync>;

pub(super) trait ExecutableHook {
    fn is_executable(&self) -> bool;
}

pub(super) enum HotkeyCondition {
    Any,
    Activation(Arc<AtomicBool>),
    Modifier(Arc<ModifierKeys>),
}

impl HotkeyCondition {
    fn is_executable(&self) -> bool {
        match self {
            HotkeyCondition::Any => true,
            HotkeyCondition::Activation(is_active) => is_active.swap(false, Ordering::SeqCst),
            HotkeyCondition::Modifier(modifier_keys) => modifier_keys.meets_conditions(),
        }
    }
}

pub(super) enum HotkeyProcess<E> {
    Callback(HookProcess<E>),
    Activate(Arc<AtomicBool>),
    Noop,
}

impl<E> HotkeyProcess<E> {
    pub(super) fn run(&self, event: E) {
        match self {
            HotkeyProcess::Callback(callback) => callback(event),
            HotkeyProcess::Activate(is_active) => is_active.store(true, Ordering::SeqCst),
            HotkeyProcess::Noop => {}
        }
    }
}

pub(super) struct HotkeyHook {
    condition: HotkeyCondition,
    process: HotkeyProcess<ButtonEvent>,
    native_event_operation: NativeEventOperation,
}

impl ExecutableHook for HotkeyHook {
    fn is_executable(&self) -> bool {
        self.condition.is_executable()
    }
}

impl HotkeyHook {
    pub(super) fn new(
        condition: HotkeyCondition,
        process: HotkeyProcess<ButtonEvent>,
        native_event_operation: NativeEventOperation,
    ) -> Self {
        HotkeyHook {
            condition,
            process,
            native_event_operation,
        }
    }
}

pub(super) struct RemapHook {
    modifier_keys: Arc<ModifierKeys>,
    button: Button,
}

impl RemapHook {
    pub(super) fn new(modifier_keys: Arc<ModifierKeys>, button: Button) -> Self {
        RemapHook {
            modifier_keys,
            button,
        }
    }
}

impl ExecutableHook for RemapHook {
    fn is_executable(&self) -> bool {
        self.modifier_keys.meets_conditions()
    }
}

pub(super) enum ButtonHook {
    Hotkey(Arc<HotkeyHook>),
    Remap(Arc<RemapHook>),
}

impl Hook<ButtonEvent> for ButtonHook {
    fn native_event_operation(&self) -> NativeEventOperation {
        match self {
            ButtonHook::Hotkey(hook) => hook.native_event_operation,
            ButtonHook::Remap(_) => NativeEventOperation::Block,
        }
    }

    fn run(&self, event: ButtonEvent) {
        match self {
            ButtonHook::Hotkey(hook) => hook.process.run(event),
            ButtonHook::Remap(hook) => match event.action {
                ButtonAction::Press => hook.button.press(),
                ButtonAction::Release => hook.button.release(),
            },
        }
    }
}
impl From<Arc<HotkeyHook>> for ButtonHook {
    fn from(hook: Arc<HotkeyHook>) -> Self {
        ButtonHook::Hotkey(hook)
    }
}
impl From<Arc<RemapHook>> for ButtonHook {
    fn from(remap: Arc<RemapHook>) -> Self {
        ButtonHook::Remap(remap)
    }
}

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

impl<E, T: Hook<E>> Hook<E> for Arc<T> {
    fn native_event_operation(&self) -> NativeEventOperation {
        (**self).native_event_operation()
    }

    fn run(&self, event: E) {
        (**self).run(event);
    }
}
