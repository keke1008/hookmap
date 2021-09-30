use super::storage::{HookInfo, HookKind, Storage};
use crate::button::ButtonSet;
use crate::hotkey::{HotkeyAction, HotkeyInfo, MouseEventHandler};
use hookmap_core::{MouseCursorEvent, MouseWheelEvent};
use std::sync::Arc;

fn hotkey_to_hook(hotkey: &HotkeyInfo) -> (Option<HookInfo>, Option<HookInfo>) {
    let modifier_keys = Arc::clone(&hotkey.modifier);
    match &hotkey.action {
        HotkeyAction::Press(action) => {
            let kind = HookKind::Independet { modifier_keys };
            let hook = HookInfo::new(kind, action.clone(), hotkey.event_block);
            (Some(hook), None)
        }
        HotkeyAction::Release(action) => {
            let kind = HookKind::Independet { modifier_keys };
            let hook = HookInfo::new(kind, action.clone(), hotkey.event_block);
            (None, Some(hook))
        }
        HotkeyAction::PressOrRelease(action) => {
            let activated = Arc::default();
            let kind = HookKind::LinkedOnPress {
                modifier_keys,
                activated: Arc::clone(&activated),
            };
            let hook_on_press = HookInfo::new(kind, action.clone(), hotkey.event_block);
            let kind = HookKind::LinkedOnRelease { activated };
            let hook_on_release = HookInfo::new(kind, action.clone(), hotkey.event_block);
            (Some(hook_on_press), Some(hook_on_release))
        }
        HotkeyAction::PressAndRelease {
            on_press: press,
            on_release: release,
        } => {
            let activated = Arc::default();
            let kind = HookKind::LinkedOnPress {
                modifier_keys,
                activated: Arc::clone(&activated),
            };
            let hook_on_press = HookInfo::new(kind, press.clone(), hotkey.event_block);
            let kind = HookKind::LinkedOnRelease { activated };
            let hook_on_release = HookInfo::new(kind, release.clone(), hotkey.event_block);
            (Some(hook_on_press), Some(hook_on_release))
        }
    }
}

#[derive(Default, Debug)]
pub(crate) struct Register {
    storage: Storage,
}

impl Register {
    pub(super) fn into_inner(self) -> Storage {
        self.storage
    }

    pub(crate) fn register_hotkey(&mut self, mut hotkey: HotkeyInfo) {
        let hotkey = {
            if let ButtonSet::All(_) = hotkey.trigger {
                hotkey.modifier = Arc::new(hotkey.modifier.add(&[hotkey.trigger.clone()], &[]))
            }
            hotkey
        };
        let (on_press_hook, on_release_hook) = hotkey_to_hook(&hotkey);
        if let Some(hook) = on_press_hook {
            let hook = Arc::new(hook);
            for &trigger in hotkey.trigger.iter() {
                self.storage
                    .on_press
                    .entry(trigger)
                    .or_default()
                    .push(Arc::clone(&hook));
            }
        }
        if let Some(hook) = on_release_hook {
            let hook = Arc::new(hook);
            for &trigger in hotkey.trigger.iter() {
                self.storage
                    .on_release
                    .entry(trigger)
                    .or_default()
                    .push(Arc::clone(&hook));
            }
        }
    }

    pub(crate) fn register_cursor_event_handler(
        &mut self,
        handler: MouseEventHandler<MouseCursorEvent>,
    ) {
        self.storage.mouse_cursor.push(Arc::new(handler));
    }

    pub(crate) fn register_wheel_event_event_handler(
        &mut self,
        handler: MouseEventHandler<MouseWheelEvent>,
    ) {
        self.storage.mouse_wheel.push(Arc::new(handler));
    }
}
