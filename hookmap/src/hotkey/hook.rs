use hookmap_core::{
    button::{Button, ButtonAction},
    event::ButtonEvent,
    hook::NativeEventOperation,
};

use super::context::Modifiers;
use crate::hook::{ButtonState, Hook};

use std::fmt::Debug;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

#[derive(Clone)]
pub struct Process<E>(Arc<dyn Fn(E) + Send + Sync>);

impl<E> Debug for Process<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Process").field(&"Fn").finish()
    }
}

impl<E, F: Fn(E) + Send + Sync + 'static> From<F> for Process<E> {
    fn from(this: F) -> Self {
        Process(Arc::new(this))
    }
}

impl<E, F: Fn(E) + Send + Sync + 'static> From<Arc<F>> for Process<E> {
    fn from(this: Arc<F>) -> Self {
        Process(this)
    }
}

#[derive(Debug, Clone)]
pub(super) enum Condition {
    Any,
    Activation(Arc<AtomicBool>),
    Modifier(Arc<Modifiers>),
}

impl Condition {
    fn is_satisfied(&self, state: &impl ButtonState) -> bool {
        match self {
            Condition::Any => true,
            Condition::Activation(is_active) => is_active.swap(false, Ordering::SeqCst),
            Condition::Modifier(modifiers) => modifiers.is_matched(state),
        }
    }
}

#[derive(Debug, Clone)]
pub(super) enum HotkeyAction<E> {
    Process(Process<E>),
    Activate(Arc<AtomicBool>),
    Noop,
}

impl<E> HotkeyAction<E> {
    pub(super) fn run(&self, event: E) {
        match self {
            HotkeyAction::Process(callback) => callback.0(event),
            HotkeyAction::Activate(is_active) => is_active.store(true, Ordering::SeqCst),
            HotkeyAction::Noop => {}
        }
    }
}

#[derive(Debug)]
pub(super) struct HotkeyHook {
    condition: Condition,
    action: HotkeyAction<ButtonEvent>,
    native_event_operation: NativeEventOperation,
}

impl HotkeyHook {
    pub(super) fn new(
        condition: Condition,
        action: HotkeyAction<ButtonEvent>,
        native_event_operation: NativeEventOperation,
    ) -> Self {
        HotkeyHook {
            condition,
            action,
            native_event_operation,
        }
    }

    pub(super) fn is_executable(&self, state: &impl ButtonState) -> bool {
        self.condition.is_satisfied(state)
    }
}

#[derive(Debug)]
pub(super) struct RemapHook {
    condition: Condition,
    button: Button,
}

impl RemapHook {
    pub(super) fn new(condition: Condition, button: Button) -> Self {
        assert!(!matches!(condition, Condition::Activation(_)));
        RemapHook { condition, button }
    }

    pub(super) fn is_executable(&self, state: &impl ButtonState) -> bool {
        self.condition.is_satisfied(state)
    }
}

#[derive(Debug)]
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
            ButtonHook::Hotkey(hook) => hook.action.run(event),
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

#[derive(Debug)]
pub(super) struct MouseHook<E> {
    condition: Condition,
    process: Process<E>,
    native_event_operation: NativeEventOperation,
}

impl<E> MouseHook<E> {
    pub(super) fn new(
        condition: Condition,
        process: Process<E>,
        native_event_operation: NativeEventOperation,
    ) -> Self {
        assert!(!matches!(condition, Condition::Activation(_)));
        MouseHook {
            condition,
            process,
            native_event_operation,
        }
    }
}

impl<E> MouseHook<E> {
    pub(super) fn is_executable(&self, state: &impl ButtonState) -> bool {
        self.condition.is_satisfied(state)
    }
}

impl<E> Hook<E> for MouseHook<E> {
    fn native_event_operation(&self) -> NativeEventOperation {
        self.native_event_operation
    }

    fn run(&self, event: E) {
        self.process.0(event);
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
