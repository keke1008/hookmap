use super::{
    button_event_handler_entry::ButtonEventHandlerEntry,
    hotkey_info::ConditionalHotkeyInfo,
    mouse_event_handler_entry::{MouseCursorHotKeyEntry, MouseWheelHotkeyEntry},
    SelectHandleTarget, SetEventBlock,
};
use crate::{hotkey::ButtonSet, runtime::Register};
use hookmap_core::{Button, EventBlock};
use std::{cell::RefCell, rc::Weak, sync::Arc};

/// A struct for selecting the target of the conditional hook.
///
/// # Example
///
/// ```
/// use hookmap::*;
/// let hotkey = Hotkey::new();
/// let mod_ctrl = hotkey.add_modifiers((&[Button::LCtrl], &[]));
/// mod_ctrl.bind(Button::H).like(Button::LeftArrow);
/// ```
///
pub struct ConditionalHook {
    register: Weak<RefCell<Register>>,
    conditional_hotkey: ConditionalHotkeyInfo,
}

impl ConditionalHook {
    /// Creates a new instance of `ConditionalHook`.
    pub(super) fn new(
        register: Weak<RefCell<Register>>,
        conditional_hotkey: ConditionalHotkeyInfo,
    ) -> Self {
        Self {
            register,
            conditional_hotkey,
        }
    }
}

impl SelectHandleTarget for ConditionalHook {
    fn bind(&self, button: impl Into<ButtonSet>) -> ButtonEventHandlerEntry {
        ButtonEventHandlerEntry::new(
            Weak::clone(&self.register),
            self.conditional_hotkey
                .clone()
                .build_partial_hotkey_info(button.into()),
        )
    }

    fn bind_mouse_wheel(&self) -> MouseWheelHotkeyEntry {
        MouseWheelHotkeyEntry::new(Weak::clone(&self.register), self.conditional_hotkey.clone())
    }

    fn bind_mouse_cursor(&self) -> MouseCursorHotKeyEntry {
        MouseCursorHotKeyEntry::new(Weak::clone(&self.register), self.conditional_hotkey.clone())
    }

    fn add_modifiers(&self, (pressed, released): (&[Button], &[Button])) -> ConditionalHook {
        ConditionalHook::new(
            Weak::clone(&self.register),
            ConditionalHotkeyInfo {
                modifier: Arc::new(self.conditional_hotkey.modifier.add(pressed, released)),
                ..self.conditional_hotkey.clone()
            },
        )
    }
}

impl SetEventBlock for ConditionalHook {
    fn block(&self) -> Self {
        ConditionalHook::new(
            Weak::clone(&self.register),
            ConditionalHotkeyInfo {
                event_block: EventBlock::Block,
                ..self.conditional_hotkey.clone()
            },
        )
    }

    fn unblock(&self) -> Self {
        ConditionalHook::new(
            Weak::clone(&self.register),
            ConditionalHotkeyInfo {
                event_block: EventBlock::Unblock,
                ..self.conditional_hotkey.clone()
            },
        )
    }
}
