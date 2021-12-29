use super::hook::{
    ButtonHook, ExecutableHook, HotkeyOnPressHook, HotkeyOnReleaseHook, MouseHook,
    RemapOnPressHook, RemapOnReleaseHook,
};
use crate::hook::HookStorage;
use hookmap_core::{Button, ButtonAction, ButtonEvent, MouseCursorEvent, MouseWheelEvent};
use std::collections::HashMap;

pub(super) struct HotkeyStorage {
    remap_on_press: HashMap<Button, RemapOnPressHook>,
    remap_on_release: HashMap<Button, RemapOnReleaseHook>,
    hotkey_on_press: HashMap<Button, Vec<HotkeyOnPressHook>>,
    hotkey_on_release: HashMap<Button, Vec<HotkeyOnReleaseHook>>,
    mouse_cursor: Vec<MouseHook<MouseCursorEvent>>,
    mouse_wheel: Vec<MouseHook<MouseWheelEvent>>,
}

impl HotkeyStorage {
    fn fetch_mouse_hook<E: Clone>(hooks: &[MouseHook<E>]) -> Vec<MouseHook<E>> {
        hooks
            .iter()
            .filter(|hook| hook.is_executable())
            .cloned()
            .collect()
    }
}

impl HookStorage for HotkeyStorage {
    type ButtonHook = ButtonHook;
    type MouseCursorHook = MouseHook<MouseCursorEvent>;
    type MouseWheelHook = MouseHook<MouseWheelEvent>;

    fn fetch_button_hook(&self, event: ButtonEvent) -> Vec<ButtonHook> {
        fn fetch_inner<T, U>(
            event: ButtonEvent,
            remap: &HashMap<Button, T>,
            hotkey: &HashMap<Button, Vec<U>>,
        ) -> Vec<ButtonHook>
        where
            T: ExecutableHook + Into<ButtonHook> + Clone,
            U: ExecutableHook + Into<ButtonHook> + Clone,
        {
            if let Some(remap_hook) = remap.get(&event.target) {
                if remap_hook.is_executable() {
                    return vec![remap_hook.clone().into()];
                }
            }
            hotkey
                .get(&event.target)
                .into_iter()
                .flatten()
                .filter_map(|hook| hook.is_executable().then(|| hook.clone().into()))
                .collect()
        }
        match event.action {
            ButtonAction::Press => fetch_inner(event, &self.remap_on_press, &self.hotkey_on_press),
            ButtonAction::Release => {
                fetch_inner(event, &self.remap_on_release, &self.hotkey_on_release)
            }
        }
    }

    fn fetch_mouse_cursor_hook(&self, _: MouseCursorEvent) -> Vec<MouseHook<MouseCursorEvent>> {
        Self::fetch_mouse_hook(&self.mouse_cursor)
    }

    fn fetch_mouse_wheel_hook(&self, _: MouseWheelEvent) -> Vec<MouseHook<MouseWheelEvent>> {
        Self::fetch_mouse_hook(&self.mouse_wheel)
    }
}
