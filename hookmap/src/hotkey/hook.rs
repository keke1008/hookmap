use crate::{button::ButtonSet, hook::Hook, ButtonInput};
use super::modifier_keys::ModifierKeys;
use hookmap_core::{ButtonEvent, NativeEventOperation};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

type HookProcess<E> = Arc<dyn Fn(E)>;

#[derive(Clone)]
pub(super) struct HotkeyOnPressHook {
    modifier_keys: ModifierKeys,
    is_active: Arc<AtomicBool>,
    process: HookProcess<ButtonEvent>,
    native_event_operation: NativeEventOperation,
}
#[derive(Clone)]
pub(super) struct HotkeyOnReleaseHook {
    is_active: Arc<AtomicBool>,
    process: HookProcess<ButtonEvent>,
    native_event_operation: NativeEventOperation,
}
#[derive(Clone)]
pub(super) struct RemapOnPressHook {
    modifier_keys: ModifierKeys,
    is_active: Arc<AtomicBool>,
    button: ButtonSet,
}
#[derive(Clone)]
pub(super) struct RemapOnReleaseHook {
    is_active: Arc<AtomicBool>,
    button: ButtonSet,
}

fn is_on_press_hook_executable(is_active: &AtomicBool, modifier_keys: &ModifierKeys) -> bool {
    let res = is_active.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |old| {
        (!old && modifier_keys.meets_conditions()).then(|| true)
    });
    matches!(res, Ok(_))
}
pub(super) trait ExecutableHook {
    fn is_executable(&self) -> bool;
}
impl ExecutableHook for HotkeyOnPressHook {
    fn is_executable(&self) -> bool {
        is_on_press_hook_executable(&self.is_active, &self.modifier_keys)
    }
}
impl ExecutableHook for HotkeyOnReleaseHook {
    fn is_executable(&self) -> bool {
        self.is_active.swap(false, Ordering::SeqCst)
    }
}
impl ExecutableHook for RemapOnPressHook {
    fn is_executable(&self) -> bool {
        is_on_press_hook_executable(&self.is_active, &self.modifier_keys)
    }
}
impl ExecutableHook for RemapOnReleaseHook {
    fn is_executable(&self) -> bool {
        self.is_active.swap(false, Ordering::SeqCst)
    }
}

#[derive(Clone)]
pub(super) enum ButtonHook {
    HotkeyOnPress(HotkeyOnPressHook),
    HotkeyOnRelease(HotkeyOnReleaseHook),
    RemapOnPress(RemapOnPressHook),
    RemapOnRelease(RemapOnReleaseHook),
}

impl Hook<ButtonEvent> for ButtonHook {
    fn native_event_operation(&self) -> NativeEventOperation {
        match self {
            ButtonHook::HotkeyOnPress(hook) => hook.native_event_operation,
            ButtonHook::HotkeyOnRelease(hook) => hook.native_event_operation,
            _ => NativeEventOperation::Block,
        }
    }

    fn run(&self, event: ButtonEvent) {
        match self {
            ButtonHook::HotkeyOnPress(hook) => (hook.process)(event),
            ButtonHook::HotkeyOnRelease(hook) => (hook.process)(event),
            ButtonHook::RemapOnPress(hook) => hook.button.press(),
            ButtonHook::RemapOnRelease(hook) => hook.button.release(),
        }
    }
}
impl From<HotkeyOnPressHook> for ButtonHook {
    fn from(hook: HotkeyOnPressHook) -> Self {
        ButtonHook::HotkeyOnPress(hook)
    }
}
impl From<HotkeyOnReleaseHook> for ButtonHook {
    fn from(hook: HotkeyOnReleaseHook) -> Self {
        ButtonHook::HotkeyOnRelease(hook)
    }
}
impl From<RemapOnPressHook> for ButtonHook {
    fn from(remap: RemapOnPressHook) -> Self {
        ButtonHook::RemapOnPress(remap)
    }
}
impl From<RemapOnReleaseHook> for ButtonHook {
    fn from(remap: RemapOnReleaseHook) -> Self {
        ButtonHook::RemapOnRelease(remap)
    }
}

#[derive(Clone)]
pub(super) struct MouseHook<E> {
    modifier_keys: ModifierKeys,
    process: HookProcess<E>,
    native_event_operation: NativeEventOperation,
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
