//! Registering Hotkeys.

pub mod button_arg;
mod entry;
mod hook;
mod modifier_keys;
mod storage;

use modifier_keys::ModifierKeys;

use crate::runtime::Runtime;
use hook::{HookProcess, HotkeyHook, MouseHook, RemapHook};
use hookmap_core::{Button, ButtonEvent, MouseCursorEvent, MouseWheelEvent, NativeEventOperation};
use std::{cell::RefCell, sync::Arc};
use storage::HotkeyStorage;

use self::{
    button_arg::{ButtonArg, ButtonArgElementTag},
    hook::{HotkeyCondition, HotkeyProcess},
};

pub trait IntoHookProcess<E> {
    fn into(self) -> HookProcess<E>;
}

impl<E> IntoHookProcess<E> for HookProcess<E> {
    fn into(self) -> HookProcess<E> {
        self
    }
}

impl<E, F: Fn(E) + Send + Sync + 'static> IntoHookProcess<E> for F {
    fn into(self) -> HookProcess<E> {
        Arc::new(self)
    }
}

impl<E, F: Fn(E) + Send + Sync + 'static> IntoHookProcess<E> for Arc<F> {
    fn into(self) -> HookProcess<E> {
        self
    }
}

/// Methods for registering hotkeys.
pub trait RegisterHotkey {
    /// Makes `target` behave like a `behavior`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::prelude::*;
    ///
    /// let hotkey = Hotkey::new();
    /// hotkey.remap(buttons!(A), Button::B);
    /// ```
    ///
    fn remap(&self, target: impl Into<ButtonArg>, behavior: Button);

    /// Run `process` when `target` is pressed.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::prelude::*;
    /// use std::sync::Arc;
    ///
    /// let hotkey = Hotkey::new();
    /// hotkey.on_press(buttons!(A), Arc::new(|e| println!("Pressed: {:?}", e)));
    /// ```
    ///
    fn on_press(&self, target: impl Into<ButtonArg>, process: impl IntoHookProcess<ButtonEvent>);

    /// Run `process` when `target` is released.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::prelude::*;
    /// use std::sync::Arc;
    ///
    /// let hotkey = Hotkey::new();
    /// hotkey.on_release(buttons!(A), Arc::new(|e| println!("Released: {:?}", e)));
    /// ```
    ///
    fn on_release(&self, target: impl Into<ButtonArg>, process: impl IntoHookProcess<ButtonEvent>);

    /// Run `process` when a mouse wheel is rotated.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::hotkey::{Hotkey, RegisterHotkey};
    /// use std::sync::Arc;
    ///
    /// let hotkey = Hotkey::new();
    /// hotkey.mouse_wheel(Arc::new(|delta| println!("Delta: {:?}", delta)));
    /// ```
    ///
    fn mouse_wheel(&self, process: impl IntoHookProcess<MouseWheelEvent>);

    /// Run `process` when a mouse cursor is moved.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::hotkey::{Hotkey, RegisterHotkey};
    /// use std::sync::Arc;
    ///
    /// let hotkey = Hotkey::new();
    /// hotkey.mouse_cursor(Arc::new(|(x, y)| println!("Cursor: ({}, {})", x, y)));
    /// ```
    ///
    fn mouse_cursor(&self, process: impl IntoHookProcess<MouseCursorEvent>);

    /// Disables the button and blocks events.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::prelude::*;
    /// use std::sync::Arc;
    ///
    /// let hotkey = Hotkey::new();
    /// hotkey.disable(buttons!(A));
    /// ```
    ///
    fn disable(&self, target: impl Into<ButtonArg>);

    /// Adds modifier keys.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::prelude::*;
    ///
    /// let hotkey = Hotkey::new();
    /// let a_or_b = hotkey.add_modifier_keys(buttons!(A, B));
    /// a_or_b.remap(buttons!(C), Button::D);
    /// ```
    fn add_modifier_keys(&self, modifier_keys: impl Into<ButtonArg>) -> ModifierHotkey;

    /// Changes the operation for native events to block or dispatch.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::{prelude::*, event::NativeEventOperation};
    /// use std::sync::Arc;
    ///
    /// let hotkey = Hotkey::new();
    /// let blocking_hotkey = hotkey.change_native_event_operation(NativeEventOperation::Block);
    /// blocking_hotkey.on_press(buttons!(A), Arc::new(|e| println!("Press: {:?}", e)));
    /// ```
    ///
    fn change_native_event_operation(&self, operation: NativeEventOperation) -> ModifierHotkey;
}

/// Registering Hotkeys.
///
/// # Examples
///
/// ```no_run
/// use hookmap::prelude::*;
///
/// let hotkey = Hotkey::new();
/// hotkey.remap(buttons!(A), Button::B);
/// hotkey.install();
/// ```
///
#[derive(Default)]
pub struct Hotkey {
    storage: RefCell<HotkeyStorage>,
    modifier_keys: Arc<ModifierKeys>,
    native_event_operation: NativeEventOperation,
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
    fn remap(&self, target: impl Into<ButtonArg>, behavior: Button) {
        let mut storage = self.storage.borrow_mut();
        let hook = Arc::new(RemapHook::new(Arc::clone(&self.modifier_keys), behavior));

        for arg in target.into().iter() {
            match arg.tag {
                ButtonArgElementTag::Inversion => panic!(),
                ButtonArgElementTag::Direct => {
                    storage.register_remap(arg.value, Arc::clone(&hook));
                }
            }
        }
    }

    fn on_press(&self, target: impl Into<ButtonArg>, process: impl IntoHookProcess<ButtonEvent>) {
        let mut storage = self.storage.borrow_mut();
        let hook = Arc::new(HotkeyHook::new(
            HotkeyCondition::Any,
            HotkeyProcess::Callback(process.into()),
            self.native_event_operation,
        ));

        for arg in target.into().iter() {
            match arg.tag {
                ButtonArgElementTag::Direct => {
                    storage.register_hotkey_on_press(arg.value, Arc::clone(&hook));
                }
                ButtonArgElementTag::Inversion => {
                    storage.register_hotkey_on_release(arg.value, Arc::clone(&hook));
                }
            }
        }
    }

    fn on_release(&self, target: impl Into<ButtonArg>, process: impl IntoHookProcess<ButtonEvent>) {
        let mut storage = self.storage.borrow_mut();
        let hook = Arc::new(HotkeyHook::new(
            HotkeyCondition::Any,
            HotkeyProcess::Callback(process.into()),
            self.native_event_operation,
        ));

        for arg in target.into().iter() {
            match arg.tag {
                ButtonArgElementTag::Direct => {
                    storage.register_hotkey_on_release(arg.value, Arc::clone(&hook));
                }
                ButtonArgElementTag::Inversion => {
                    storage.register_hotkey_on_press(arg.value, Arc::clone(&hook));
                }
            }
        }
    }

    fn mouse_wheel(&self, process: impl IntoHookProcess<MouseWheelEvent>) {
        let hook = Arc::new(MouseHook::new(
            Arc::clone(&self.modifier_keys),
            process.into(),
            self.native_event_operation,
        ));
        self.storage.borrow_mut().register_mouse_wheel_hotkey(hook);
    }

    fn mouse_cursor(&self, process: impl IntoHookProcess<MouseCursorEvent>) {
        let hook = Arc::new(MouseHook::new(
            Arc::clone(&self.modifier_keys),
            process.into(),
            self.native_event_operation,
        ));
        self.storage.borrow_mut().register_mouse_cursor_hotkey(hook);
    }

    fn disable(&self, target: impl Into<ButtonArg>) {
        let mut storage = self.storage.borrow_mut();
        let hook = Arc::new(HotkeyHook::new(
            HotkeyCondition::Any,
            HotkeyProcess::Noop,
            self.native_event_operation,
        ));

        for arg in target.into().iter() {
            storage.register_hotkey_on_press(arg.value, Arc::clone(&hook));
            storage.register_hotkey_on_release(arg.value, Arc::clone(&hook));
        }
    }

    fn add_modifier_keys(&self, modifier_keys: impl Into<ButtonArg>) -> ModifierHotkey {
        ModifierHotkey::new(
            &self.storage,
            Arc::new(ModifierKeys::from(modifier_keys.into())),
            self.native_event_operation,
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
    fn remap(&self, target: impl Into<ButtonArg>, behavior: Button) {
        let mut storage = self.storage.borrow_mut();
        let hook = Arc::new(RemapHook::new(Arc::clone(&self.modifier_keys), behavior));

        for arg in target.into().iter() {
            match arg.tag {
                ButtonArgElementTag::Inversion => panic!(),
                ButtonArgElementTag::Direct => {
                    storage.register_remap(arg.value, Arc::clone(&hook));
                }
            }
        }
    }

    fn on_press(&self, target: impl Into<ButtonArg>, process: impl IntoHookProcess<ButtonEvent>) {
        let mut storage = self.storage.borrow_mut();
        let hook = Arc::new(HotkeyHook::new(
            HotkeyCondition::Modifier(Arc::clone(&self.modifier_keys)),
            HotkeyProcess::Callback(process.into()),
            self.native_event_operation,
        ));

        for arg in target.into().iter() {
            match arg.tag {
                ButtonArgElementTag::Direct => {
                    storage.register_hotkey_on_press(arg.value, Arc::clone(&hook));
                }
                ButtonArgElementTag::Inversion => {
                    storage.register_hotkey_on_release(arg.value, Arc::clone(&hook));
                }
            }
        }
    }

    fn on_release(&self, target: impl Into<ButtonArg>, process: impl IntoHookProcess<ButtonEvent>) {
        let mut storage = self.storage.borrow_mut();
        let process = process.into();

        for arg in target.into().iter() {
            let is_active = Arc::default();
            let inactivation_hook = Arc::new(HotkeyHook::new(
                HotkeyCondition::Activation(Arc::clone(&is_active)),
                HotkeyProcess::Callback(Arc::clone(&process)),
                self.native_event_operation,
            ));
            let is_active = Arc::clone(&is_active);
            let activation_hook = Arc::new(HotkeyHook::new(
                HotkeyCondition::Modifier(Arc::clone(&self.modifier_keys)),
                HotkeyProcess::Activate(Arc::clone(&is_active)),
                NativeEventOperation::Dispatch,
            ));

            match arg.tag {
                ButtonArgElementTag::Direct => {
                    storage.register_hotkey_on_press(arg.value, Arc::clone(&activation_hook));
                    storage.register_hotkey_on_release(arg.value, Arc::clone(&inactivation_hook));
                }
                ButtonArgElementTag::Inversion => {
                    storage.register_hotkey_on_release(arg.value, Arc::clone(&activation_hook));
                    storage.register_hotkey_on_press(arg.value, Arc::clone(&inactivation_hook));
                }
            }
            for target in &self.modifier_keys.pressed {
                storage.register_hotkey_on_release(*target, Arc::clone(&inactivation_hook));
            }
            for target in &self.modifier_keys.released {
                storage.register_hotkey_on_press(*target, Arc::clone(&inactivation_hook));
            }
        }
    }

    fn mouse_wheel(&self, process: impl IntoHookProcess<MouseWheelEvent>) {
        let hook = Arc::new(MouseHook::new(
            Arc::clone(&self.modifier_keys),
            process.into(),
            self.native_event_operation,
        ));
        self.storage.borrow_mut().register_mouse_wheel_hotkey(hook);
    }

    fn mouse_cursor(&self, process: impl IntoHookProcess<MouseCursorEvent>) {
        let hook = Arc::new(MouseHook::new(
            Arc::clone(&self.modifier_keys),
            process.into(),
            self.native_event_operation,
        ));
        self.storage.borrow_mut().register_mouse_cursor_hotkey(hook);
    }

    fn disable(&self, target: impl Into<ButtonArg>) {
        let mut storage = self.storage.borrow_mut();
        let hook = Arc::new(HotkeyHook::new(
            HotkeyCondition::Modifier(Arc::clone(&self.modifier_keys)),
            HotkeyProcess::Noop,
            self.native_event_operation,
        ));

        for arg in target.into().iter() {
            storage.register_hotkey_on_press(arg.value, Arc::clone(&hook));
            storage.register_hotkey_on_release(arg.value, Arc::clone(&hook));
        }
    }

    fn add_modifier_keys(&self, modifier_keys: impl Into<ButtonArg>) -> ModifierHotkey {
        ModifierHotkey::new(
            self.storage,
            Arc::new(
                self.modifier_keys
                    .merge(ModifierKeys::from(modifier_keys.into())),
            ),
            self.native_event_operation,
        )
    }

    fn change_native_event_operation(&self, operation: NativeEventOperation) -> ModifierHotkey {
        ModifierHotkey::new(self.storage, Arc::clone(&self.modifier_keys), operation)
    }
}
