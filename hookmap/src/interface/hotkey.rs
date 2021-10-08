use super::{
    button_event_handler_entry::ButtonEventHandlerEntry,
    conditional_hook::ConditionalHotkey,
    mouse_event_handler_entry::{MouseCursorHotKeyEntry, MouseWheelHotkeyEntry},
    remap_entry::RemapEntry,
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
    fn bind(&self, trigger: impl Into<ButtonSet>) -> ButtonEventHandlerEntry {
        ButtonEventHandlerEntry::builder()
            .register(Rc::downgrade(&self.register))
            .trigger(Trigger::Just(trigger.into()))
            .build()
    }

    fn bind_all(&self) -> ButtonEventHandlerEntry {
        ButtonEventHandlerEntry::builder()
            .register(Rc::downgrade(&self.register))
            .trigger(Trigger::All)
            .build()
    }

    fn remap(&self, trigger: impl Into<ButtonSet>) -> RemapEntry {
        RemapEntry::builder()
            .register(Rc::downgrade(&self.register))
            .trigger(trigger.into())
            .build()
    }

    fn bind_mouse_wheel(&self) -> MouseWheelHotkeyEntry {
        MouseWheelHotkeyEntry::builder()
            .register(Rc::downgrade(&self.register))
            .build()
    }

    fn bind_mouse_cursor(&self) -> MouseCursorHotKeyEntry {
        MouseCursorHotKeyEntry::builder()
            .register(Rc::downgrade(&self.register))
            .build()
    }

    fn add_modifiers(
        &self,
        (pressed, released): (&[ButtonSet], &[ButtonSet]),
    ) -> ConditionalHotkey {
        ConditionalHotkey::builder()
            .register(Rc::downgrade(&self.register))
            .modifier_keys(Arc::new(ModifierKeys::new(pressed, released)))
            .build()
    }
}

impl SetEventBlock for Hotkey {
    fn block(&self) -> ConditionalHotkey {
        ConditionalHotkey::builder()
            .register(Rc::downgrade(&self.register))
            .event_block(EventBlock::Block)
            .build()
    }

    fn unblock(&self) -> ConditionalHotkey {
        ConditionalHotkey::builder()
            .register(Rc::downgrade(&self.register))
            .event_block(EventBlock::Unblock)
            .build()
    }
}

impl Debug for Hotkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Hook")
            .field("event_block", &EventBlock::default())
            .finish()
    }
}
