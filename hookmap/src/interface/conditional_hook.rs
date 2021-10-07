use super::{
    button_event_handler_entry::ButtonEventHandlerEntry,
    hotkey_info::ConditionalHotkeyInfo,
    mouse_event_handler_entry::{MouseCursorHotKeyEntry, MouseWheelHotkeyEntry},
    SelectHandleTarget, SetEventBlock,
};
use crate::{button::ButtonSet, hotkey::Trigger, runtime::Register};
use hookmap_core::EventBlock;
use std::{cell::RefCell, rc::Weak, sync::Arc};

/// A struct for selecting the target of the conditional hook.
///
/// # Example
///
/// ```
/// use hookmap::*;
/// let hotkey = Hotkey::new();
/// let mod_ctrl = hotkey.add_modifiers((&[Button::LCtrl.into()], &[]));
/// mod_ctrl.bind(Button::H).like(Button::LeftArrow);
/// ```
///
pub struct ConditionalHotkey {
    register: Weak<RefCell<Register>>,
    conditional_hotkey: ConditionalHotkeyInfo,
}

impl ConditionalHotkey {
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

impl SelectHandleTarget for ConditionalHotkey {
    fn bind(&self, button: impl Into<ButtonSet>) -> ButtonEventHandlerEntry {
        ButtonEventHandlerEntry::new(
            Weak::clone(&self.register),
            self.conditional_hotkey
                .clone()
                .build_partial_hotkey_info(Trigger::Just(button.into())),
        )
    }

    fn bind_all(&self) -> ButtonEventHandlerEntry {
        ButtonEventHandlerEntry::new(
            Weak::clone(&self.register),
            self.conditional_hotkey
                .clone()
                .build_partial_hotkey_info(Trigger::All),
        )
    }

    fn bind_mouse_wheel(&self) -> MouseWheelHotkeyEntry {
        MouseWheelHotkeyEntry::new(Weak::clone(&self.register), self.conditional_hotkey.clone())
    }

    fn bind_mouse_cursor(&self) -> MouseCursorHotKeyEntry {
        MouseCursorHotKeyEntry::new(Weak::clone(&self.register), self.conditional_hotkey.clone())
    }

    fn add_modifiers(
        &self,
        (pressed, released): (&[ButtonSet], &[ButtonSet]),
    ) -> ConditionalHotkey {
        ConditionalHotkey::new(
            Weak::clone(&self.register),
            ConditionalHotkeyInfo {
                modifier_keys: Arc::new(
                    self.conditional_hotkey.modifier_keys.add(pressed, released),
                ),
                ..self.conditional_hotkey.clone()
            },
        )
    }
}

impl SetEventBlock for ConditionalHotkey {
    fn block(&self) -> Self {
        ConditionalHotkey::new(
            Weak::clone(&self.register),
            ConditionalHotkeyInfo {
                event_block: EventBlock::Block,
                ..self.conditional_hotkey.clone()
            },
        )
    }

    fn unblock(&self) -> Self {
        ConditionalHotkey::new(
            Weak::clone(&self.register),
            ConditionalHotkeyInfo {
                event_block: EventBlock::Unblock,
                ..self.conditional_hotkey.clone()
            },
        )
    }
}
