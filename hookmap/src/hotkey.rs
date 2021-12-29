mod modifier_keys;
mod hook;
mod storage;

use crate::button::{ButtonSet, ButtonState};
use hookmap_core::{ButtonEvent, NativeEventOperation};
use std::{fmt::Debug, iter, sync::Arc};
use typed_builder::TypedBuilder;

type ActionFn<E> = Arc<dyn Fn(E) + Send + Sync>;

#[derive(Clone)]
pub(crate) enum Action<E> {
    Single(ActionFn<E>),
    Vec(Vec<ActionFn<E>>),
    Noop,
}

impl<E: Copy> Action<E> {
    fn iter(&self) -> Box<dyn Iterator<Item = ActionFn<E>> + '_> {
        match self {
            Action::Single(callback) => Box::new(iter::once(Arc::clone(callback))),
            Action::Vec(callbacks) => Box::new(callbacks.iter().cloned()),
            Action::Noop => Box::new(iter::empty()),
        }
    }

    pub(super) fn call(&self, event: E) {
        match self {
            Action::Single(callback) => callback(event),
            Action::Vec(callbacks) => callbacks.iter().for_each(move |f| f(event)),
            Action::Noop => {}
        }
    }
}

impl<E, T: Fn(E) + Send + Sync + 'static> From<T> for Action<E> {
    fn from(callback: T) -> Self {
        Action::Single(Arc::new(callback))
    }
}

impl<E: Copy> From<Vec<Action<E>>> for Action<E> {
    fn from(actions: Vec<Action<E>>) -> Self {
        let actions = actions
            .iter()
            .map(Action::iter)
            .flatten()
            .collect::<Vec<_>>();
        Action::Vec(actions)
    }
}

impl<E> Debug for Action<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(std::any::type_name::<Arc<dyn Fn(E) + Send + Sync>>())
    }
}

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq)]
struct ModifierKeysInner {
    pressed: Vec<ButtonSet>,
    released: Vec<ButtonSet>,
}

impl ModifierKeysInner {
    pub(crate) fn new(pressed: &[ButtonSet], released: &[ButtonSet]) -> Self {
        Self {
            pressed: pressed.into(),
            released: released.into(),
        }
    }

    pub(crate) fn add(&self, pressed: &[ButtonSet], released: &[ButtonSet]) -> Self {
        let mut result = self.clone();
        result.pressed.extend_from_slice(pressed);
        result.released.extend_from_slice(released);
        result
    }

    pub(crate) fn satisfies_condition(&self) -> bool {
        self.pressed.iter().all(ButtonState::is_pressed)
            && self.released.iter().all(ButtonState::is_released)
    }
}

#[doc(hidden)]
#[derive(Debug, Default, Hash, PartialEq, Eq)]
pub struct ModifierKeys(Option<ModifierKeysInner>);

impl ModifierKeys {
    pub(crate) fn new(pressed: &[ButtonSet], released: &[ButtonSet]) -> Self {
        Self::default().add(pressed, released)
    }

    pub(crate) fn add(&self, pressed: &[ButtonSet], released: &[ButtonSet]) -> Self {
        let inner = self
            .0
            .as_ref()
            .map(|inner| inner.add(pressed, released))
            .unwrap_or_else(|| ModifierKeysInner::new(pressed, released));
        Self(Some(inner))
    }

    pub(crate) fn satisfies_condition(&self) -> bool {
        self.0
            .as_ref()
            .map(ModifierKeysInner::satisfies_condition)
            .unwrap_or(true)
    }
}

#[doc(hidden)]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Trigger {
    All,
    Just(ButtonSet),
}

#[derive(Debug, Clone)]
pub(crate) enum HotkeyAction {
    Press(Action<ButtonEvent>),
    Release(Action<ButtonEvent>),
    PressOrRelease(Action<ButtonEvent>),
    PressAndRelease {
        on_press: Action<ButtonEvent>,
        on_release: Action<ButtonEvent>,
    },
}

#[derive(Debug, TypedBuilder)]
pub(crate) struct HotkeyInfo {
    pub(crate) trigger: Trigger,
    pub(crate) modifier_keys: Arc<ModifierKeys>,
    pub(crate) native_event_operation: NativeEventOperation,
    pub(crate) action: HotkeyAction,
}

#[derive(Clone, Debug, TypedBuilder)]
pub(crate) struct MouseEventHandler<E> {
    pub(crate) modifier_keys: Arc<ModifierKeys>,
    pub(crate) native_event_operation: NativeEventOperation,
    pub(crate) action: Action<E>,
}

#[derive(Clone, Debug, TypedBuilder)]
pub(crate) struct RemapInfo {
    pub(crate) modifier_keys: Arc<ModifierKeys>,
    pub(crate) trigger: ButtonSet,
    pub(crate) target: ButtonSet,
}
