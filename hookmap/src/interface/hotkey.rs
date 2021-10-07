use super::{
    button_event_handler_entry::ButtonEventHandlerEntry,
    conditional_hook::ConditionalHotkey,
    hotkey_info::{ConditionalHotkeyInfo, PartialHotkeyInfo},
    mouse_event_handler_entry::{MouseCursorHotKeyEntry, MouseWheelHotkeyEntry},
    SelectHandleTarget, SetEventBlock,
};
use crate::button::ButtonSet;
use crate::hotkey::{ModifierKeys, Trigger};
use crate::runtime::{HookInstaller, Register};
use hookmap_core::EventBlock;
use std::{cell::RefCell, fmt::Debug, rc::Rc, sync::Arc};

/// A struct for selecting the target of the hook.
///
/// # Example
///
/// ```
/// use hookmap::*;
/// let hotkey = Hotkey::new();
/// hotkey.bind(Button::A)
///     .on_press(|e| println!("{:?}", e));
/// ```
///
#[derive(Default)]
pub struct Hotkey {
    pub(crate) register: Rc<RefCell<Register>>,
}

impl Hotkey {
    /// Creates a new instance of `Hook`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles input events and blocks the current thread.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use hookmap::*;
    /// let hotkey = Hotkey::new();
    /// hotkey.bind(Button::A).on_press(|_| println!("The A key is pressed"));
    /// hotkey.handle_input(); // Blocking the current thread.
    /// ```
    ///
    pub fn handle_input(self) {
        let hook_installer = HookInstaller::from(self);
        hook_installer.install_hook();
    }
}

impl SelectHandleTarget for Hotkey {
    fn bind(&self, button: impl Into<ButtonSet>) -> ButtonEventHandlerEntry {
        ButtonEventHandlerEntry::new(
            Rc::downgrade(&self.register),
            PartialHotkeyInfo {
                trigger: Trigger::Just(button.into()),
                modifier: Arc::default(),
                event_block: EventBlock::default(),
            },
        )
    }

    fn bind_all(&self) -> ButtonEventHandlerEntry {
        ButtonEventHandlerEntry::new(
            Rc::downgrade(&self.register),
            PartialHotkeyInfo {
                trigger: Trigger::All,
                modifier: Arc::default(),
                event_block: EventBlock::default(),
            },
        )
    }

    fn bind_mouse_wheel(&self) -> MouseWheelHotkeyEntry {
        MouseWheelHotkeyEntry::new(
            Rc::downgrade(&self.register),
            ConditionalHotkeyInfo::default(),
        )
    }

    fn bind_mouse_cursor(&self) -> MouseCursorHotKeyEntry {
        MouseCursorHotKeyEntry::new(
            Rc::downgrade(&self.register),
            ConditionalHotkeyInfo::default(),
        )
    }

    fn add_modifiers(
        &self,
        (pressed, released): (&[ButtonSet], &[ButtonSet]),
    ) -> ConditionalHotkey {
        ConditionalHotkey::new(
            Rc::downgrade(&self.register),
            ConditionalHotkeyInfo {
                modifier_keys: Arc::new(ModifierKeys::new(pressed, released)),
                ..ConditionalHotkeyInfo::default()
            },
        )
    }
}

impl SetEventBlock for Hotkey {
    fn block(&self) -> ConditionalHotkey {
        ConditionalHotkey::new(
            Rc::downgrade(&self.register),
            ConditionalHotkeyInfo {
                event_block: EventBlock::Block,
                ..ConditionalHotkeyInfo::default()
            },
        )
    }

    fn unblock(&self) -> ConditionalHotkey {
        ConditionalHotkey::new(
            Rc::downgrade(&self.register),
            ConditionalHotkeyInfo {
                event_block: EventBlock::Unblock,
                ..ConditionalHotkeyInfo::default()
            },
        )
    }
}

impl Debug for Hotkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Hook")
            .field("event_block", &EventBlock::default())
            .finish()
    }
}
