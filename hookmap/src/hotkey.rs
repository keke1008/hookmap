use crate::ButtonState;
use hookmap_core::{Button, ButtonAction, ButtonEvent, EventBlock};
use std::{fmt::Debug, iter, sync::Arc};

#[derive(Clone, Copy, Debug)]
pub(crate) enum TriggerAction {
    Press,
    Release,
    PressOrRelease,
}

impl TriggerAction {
    pub(crate) fn is_satisfied(&self, action: ButtonAction) -> bool {
        match self {
            TriggerAction::PressOrRelease => true,
            TriggerAction::Press => action == ButtonAction::Press,
            TriggerAction::Release => action == ButtonAction::Release,
        }
    }
}

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

#[derive(Debug, Default, Hash, PartialEq, Eq)]
pub(crate) struct Modifier(Vec<Button>);

impl Modifier {
    pub(crate) fn new(modifier: &[Button]) -> Self {
        Self(modifier.into())
    }

    pub(crate) fn add(&self, modifier: &[Button]) -> Self {
        let inner = self
            .0
            .iter()
            .cloned()
            .chain(modifier.iter().cloned())
            .collect();
        Self(inner)
    }

    pub(crate) fn is_all_pressed(&self) -> bool {
        self.0.iter().all(|button| button.is_pressed())
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &Button> {
        self.0.iter()
    }
}

#[derive(Debug)]
pub(crate) struct HotkeyInfo {
    pub(crate) trigger: Button,
    pub(crate) trigger_action: TriggerAction,
    pub(crate) modifier: Arc<Modifier>,
    pub(crate) event_block: EventBlock,
    pub(crate) action: Action<ButtonEvent>,
}

#[derive(Clone, Debug)]
pub(crate) struct MouseEventHandler<E> {
    pub(crate) modifier: Arc<Modifier>,
    pub(crate) event_block: EventBlock,
    pub(crate) action: Action<E>,
}
