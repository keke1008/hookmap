use hookmap_core::button::Button;
use hookmap_core::event::NativeEventOperation;

use super::hook::Condition;
use crate::hook::ButtonState;
use crate::macros::button_arg::ButtonArg;

use std::sync::Arc;

/// Represents hotkey information.
///
/// # Examples
///
/// ```
/// use hookmap::prelude::*;
/// Context::new()
///     .modifiers(buttons!(A, B, C))
///     .native_event_operation(NativeEventOperation::Block);
/// ```
///
#[derive(Debug, Default, Clone)]
pub struct Context {
    modifiers: Option<Arc<Modifiers>>,
    pub(crate) native_event_operation: NativeEventOperation,
}

impl Context {
    /// Creates a new instance of [`Context`].
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::prelude::*;
    /// let context = Context::new();
    /// ```
    ///
    pub fn new() -> Self {
        Self::default()
    }

    /// Merges other [`Context`] into self.
    /// `other` will not be changed.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::prelude::*;
    /// let context = Context::new().modifiers(buttons!(A, B));
    /// Context::new()
    ///     .modifiers(buttons!(C, D))
    ///     .merge(&context);
    /// ```
    ///
    pub fn merge(mut self, other: &Self) -> Self {
        self.modifiers = match (self.modifiers.as_ref(), other.modifiers.as_ref()) {
            (Some(s), Some(o)) => Some(Arc::new(s.merge(o))),
            (Some(m), None) | (None, Some(m)) => Some(Arc::clone(m)),
            (None, None) => None,
        };

        use NativeEventOperation::{Block, Dispatch};
        self.native_event_operation =
            match (self.native_event_operation, other.native_event_operation) {
                (Dispatch, Dispatch) => Dispatch,
                _ => Block,
            };

        self
    }

    /// Adds modifier keys to the hotkey to be registered.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::prelude::*;
    /// Context::new()
    ///     .modifiers(buttons!(A, B));
    /// ```
    ///
    pub fn modifiers(mut self, modifiers: impl Into<ButtonArg>) -> Self {
        self.modifiers = Some(Arc::new(Modifiers::from(modifiers.into())));
        self
    }

    /// Indicates whether to block the native event when the hotkey is active.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::prelude::*;
    /// Context::new()
    ///     .native_event_operation(NativeEventOperation::Block);
    /// ```
    ///
    pub fn native_event_operation(mut self, native_event_operation: NativeEventOperation) -> Self {
        self.native_event_operation = native_event_operation;
        self
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
    pub fn merge(&self, other: &Self) -> Self {
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
