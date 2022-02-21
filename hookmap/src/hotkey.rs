//! Registering Hotkeys.

pub mod button_arg;
mod entry;
mod hook;
mod modifier_keys;
mod storage;

use modifier_keys::ModifierKeys;

use crate::runtime::Runtime;
use hook::HookProcess;
use hookmap_core::{Button, ButtonEvent, MouseCursorEvent, MouseWheelEvent, NativeEventOperation};
use std::sync::Arc;

use self::{
    button_arg::ButtonArg,
    entry::{Context, HotkeyEntry},
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
    entry: HotkeyEntry,
    context: Context,
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
        let runtime = Runtime::new(self.entry.into_inner());
        runtime.start();
    }
}

impl RegisterHotkey for Hotkey {
    fn remap(&self, target: impl Into<ButtonArg>, behavior: Button) {
        self.entry
            .remap(target.into(), behavior, self.context.clone());
    }

    fn on_press(&self, target: impl Into<ButtonArg>, process: impl IntoHookProcess<ButtonEvent>) {
        self.entry
            .on_press(target.into(), process.into(), self.context.clone());
    }

    fn on_release(&self, target: impl Into<ButtonArg>, process: impl IntoHookProcess<ButtonEvent>) {
        self.entry
            .on_release(target.into(), process.into(), self.context.clone());
    }

    fn mouse_wheel(&self, process: impl IntoHookProcess<MouseWheelEvent>) {
        self.entry.mouse_wheel(process.into(), self.context.clone());
    }

    fn mouse_cursor(&self, process: impl IntoHookProcess<MouseCursorEvent>) {
        self.entry
            .mouse_cursor(process.into(), self.context.clone());
    }

    fn disable(&self, target: impl Into<ButtonArg>) {
        self.entry.disable(target.into(), self.context.clone());
    }

    fn add_modifier_keys(&self, modifier_keys: impl Into<ButtonArg>) -> ModifierHotkey {
        let context = Context {
            modifier_keys: Some(Arc::new(ModifierKeys::from(modifier_keys.into()))),
            native_event_operation: self.context.native_event_operation,
        };
        ModifierHotkey::new(&self.entry, context)
    }

    fn change_native_event_operation(&self, operation: NativeEventOperation) -> ModifierHotkey {
        let mut context = self.context.clone();
        context.native_event_operation = operation;
        ModifierHotkey::new(&self.entry, context)
    }
}

/// Registers Hotkeys with modifier keys.
pub struct ModifierHotkey<'a> {
    entry: &'a HotkeyEntry,
    context: Context,
}

impl<'a> ModifierHotkey<'a> {
    fn new(entry: &'a HotkeyEntry, context: Context) -> Self {
        ModifierHotkey { entry, context }
    }
}

impl RegisterHotkey for ModifierHotkey<'_> {
    fn remap(&self, target: impl Into<ButtonArg>, behavior: Button) {
        self.entry
            .remap(target.into(), behavior, self.context.clone());
    }

    fn on_press(&self, target: impl Into<ButtonArg>, process: impl IntoHookProcess<ButtonEvent>) {
        self.entry
            .on_press(target.into(), process.into(), self.context.clone());
    }

    fn on_release(&self, target: impl Into<ButtonArg>, process: impl IntoHookProcess<ButtonEvent>) {
        self.entry
            .on_release(target.into(), process.into(), self.context.clone());
    }

    fn mouse_wheel(&self, process: impl IntoHookProcess<MouseWheelEvent>) {
        self.entry.mouse_wheel(process.into(), self.context.clone());
    }

    fn mouse_cursor(&self, process: impl IntoHookProcess<MouseCursorEvent>) {
        self.entry
            .mouse_cursor(process.into(), self.context.clone());
    }

    fn disable(&self, target: impl Into<ButtonArg>) {
        self.entry.disable(target.into(), self.context.clone());
    }

    fn add_modifier_keys(&self, modifier_keys: impl Into<ButtonArg>) -> ModifierHotkey {
        let new = ModifierKeys::from(modifier_keys.into());
        let modifier_keys = if let Some(modifier_keys) = &self.context.modifier_keys {
            modifier_keys.merge(new)
        } else {
            new
        };
        let context = Context {
            modifier_keys: Some(Arc::new(modifier_keys)),
            native_event_operation: self.context.native_event_operation,
        };
        ModifierHotkey::new(self.entry, context)
    }

    fn change_native_event_operation(&self, operation: NativeEventOperation) -> ModifierHotkey {
        let context = Context {
            modifier_keys: self.context.modifier_keys.clone(),
            native_event_operation: operation,
        };
        ModifierHotkey::new(self.entry, context)
    }
}
