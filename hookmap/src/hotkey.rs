use crate::button::{ButtonSet, ButtonState, ToButtonSet};
use hookmap_core::{ButtonAction, ButtonEvent, EventBlock};
use std::{fmt::Debug, sync::Arc};

#[derive(Clone, Copy, Debug)]
pub(crate) enum TriggerAction {
    Press,
    Release,
    PressOrRelease,
}

impl TriggerAction {
    fn is_satisfied(&self, action: ButtonAction) -> bool {
        match self {
            TriggerAction::PressOrRelease => true,
            TriggerAction::Press => action == ButtonAction::Press,
            TriggerAction::Release => action == ButtonAction::Release,
        }
    }
}

#[derive(Debug)]
pub enum ConditionUnit<T: ToButtonSet> {
    Pressed(T),
    Released(T),
}

#[derive(Debug, Default)]
pub(crate) struct Condition {
    pressed: Vec<ButtonSet>,
    released: Vec<ButtonSet>,
}

impl Condition {
    fn is_satisfied(&self) -> bool {
        self.pressed.iter().all(ButtonSet::is_pressed)
            && self.released.iter().all(ButtonSet::is_released)
    }

    fn clone(&self) -> Self {
        Self {
            pressed: self.pressed.clone(),
            released: self.released.clone(),
        }
    }

    pub(crate) fn add<T: ToButtonSet>(&self, condition_unit: ConditionUnit<T>) -> Self {
        let mut this = self.clone();
        match condition_unit {
            ConditionUnit::Pressed(button) => this.pressed.push(button.to_button_set()),
            ConditionUnit::Released(button) => this.released.push(button.to_button_set()),
        }
        this
    }
}

#[derive(Clone)]
pub(crate) struct Action<E>(pub(crate) Arc<dyn Fn(E) + Send + Sync>);

impl<E, T: Fn(E) + Send + Sync + 'static> From<T> for Action<E> {
    fn from(callback: T) -> Self {
        Action(Arc::new(callback))
    }
}

impl<E> Debug for Action<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(std::any::type_name::<Arc<dyn Fn(E) + Send + Sync>>())
    }
}

#[derive(Debug)]
pub(crate) struct Hotkey {
    pub(crate) trigger: ButtonSet,
    pub(crate) trigger_action: TriggerAction,
    pub(crate) condition: Arc<Condition>,
    pub(crate) event_block: EventBlock,
    pub(crate) action: Action<ButtonEvent>,
}

impl Hotkey {
    pub(crate) fn is_satisfied(&self, event: &ButtonEvent) -> bool {
        self.trigger_action.is_satisfied(event.action) && self.condition.is_satisfied()
    }
}

#[derive(Clone, Debug)]
pub(crate) struct PartialHotkeyUsedEntry {
    pub(crate) trigger: ButtonSet,
    pub(crate) condition: Arc<Condition>,
    pub(crate) event_block: EventBlock,
}

impl PartialHotkeyUsedEntry {
    pub(crate) fn build_hotkey(
        self,
        trigger_action: TriggerAction,
        action: Action<ButtonEvent>,
    ) -> Hotkey {
        Hotkey {
            trigger: self.trigger,
            trigger_action,
            condition: self.condition,
            event_block: self.event_block,
            action,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub(crate) struct PartialHotkeyUsedHook {
    pub(crate) condition: Arc<Condition>,
    pub(crate) event_block: EventBlock,
}

impl PartialHotkeyUsedHook {
    pub(crate) fn build_partial_hotkey_used_entry(
        self,
        trigger: ButtonSet,
    ) -> PartialHotkeyUsedEntry {
        PartialHotkeyUsedEntry {
            trigger,
            condition: self.condition,
            event_block: self.event_block,
        }
    }

    pub(crate) fn build_mouse_hotkey<E>(self, action: Action<E>) -> MouseHotkey<E> {
        MouseHotkey {
            condition: self.condition,
            event_block: self.event_block,
            action,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct MouseHotkey<E> {
    pub(crate) condition: Arc<Condition>,
    pub(crate) event_block: EventBlock,
    pub(crate) action: Action<E>,
}

impl<E> MouseHotkey<E> {
    pub(crate) fn is_satisfied(&self) -> bool {
        self.condition.is_satisfied()
    }
}
