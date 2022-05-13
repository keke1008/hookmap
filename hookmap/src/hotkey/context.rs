use hookmap_core::button::Button;
use hookmap_core::event::NativeEventOperation;

use super::hook::Condition;
use crate::hook::ButtonState;
use crate::macros::button_arg::ButtonArg;

use std::sync::Arc;

#[derive(Debug, Default, Clone)]
pub struct ContextBuilder {
    modifiers: Option<Modifiers>,
    native_event_operation: NativeEventOperation,
}

impl ContextBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn modifiers(mut self, modifiers: impl Into<ButtonArg>) -> Self {
        self.modifiers = Some(modifiers.into().into());
        self
    }

    pub fn native_event_operation(mut self, operation: NativeEventOperation) -> Self {
        self.native_event_operation = operation;
        self
    }

    pub fn build(self) -> Context {
        Context {
            modifiers: self.modifiers.map(Arc::new),
            native_event_operation: self.native_event_operation,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Context {
    modifiers: Option<Arc<Modifiers>>,
    native_event_operation: NativeEventOperation,
}

impl Context {
    pub(super) fn native_event_operation(&self) -> NativeEventOperation {
        self.native_event_operation
    }

    pub(super) fn has_no_modifiers(&self) -> bool {
        self.modifiers.is_none()
    }

    pub(super) fn to_condition(&self) -> Condition {
        self.modifiers
            .clone()
            .map_or(Condition::Any, Condition::Modifier)
    }

    pub(super) fn iter_pressed(&self) -> impl Iterator<Item = &Button> {
        self.modifiers.iter().flat_map(|m| m.iter_pressed())
    }

    pub(super) fn iter_released(&self) -> impl Iterator<Item = &Button> {
        self.modifiers.iter().flat_map(|m| m.iter_released())
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct Modifiers {
    pressed: Vec<Button>,
    released: Vec<Button>,
}

impl Modifiers {
    pub fn merge(&self, other: Self) -> Self {
        Modifiers {
            pressed: self
                .pressed
                .iter()
                .chain(other.pressed.iter())
                .cloned()
                .collect(),
            released: self
                .released
                .iter()
                .chain(other.released.iter())
                .cloned()
                .collect(),
        }
    }
    pub(super) fn is_matched(&self, state: &impl ButtonState) -> bool {
        self.iter_pressed().all(|&b| state.is_pressed(b))
            && self.iter_released().all(|&b| state.is_released(b))
    }

    pub(super) fn iter_pressed(&self) -> std::slice::Iter<Button> {
        self.pressed.iter()
    }

    pub(super) fn iter_released(&self) -> std::slice::Iter<Button> {
        self.released.iter()
    }
}

impl From<ButtonArg> for Modifiers {
    fn from(args: ButtonArg) -> Self {
        Self {
            pressed: args.iter_plain().collect(),
            released: args.iter_not().collect(),
        }
    }
}
