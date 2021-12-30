mod hook;
mod modifier_keys;
mod storage;

use hook::{HookProcess, HotkeyHook, MouseHook, RemapHook};
pub use modifier_keys::ModifierKeys;
use storage::HotkeyStorage;

use crate::{button::ButtonSet, runtime::Runtime};
use hookmap_core::{Button, ButtonEvent, MouseCursorEvent, MouseWheelEvent, NativeEventOperation};
use std::{cell::RefCell, sync::Arc};

/// Methods for registering hotkeys.
pub trait RegisterHotkey {
    /// Makes `target` behave like a `behavior`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::{
    ///     hotkey::{Hotkey, RegisterHotkey},
    ///     button::Button,
    /// };
    ///
    /// let hotkey = Hotkey::new();
    /// hotkey.remap(Button::A, Button::B);
    /// ```
    ///
    fn remap(&self, target: impl Into<ButtonSet>, behavior: impl Into<ButtonSet>);

    /// Run `process` when `target` is pressed.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::{
    ///     hotkey::{Hotkey, RegisterHotkey},
    ///     button::Button,
    /// };
    /// use std::sync::Arc;
    ///
    /// let hotkey = Hotkey::new();
    /// hotkey.on_press(Button::A, Arc::new(|e| println!("Pressed: {:?}")));
    /// ```
    ///
    fn on_press(&self, target: impl Into<ButtonSet>, process: HookProcess<ButtonEvent>);

    /// Run `process` when `target` is released.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::{
    ///     hotkey::{Hotkey, RegisterHotkey},
    ///     button::Button,
    /// };
    /// use std::sync::Arc;
    ///
    /// let hotkey = Hotkey::new();
    /// hotkey.on_release(Button::A, Arc::new(|_| println!("Released: {:?}")));
    /// ```
    ///
    fn on_release(&self, target: impl Into<ButtonSet>, process: HookProcess<ButtonEvent>);

    /// Run `process` when a mouse wheel is rotated.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::{hotkey::Hotkey, RegisterHotkey}
    /// use std::sync::Arc;
    ///
    /// let hotkey = Hotkey::new();
    /// hotkey.mouse_wheel(Arc::new(|delta| println!("Delta: {:?}", delta)));
    /// ```
    ///
    fn mouse_wheel(&self, process: HookProcess<MouseWheelEvent>);

    /// Run `process` when a mouse cursor is moved.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::{hotkey::Hotkey, RegisterHotkey};
    /// use std::sync::Arc;
    ///
    /// let hotkey = Hotkey::new();
    /// hotkey.mouse_cursor(Arc::new(|(x, y)| println!("Cursor: ({}, {})", x, y)));
    /// ```
    ///
    fn mouse_cursor(&self, process: HookProcess<MouseCursorEvent>);

    /// Disables the button and blocks events.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::{
    ///     hotkey::{Hotkey, RegisterHotkey},
    ///     button::Button,
    /// };
    /// use std::sync::Arc;
    ///
    /// let hotkey = Hotkey::new();
    /// hotkey.disable(Button::A);
    /// ```
    ///
    fn disable(&self, target: impl Into<ButtonSet>);

    /// Adds modifier keys.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::{
    ///     hotkey::{Hotkey, RegisterHotkey, ModifierKeys},
    ///     button::{Button, ButtonSet},
    /// };
    ///
    /// let hotkey = Hotkey::new();
    /// let modifier_keys = ModifierKeys::new(vec![ButtonSet::Any(vec![Button::A, Button::B])], vec![]);
    /// let a_or_b = hotkey.add_modifier_keys(&modifier_keys);
    /// a_or_b.remap(Button::C, Button::D);
    /// ```
    fn add_modifier_keys(&self, modifier_keys: &ModifierKeys) -> ModifierHotkey;

    /// Changes the operation for native events to block or dispatch.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::{
    ///     hotkey::{Hotkey, RegisterHotkey, ModifierKeys},
    ///     button::Button,
    ///     hook::NativeEventBlock,
    /// };
    ///
    /// let hotkey = Hotkey::new();
    /// let blocking_hotkey = hotkey.change_native_event_operation(NativeEventOperation::Block);
    /// blocking_hotkey.on_press(Button::A, Arc::new(|e| println!("Press: {:?}", e)));
    /// ```
    ///
    fn change_native_event_operation(&self, operation: NativeEventOperation) -> ModifierHotkey;
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

/// Registering Hotkeys.
///
/// # Examples
///
/// ```no_run
/// use hookmap::{hotkey::Hotkey, button::Button};
///
/// let hotkey = Hotkey::new();
/// hotkey.remap(Button::A, Button::B);
/// hotkey.install();
/// ```
///
#[derive(Default)]
pub struct Hotkey {
    storage: RefCell<HotkeyStorage>,
    modifier_keys: Arc<ModifierKeys>,
}

impl Hotkey {
    /// Creates a new insgance of `Hotkey`.
    pub fn new() -> Self {
        Hotkey::default()
    }

    /// Installs registered hotkeys.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use hookmap::hotkey::Hotkey;
    ///
    /// let hotkey = Hotkey::new();
    /// hotkey.install();
    /// ```
    ///
    pub fn install(self) {
        let runtime = Runtime::new(self.storage.into_inner());
        runtime.start();
    }
}

impl RegisterHotkey for Hotkey {
    fn remap(&self, target: impl Into<ButtonSet>, behavior: impl Into<ButtonSet>) {
        let behavior = behavior.into();
        register_each_target(target.into(), &self.modifier_keys, |key, modifier_keys| {
            let hook = RemapHook::new(modifier_keys, behavior.clone());
            self.storage.borrow_mut().register_remap(key, hook);
        });
    }

    fn on_press(&self, target: impl Into<ButtonSet>, process: HookProcess<ButtonEvent>) {
        register_button_hotkey(
            target.into(),
            HotkeyStorage::register_hotkey_on_press,
            &self.storage,
            &self.modifier_keys,
            process,
            NativeEventOperation::default(),
        );
    }

    fn on_release(&self, target: impl Into<ButtonSet>, process: HookProcess<ButtonEvent>) {
        register_button_hotkey(
            target.into(),
            HotkeyStorage::register_hotkey_on_release,
            &self.storage,
            &self.modifier_keys,
            process,
            NativeEventOperation::default(),
        );
    }

    fn mouse_wheel(&self, process: HookProcess<MouseWheelEvent>) {
        let hook = MouseHook::new(
            Arc::clone(&self.modifier_keys),
            process,
            NativeEventOperation::default(),
        );
        self.storage.borrow_mut().register_mouse_wheel_hotkey(hook);
    }

    fn mouse_cursor(&self, process: HookProcess<MouseCursorEvent>) {
        let hook = MouseHook::new(
            Arc::clone(&self.modifier_keys),
            process,
            NativeEventOperation::default(),
        );
        self.storage.borrow_mut().register_mouse_cursor_hotkey(hook);
    }

    fn disable(&self, target: impl Into<ButtonSet>) {
        let process = Arc::new(|_| {}) as Arc<dyn Fn(_) + Send + Sync>;
        let target = target.into();
        self.on_press(target.clone(), Arc::clone(&process));
        self.on_press(target, process);
    }

    fn add_modifier_keys(&self, modifier_keys: &ModifierKeys) -> ModifierHotkey {
        ModifierHotkey::new(
            &self.storage,
            Arc::new(modifier_keys.to_owned()),
            NativeEventOperation::default(),
        )
    }

    fn change_native_event_operation(&self, operation: NativeEventOperation) -> ModifierHotkey {
        ModifierHotkey::new(&self.storage, Arc::clone(&self.modifier_keys), operation)
    }
}

/// Registers Hotkeys with modifier keys.
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
    fn remap(&self, target: impl Into<ButtonSet>, behavior: impl Into<ButtonSet>) {
        let behavior = behavior.into();
        register_each_target(target.into(), &self.modifier_keys, |key, modifier_keys| {
            let hook = RemapHook::new(modifier_keys, behavior.clone());
            self.storage.borrow_mut().register_remap(key, hook);
        });
    }

    fn on_press(&self, target: impl Into<ButtonSet>, process: HookProcess<ButtonEvent>) {
        register_button_hotkey(
            target.into(),
            HotkeyStorage::register_hotkey_on_press,
            self.storage,
            &self.modifier_keys,
            process,
            self.native_event_operation,
        );
    }

    fn on_release(&self, target: impl Into<ButtonSet>, process: HookProcess<ButtonEvent>) {
        register_button_hotkey(
            target.into(),
            HotkeyStorage::register_hotkey_on_release,
            self.storage,
            &self.modifier_keys,
            process,
            self.native_event_operation,
        );
    }

    fn mouse_wheel(&self, process: HookProcess<MouseWheelEvent>) {
        let hook = MouseHook::new(
            Arc::clone(&self.modifier_keys),
            process,
            self.native_event_operation,
        );
        self.storage.borrow_mut().register_mouse_wheel_hotkey(hook);
    }

    fn mouse_cursor(&self, process: HookProcess<MouseCursorEvent>) {
        let hook = MouseHook::new(
            Arc::clone(&self.modifier_keys),
            process,
            self.native_event_operation,
        );
        self.storage.borrow_mut().register_mouse_cursor_hotkey(hook);
    }

    fn disable(&self, target: impl Into<ButtonSet>) {
        let process = Arc::new(|_| {}) as Arc<dyn Fn(_) + Send + Sync>;
        let target = target.into();
        self.on_press(target.clone(), Arc::clone(&process));
        self.on_press(target, process);
    }

    fn add_modifier_keys(&self, modifier_keys: &ModifierKeys) -> ModifierHotkey {
        ModifierHotkey::new(
            self.storage,
            Arc::new(self.modifier_keys.merge(modifier_keys)),
            self.native_event_operation,
        )
    }

    fn change_native_event_operation(&self, operation: NativeEventOperation) -> ModifierHotkey {
        ModifierHotkey::new(self.storage, Arc::clone(&self.modifier_keys), operation)
    }
}
