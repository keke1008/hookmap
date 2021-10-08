use super::{
    button_event_handler_entry::ButtonEventHandlerEntry,
    mouse_event_handler_entry::{MouseCursorHotKeyEntry, MouseWheelHotkeyEntry},
    remap_entry::RemapEntry,
    SelectHandleTarget, SetEventBlock,
};
use crate::{
    button::ButtonSet,
    hotkey::{ModifierKeys, Trigger},
    runtime::Register,
};
use hookmap_core::EventBlock;
use std::{cell::RefCell, rc::Weak, sync::Arc};
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
