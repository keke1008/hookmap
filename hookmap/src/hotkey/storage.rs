use hookmap_core::button::{Button, ButtonAction};
use hookmap_core::event::{ButtonEvent, CursorEvent, WheelEvent};

use super::hook::{ButtonHook, HotkeyHook, MouseHook, RemapHook};
use crate::hook::HookStorage;
use std::{collections::HashMap, sync::Arc};

#[derive(Default)]
pub(super) struct HotkeyStorage {
    remap: HashMap<Button, Vec<Arc<RemapHook>>>,
    hotkey_on_press: HashMap<Button, Vec<Arc<HotkeyHook>>>,
    hotkey_on_release: HashMap<Button, Vec<Arc<HotkeyHook>>>,
    mouse_cursor: Vec<Arc<MouseHook<CursorEvent>>>,
    mouse_wheel: Vec<Arc<MouseHook<WheelEvent>>>,
}

impl HotkeyStorage {
    fn fetch_mouse_hook<E>(hooks: &[Arc<MouseHook<E>>]) -> Vec<Arc<MouseHook<E>>> {
        hooks
            .iter()
            .filter(|hook| hook.is_executable())
            .map(|hook| Arc::clone(hook))
            .collect()
    }

    pub(super) fn register_remap(&mut self, target: Button, hook: Arc<RemapHook>) {
        self.remap.entry(target).or_default().push(hook);
    }

    pub(super) fn register_hotkey_on_press(&mut self, target: Button, hook: Arc<HotkeyHook>) {
        self.hotkey_on_press.entry(target).or_default().push(hook);
    }

    pub(super) fn register_hotkey_on_release(&mut self, target: Button, hook: Arc<HotkeyHook>) {
        self.hotkey_on_release.entry(target).or_default().push(hook);
    }

    pub(super) fn register_mouse_cursor_hotkey(&mut self, hook: Arc<MouseHook<CursorEvent>>) {
        self.mouse_cursor.push(hook);
    }

    pub(super) fn register_mouse_wheel_hotkey(&mut self, hook: Arc<MouseHook<WheelEvent>>) {
        self.mouse_wheel.push(hook);
    }
}

impl HookStorage for HotkeyStorage {
    type ButtonHook = ButtonHook;
    type MouseCursorHook = Arc<MouseHook<CursorEvent>>;
    type MouseWheelHook = Arc<MouseHook<WheelEvent>>;

    fn fetch_button_hook(&self, event: ButtonEvent) -> Vec<ButtonHook> {
        fn fetch_inner(
            event: ButtonEvent,
            remap: &HashMap<Button, Vec<Arc<RemapHook>>>,
            hotkey: &HashMap<Button, Vec<Arc<HotkeyHook>>>,
        ) -> Vec<ButtonHook> {
            let remap_hook = remap
                .get(&event.target)
                .into_iter()
                .flatten()
                .find(|hook| hook.is_executable())
                .map(|hook| ButtonHook::from(Arc::clone(hook)));
            if let Some(hook) = remap_hook {
                return vec![hook];
            }
            hotkey
                .get(&event.target)
                .into_iter()
                .flatten()
                .filter(|hook| hook.is_executable())
                .map(|hook| ButtonHook::from(Arc::clone(hook)))
                .collect()
        }
        match event.action {
            ButtonAction::Press => fetch_inner(event, &self.remap, &self.hotkey_on_press),
            ButtonAction::Release => fetch_inner(event, &self.remap, &self.hotkey_on_release),
        }
    }

    fn fetch_mouse_cursor_hook(&self, _: CursorEvent) -> Vec<Arc<MouseHook<CursorEvent>>> {
        Self::fetch_mouse_hook(&self.mouse_cursor)
    }

    fn fetch_mouse_wheel_hook(&self, _: WheelEvent) -> Vec<Arc<MouseHook<WheelEvent>>> {
        Self::fetch_mouse_hook(&self.mouse_wheel)
    }
}
