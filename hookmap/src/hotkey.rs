mod hook;
mod modifier_keys;
mod storage;

use hook::{HookProcess, HotkeyHook, MouseHook, RemapHook};
pub use modifier_keys::ModifierKeys;
use storage::HotkeyStorage;

use crate::{button::ButtonSet, runtime::Runtime};
use hookmap_core::{Button, ButtonEvent, MouseCursorEvent, MouseWheelEvent, NativeEventOperation};
use std::{cell::RefCell, sync::Arc};

pub trait RegisterHotkey {
    fn remap(&mut self, target: impl Into<ButtonSet>, behavior: impl Into<ButtonSet>);
    fn on_press(&mut self, target: impl Into<ButtonSet>, process: HookProcess<ButtonEvent>);
    fn on_release(&mut self, target: impl Into<ButtonSet>, process: HookProcess<ButtonEvent>);
    fn mouse_wheel(&mut self, process: HookProcess<MouseWheelEvent>);
    fn mouse_cursor(&mut self, process: HookProcess<MouseCursorEvent>);
    fn disable(&mut self, target: impl Into<ButtonSet>);
    fn add_modifier_keys(&mut self, modifier_keys: &ModifierKeys) -> ModifierHotkey;
    fn change_native_event_operation(&mut self, operation: NativeEventOperation) -> ModifierHotkey;
}

fn register_each_target(
    target: ButtonSet,
    modifier_keys: &Arc<ModifierKeys>,
    mut f: impl FnMut(Button, Arc<ModifierKeys>),
) {
    if let ButtonSet::All(keys) = &target {
        for (index, &key) in keys.iter().enumerate() {
            let mut keys = keys.clone();
            keys.remove(index);
            let mut modifier_keys = (**modifier_keys).clone();
            modifier_keys.pressed.push(ButtonSet::All(keys));
            f(key, Arc::new(modifier_keys));
        }
        return;
    }
    for &key in target.iter() {
        f(key, Arc::clone(modifier_keys));
    }
}

fn register_button_hotkey(
    target: ButtonSet,
    register: fn(&mut HotkeyStorage, Button, HotkeyHook),
    storage: &RefCell<HotkeyStorage>,
    modifier_keys: &Arc<ModifierKeys>,
    process: HookProcess<ButtonEvent>,
    native_event_operation: NativeEventOperation,
) {
    register_each_target(target, modifier_keys, move |key, modifier_keys| {
        let hook = HotkeyHook::new(modifier_keys, Arc::clone(&process), native_event_operation);
        register(&mut storage.borrow_mut(), key, hook);
    });
}

#[derive(Default)]
pub struct Hotkey {
    storage: RefCell<HotkeyStorage>,
    modifier_keys: Arc<ModifierKeys>,
}

impl Hotkey {
    pub fn new() -> Self {
        Hotkey::default()
    }

    pub fn install(self) {
        let runtime = Runtime::new(self.storage.into_inner());
        runtime.start();
    }
}

impl RegisterHotkey for Hotkey {
    fn remap(&mut self, target: impl Into<ButtonSet>, behavior: impl Into<ButtonSet>) {
        let behavior = behavior.into();
        register_each_target(target.into(), &self.modifier_keys, |key, modifier_keys| {
            let hook = RemapHook::new(modifier_keys, behavior.clone());
            self.storage.borrow_mut().register_remap(key, hook);
        });
    }

    fn on_press(&mut self, target: impl Into<ButtonSet>, process: HookProcess<ButtonEvent>) {
        register_button_hotkey(
            target.into(),
            HotkeyStorage::register_hotkey_on_press,
            &self.storage,
            &self.modifier_keys,
            process,
            NativeEventOperation::default(),
        );
    }

    fn on_release(&mut self, target: impl Into<ButtonSet>, process: HookProcess<ButtonEvent>) {
        register_button_hotkey(
            target.into(),
            HotkeyStorage::register_hotkey_on_release,
            &self.storage,
            &self.modifier_keys,
            process,
            NativeEventOperation::default(),
        );
    }

    fn mouse_wheel(&mut self, process: HookProcess<MouseWheelEvent>) {
        let hook = MouseHook::new(
            Arc::clone(&self.modifier_keys),
            process,
            NativeEventOperation::default(),
        );
        self.storage.borrow_mut().register_mouse_wheel_hotkey(hook);
    }

    fn mouse_cursor(&mut self, process: HookProcess<MouseCursorEvent>) {
        let hook = MouseHook::new(
            Arc::clone(&self.modifier_keys),
            process,
            NativeEventOperation::default(),
        );
        self.storage.borrow_mut().register_mouse_cursor_hotkey(hook);
    }

    fn disable(&mut self, target: impl Into<ButtonSet>) {
        let process = Arc::new(|_| {}) as Arc<dyn Fn(_) + Send + Sync>;
        let target = target.into();
        self.on_press(target.clone(), Arc::clone(&process));
        self.on_press(target, process);
    }

    fn add_modifier_keys(&mut self, modifier_keys: &ModifierKeys) -> ModifierHotkey {
        ModifierHotkey::new(
            &self.storage,
            Arc::new(modifier_keys.to_owned()),
            NativeEventOperation::default(),
        )
    }

    fn change_native_event_operation(&mut self, operation: NativeEventOperation) -> ModifierHotkey {
        ModifierHotkey::new(&self.storage, Arc::clone(&self.modifier_keys), operation)
    }
}

pub struct ModifierHotkey<'a> {
    storage: &'a RefCell<HotkeyStorage>,
    modifier_keys: Arc<ModifierKeys>,
    native_event_operation: NativeEventOperation,
}

impl<'a> ModifierHotkey<'a> {
    fn new(
        storage: &'a RefCell<HotkeyStorage>,
        modifier_keys: Arc<ModifierKeys>,
        native_event_operation: NativeEventOperation,
    ) -> Self {
        ModifierHotkey {
            storage,
            modifier_keys,
            native_event_operation,
        }
    }
}

impl RegisterHotkey for ModifierHotkey<'_> {
    fn remap(&mut self, target: impl Into<ButtonSet>, behavior: impl Into<ButtonSet>) {
        let behavior = behavior.into();
        register_each_target(target.into(), &self.modifier_keys, |key, modifier_keys| {
            let hook = RemapHook::new(modifier_keys, behavior.clone());
            self.storage.borrow_mut().register_remap(key, hook);
        });
    }

    fn on_press(&mut self, target: impl Into<ButtonSet>, process: HookProcess<ButtonEvent>) {
        register_button_hotkey(
            target.into(),
            HotkeyStorage::register_hotkey_on_press,
            self.storage,
            &self.modifier_keys,
            process,
            self.native_event_operation,
        );
    }

    fn on_release(&mut self, target: impl Into<ButtonSet>, process: HookProcess<ButtonEvent>) {
        register_button_hotkey(
            target.into(),
            HotkeyStorage::register_hotkey_on_release,
            self.storage,
            &self.modifier_keys,
            process,
            self.native_event_operation,
        );
    }

    fn mouse_wheel(&mut self, process: HookProcess<MouseWheelEvent>) {
        let hook = MouseHook::new(
            Arc::clone(&self.modifier_keys),
            process,
            self.native_event_operation,
        );
        self.storage.borrow_mut().register_mouse_wheel_hotkey(hook);
    }

    fn mouse_cursor(&mut self, process: HookProcess<MouseCursorEvent>) {
        let hook = MouseHook::new(
            Arc::clone(&self.modifier_keys),
            process,
            self.native_event_operation,
        );
        self.storage.borrow_mut().register_mouse_cursor_hotkey(hook);
    }

    fn disable(&mut self, target: impl Into<ButtonSet>) {
        let process = Arc::new(|_| {}) as Arc<dyn Fn(_) + Send + Sync>;
        let target = target.into();
        self.on_press(target.clone(), Arc::clone(&process));
        self.on_press(target, process);
    }

    fn add_modifier_keys(&mut self, modifier_keys: &ModifierKeys) -> ModifierHotkey {
        ModifierHotkey::new(
            self.storage,
            Arc::new(self.modifier_keys.merge(modifier_keys)),
            self.native_event_operation,
        )
    }

    fn change_native_event_operation(&mut self, operation: NativeEventOperation) -> ModifierHotkey {
        ModifierHotkey::new(self.storage, Arc::clone(&self.modifier_keys), operation)
    }
}
