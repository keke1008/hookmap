use super::{
    hook::{Condition, HotkeyCondition},
    modifiers::Modifiers,
};
use crate::{button::Button, event::NativeEventOperation};

use std::sync::Arc;

#[derive(Debug, Default, Clone)]
pub struct Context {
    pub(super) modifiers: Option<Arc<Modifiers>>,
    pub(super) native_event_operation: NativeEventOperation,
}

impl Context {
    pub(super) fn to_condition(&self) -> Condition {
        self.modifiers
            .clone()
            .map_or(Condition::Any, Condition::Modifier)
    }

    pub(super) fn to_hotkey_condition(&self) -> HotkeyCondition {
        self.modifiers
            .clone()
            .map_or(HotkeyCondition::Any, HotkeyCondition::Modifier)
    }

    pub(super) fn iter_pressed(&self) -> impl Iterator<Item = &Button> {
        self.modifiers.iter().flat_map(|m| m.iter_pressed())
    }

    pub(super) fn iter_released(&self) -> impl Iterator<Item = &Button> {
        self.modifiers.iter().flat_map(|m| m.iter_released())
    }
}
