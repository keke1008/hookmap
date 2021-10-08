use super::storage::{ButtonStorage, HookInfo, HookKind, Remap, Storage};
use crate::button::ButtonSet;
use crate::hotkey::{HotkeyAction, HotkeyInfo, MouseEventHandler, RemapInfo, Trigger};
use hookmap_core::{MouseCursorEvent, MouseWheelEvent};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

#[derive(Debug, Default)]
struct HotkeyConverter(Option<Arc<AtomicBool>>);

impl HotkeyConverter {
    fn activated(&mut self) -> Arc<AtomicBool> {
        match self.0 {
            Some(ref activated) => Arc::clone(activated),
            None => {
                let activated = Arc::default();
                self.0 = Some(Arc::clone(&activated));
                activated
            }
        }
    }

    fn crate_pressed_hook(&mut self, hotkey: &HotkeyInfo) -> Option<HookInfo> {
        let modifier_keys = Arc::clone(&hotkey.modifier_keys);
        match &hotkey.action {
            HotkeyAction::Release(_) => None,
            HotkeyAction::Press(action) => {
                let kind = HookKind::Independet { modifier_keys };
                let hook = HookInfo::new(kind, action.clone(), hotkey.event_block);
                Some(hook)
            }
            HotkeyAction::PressOrRelease(action) => {
                let kind = HookKind::LinkedOnPress {
                    modifier_keys,
                    activated: self.activated(),
                };
                let hook_on_press = HookInfo::new(kind, action.clone(), hotkey.event_block);
                Some(hook_on_press)
            }
            HotkeyAction::PressAndRelease {
                on_press: press, ..
            } => {
                let kind = HookKind::LinkedOnPress {
                    modifier_keys,
                    activated: self.activated(),
                };
                let hook_on_press = HookInfo::new(kind, press.clone(), hotkey.event_block);
                Some(hook_on_press)
            }
        }
    }

    fn create_released_function(&mut self, hotkey: &HotkeyInfo) -> Option<HookInfo> {
        match &hotkey.action {
            HotkeyAction::Press(_) => None,
            HotkeyAction::Release(action) => {
                let modifier_keys = Arc::clone(&hotkey.modifier_keys);
                let kind = HookKind::Independet { modifier_keys };
                let hook = HookInfo::new(kind, action.clone(), hotkey.event_block);
                Some(hook)
            }
            HotkeyAction::PressOrRelease(action) => {
                let kind = HookKind::LinkedOnRelease {
                    activated: self.activated(),
                };
                let hook_on_release = HookInfo::new(kind, action.clone(), hotkey.event_block);
                Some(hook_on_release)
            }
            HotkeyAction::PressAndRelease {
                on_release: release,
                ..
            } => {
                let kind = HookKind::LinkedOnRelease {
                    activated: self.activated(),
                };
                let hook_on_release = HookInfo::new(kind, release.clone(), hotkey.event_block);
                Some(hook_on_release)
            }
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

    fn register_hotkey_inner(
        trigger: &Trigger,
        hook: Option<HookInfo>,
        storage: &mut ButtonStorage,
    ) {
        if let Some(hook) = hook {
            match trigger {
                Trigger::Just(trigger) => {
                    for &trigger in trigger.iter() {
                        storage.just.entry(trigger).or_default().push(hook.clone());
                    }
                }
                Trigger::All => storage.all.push(hook),
            };
        }
    }

    pub(crate) fn register_hotkey(&mut self, mut hotkey: HotkeyInfo) {
        let hotkey = {
            if let Trigger::Just(ref trigger @ ButtonSet::All(_)) = hotkey.trigger {
                hotkey.modifier_keys = Arc::new(hotkey.modifier_keys.add(&[trigger.clone()], &[]))
            }
            hotkey
        };
        let mut converter = HotkeyConverter::default();
        Self::register_hotkey_inner(
            &hotkey.trigger,
            converter.crate_pressed_hook(&hotkey),
            &mut self.storage.on_press,
        );
        Self::register_hotkey_inner(
            &hotkey.trigger,
            converter.create_released_function(&hotkey),
            &mut self.storage.on_release,
        );
    }

    pub(crate) fn register_remap(&mut self, mut remap_info: RemapInfo) {
        let remap_info = {
            if let trigger @ ButtonSet::All(_) = &remap_info.trigger {
                remap_info.modifier_keys =
                    Arc::new(remap_info.modifier_keys.add(&[trigger.clone()], &[]))
            }
            remap_info
        };
        let remap = Remap {
            modifier_keys: remap_info.modifier_keys,
            target: remap_info.target,
            activated: Arc::default(),
        };
        for trigger in remap_info.trigger.iter() {
            self.storage
                .remap
                .entry(*trigger)
                .or_default()
                .push(remap.clone());
        }
    }

    pub(crate) fn register_cursor_event_handler(
        &mut self,
        handler: MouseEventHandler<MouseCursorEvent>,
    ) {
        self.storage.mouse_cursor.push(handler);
    }

    pub(crate) fn register_wheel_event_event_handler(
        &mut self,
        handler: MouseEventHandler<MouseWheelEvent>,
    ) {
        self.storage.mouse_wheel.push(handler);
    }
}
