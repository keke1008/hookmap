mod hook;
mod modifier_keys;
mod storage;

use hook::{HotkeyHook, MouseHook, RemapHook};
use modifier_keys::ModifierKeys;
use storage::HotkeyStorage;

use crate::button::ButtonSet;
use hookmap_core::{Button, ButtonEvent, MouseCursorEvent, MouseWheelEvent, NativeEventOperation};
use std::{cell::RefCell, sync::Arc};

pub trait RegisterHotkey {
    fn remap(&mut self, target: impl Into<ButtonSet>, behavior: impl Into<ButtonSet>);
    fn on_press(&mut self, target: impl Into<ButtonSet>, process: impl Fn(ButtonEvent) + 'static);
    fn on_release(&mut self, target: impl Into<ButtonSet>, process: impl Fn(ButtonEvent) + 'static);
    fn mouse_wheel(&mut self, process: impl Fn(MouseWheelEvent) + 'static);
    fn mouse_cursor(&mut self, process: impl Fn(MouseCursorEvent) + 'static);
}

fn register_each_target(
    target: ButtonSet,
    modifier_keys: &ModifierKeys,
    mut f: impl FnMut(Button, ModifierKeys),
) {
    if let ButtonSet::All(keys) = &target {
        for (index, &key) in keys.iter().enumerate() {
            let mut keys = keys.clone();
            keys.remove(index);
            let mut modifier_keys = modifier_keys.clone();
            modifier_keys.pressed.push(ButtonSet::All(keys));
            f(key, modifier_keys);
        }
        return;
    }
    for &key in target.iter() {
        f(key, modifier_keys.clone());
    }
}

fn register_button_hotkey(
    target: ButtonSet,
    register: fn(&mut HotkeyStorage, Button, HotkeyHook),
    storage: &RefCell<HotkeyStorage>,
    modifier_keys: &ModifierKeys,
    process: Arc<dyn Fn(ButtonEvent)>,
    native_event_operation: NativeEventOperation,
) {
    register_each_target(target, &modifier_keys, move |key, modifier_keys| {
        let hook = HotkeyHook::new(modifier_keys, Arc::clone(&process), native_event_operation);
        register(&mut storage.borrow_mut(), key, hook);
    });
}

#[derive(Default)]
pub struct Hotkey {
    storage: RefCell<HotkeyStorage>,
}

impl Hotkey {
    pub fn new() -> Self {
        Hotkey::default()
    }
}

impl RegisterHotkey for Hotkey {
    fn remap(&mut self, target: impl Into<ButtonSet>, behavior: impl Into<ButtonSet>) {
        let behavior = behavior.into();
        register_each_target(
            target.into(),
            &ModifierKeys::default(),
            |key, modifier_keys| {
                let hook = RemapHook::new(modifier_keys, behavior.clone());
                self.storage.borrow_mut().register_remap(key, hook);
            },
        );
    }

    fn on_press(&mut self, target: impl Into<ButtonSet>, process: impl Fn(ButtonEvent) + 'static) {
        register_button_hotkey(
            target.into(),
            HotkeyStorage::register_hotkey_on_press,
            &self.storage,
            &ModifierKeys::default(),
            Arc::new(process),
            NativeEventOperation::default(),
        );
    }

    fn on_release(
        &mut self,
        target: impl Into<ButtonSet>,
        process: impl Fn(ButtonEvent) + 'static,
    ) {
        register_button_hotkey(
            target.into(),
            HotkeyStorage::register_hotkey_on_release,
            &self.storage,
            &ModifierKeys::default(),
            Arc::new(process),
            NativeEventOperation::default(),
        );
    }

    fn mouse_wheel(&mut self, process: impl Fn(MouseWheelEvent) + 'static) {
        let hook = MouseHook::new(
            ModifierKeys::default(),
            Arc::new(process),
            NativeEventOperation::default(),
        );
        self.storage.borrow_mut().register_mouse_wheel_hotkey(hook);
    }

    fn mouse_cursor(&mut self, process: impl Fn(MouseCursorEvent) + 'static) {
        let hook = MouseHook::new(
            ModifierKeys::default(),
            Arc::new(process),
            NativeEventOperation::default(),
        );
        self.storage.borrow_mut().register_mouse_cursor_hotkey(hook);
    }
}
