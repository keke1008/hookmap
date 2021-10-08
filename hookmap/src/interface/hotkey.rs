use super::{
    entry::{ButtonEventHandlerEntry, MouseCursorHotKeyEntry, MouseWheelHotkeyEntry, RemapEntry},
    SelectHandleTarget, SetEventBlock,
};
use crate::button::ButtonSet;
use crate::hotkey::{ModifierKeys, Trigger};
use crate::runtime::{HookInstaller, Register};
use hookmap_core::EventBlock;
use std::{
    cell::RefCell,
    fmt::Debug,
    rc::{Rc, Weak},
    sync::Arc,
};
use typed_builder::TypedBuilder;

/// A struct for selecting the target of the conditional hook.
///
/// # Example
///
/// ```
/// use hookmap::*;
/// let hotkey = Hotkey::new();
/// let mod_ctrl = hotkey.add_modifiers((&[Button::LCtrl.into()], &[]));
/// mod_ctrl.remap(Button::H).to(Button::LeftArrow);
/// ```
///
#[derive(TypedBuilder)]
pub struct ConditionalHotkey {
    register: Weak<RefCell<Register>>,

    #[builder(default)]
    modifier_keys: Arc<ModifierKeys>,

    #[builder(default)]
    event_block: EventBlock,
}

impl SelectHandleTarget for ConditionalHotkey {
    fn bind(&self, trigger: impl Into<ButtonSet>) -> ButtonEventHandlerEntry {
        ButtonEventHandlerEntry::builder()
            .register(Weak::clone(&self.register))
            .trigger(Trigger::Just(trigger.into()))
            .modifier_keys(Arc::clone(&self.modifier_keys))
            .event_block(self.event_block)
            .build()
    }

    fn bind_all(&self) -> ButtonEventHandlerEntry {
        ButtonEventHandlerEntry::builder()
            .register(Weak::clone(&self.register))
            .trigger(Trigger::All)
            .modifier_keys(Arc::clone(&self.modifier_keys))
            .event_block(self.event_block)
            .build()
    }

    fn remap(&self, target: impl Into<ButtonSet>) -> RemapEntry {
        RemapEntry::builder()
            .register(Weak::clone(&self.register))
            .trigger(target.into())
            .modifier_keys(Arc::clone(&self.modifier_keys))
            .build()
    }

    fn bind_mouse_wheel(&self) -> MouseWheelHotkeyEntry {
        MouseWheelHotkeyEntry::builder()
            .register(Weak::clone(&self.register))
            .modifier_keys(Arc::clone(&self.modifier_keys))
            .event_block(self.event_block)
            .build()
    }

    fn bind_mouse_cursor(&self) -> MouseCursorHotKeyEntry {
        MouseCursorHotKeyEntry::builder()
            .register(Weak::clone(&self.register))
            .modifier_keys(Arc::clone(&self.modifier_keys))
            .event_block(self.event_block)
            .build()
    }

    fn add_modifiers(
        &self,
        (pressed, released): (&[ButtonSet], &[ButtonSet]),
    ) -> ConditionalHotkey {
        ConditionalHotkey::builder()
            .register(Weak::clone(&self.register))
            .modifier_keys(Arc::new(self.modifier_keys.add(pressed, released)))
            .event_block(self.event_block)
            .build()
    }
}

impl SetEventBlock for ConditionalHotkey {
    fn block(&self) -> Self {
        ConditionalHotkey::builder()
            .register(Weak::clone(&self.register))
            .modifier_keys(Arc::clone(&self.modifier_keys))
            .event_block(EventBlock::Block)
            .build()
    }

    fn unblock(&self) -> Self {
        ConditionalHotkey::builder()
            .register(Weak::clone(&self.register))
            .modifier_keys(Arc::clone(&self.modifier_keys))
            .event_block(EventBlock::Unblock)
            .build()
    }
}

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
