use super::{
    button_event_handler_entry::ButtonEventHandlerEntry,
    mouse_event_handler_entry::{MouseCursorHotKeyEntry, MouseWheelHotkeyEntry},
    SelectHandleTarget, SetEventBlock,
};
use crate::hotkey::PartialHotkeyUsedHook;
use crate::runtime::Register;
use hookmap_core::{Button, EventBlock};
use std::rc::{Rc, Weak};

/// A struct for selecting the target of the conditional hook.
///
/// # Example
///
/// ```
/// use hookmap::*;
/// let hook = Hook::new();
/// let mod_ctrl = hook.cond(ConditionUnit::Pressed(Button::LCtrl));
/// mod_ctrl.bind(Button::H).like(Button::LeftArrow);
/// ```
///
pub struct ConditionalHook {
    register: Weak<Register>,
    partial_hotkey: PartialHotkeyUsedHook,
}

impl ConditionalHook {
    /// Creates a new instance of `ConditionalHook`.
    pub(crate) fn new(register: Weak<Register>, partial_hotkey: PartialHotkeyUsedHook) -> Self {
        Self {
            register,
            partial_hotkey,
        }
    }
}

impl SelectHandleTarget for ConditionalHook {
    fn bind(&self, button: Button) -> ButtonEventHandlerEntry {
        ButtonEventHandlerEntry::new(
            Rc::downgrade(&self.register.upgrade().unwrap()),
            self.partial_hotkey
                .clone()
                .build_partial_hotkey_used_entry(button),
        )
    }

    fn bind_mouse_wheel(&self) -> MouseWheelHotkeyEntry {
        MouseWheelHotkeyEntry::new(
            Rc::downgrade(&self.register.upgrade().unwrap()),
            self.partial_hotkey.clone(),
        )
    }

    fn bind_mouse_cursor(&self) -> MouseCursorHotKeyEntry {
        MouseCursorHotKeyEntry::new(
            Rc::downgrade(&self.register.upgrade().unwrap()),
            self.partial_hotkey.clone(),
        )
    }
}

impl SetEventBlock for ConditionalHook {
    fn block(&self) -> Self {
        ConditionalHook::new(
            Weak::clone(&self.register),
            PartialHotkeyUsedHook {
                event_block: EventBlock::Block,
                ..self.partial_hotkey.clone()
            },
        )
    }

    fn unblock(&self) -> Self {
        ConditionalHook::new(
            Weak::clone(&self.register),
            PartialHotkeyUsedHook {
                event_block: EventBlock::Unblock,
                ..self.partial_hotkey.clone()
            },
        )
    }
}
