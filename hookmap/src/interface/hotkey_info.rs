use crate::hotkey::{Action, HotkeyAction, HotkeyInfo, ModifierKeys, MouseEventHandler, Trigger};
use hookmap_core::EventBlock;
use std::sync::Arc;

#[derive(Clone, Debug, Default)]
pub(super) struct ConditionalHotkeyInfo {
    pub(super) modifier_keys: Arc<ModifierKeys>,
    pub(super) event_block: EventBlock,
}

impl ConditionalHotkeyInfo {
    pub(super) fn build_partial_hotkey_info(self, trigger: Trigger) -> PartialHotkeyInfo {
        PartialHotkeyInfo {
            trigger,
            modifier: self.modifier_keys,
            event_block: self.event_block,
        }
    }

    pub(super) fn build_mouse_event_handler<E>(self, action: Action<E>) -> MouseEventHandler<E> {
        MouseEventHandler {
            modifier_keys: self.modifier_keys,
            event_block: self.event_block,
            action,
        }
    }
}

#[derive(Clone, Debug)]
pub(super) struct PartialHotkeyInfo {
    pub(super) trigger: Trigger,
    pub(super) modifier: Arc<ModifierKeys>,
    pub(super) event_block: EventBlock,
}

impl PartialHotkeyInfo {
    pub(super) fn build_hotkey_info(self, action: HotkeyAction) -> HotkeyInfo {
        HotkeyInfo {
            trigger: self.trigger,
            modifier_keys: self.modifier,
            event_block: self.event_block,
            action,
        }
    }
}
