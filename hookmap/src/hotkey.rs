//! Registering Hotkeys.

mod context;
mod hook;
mod storage;

pub use self::context::Context;

use self::hook::{Condition, HotkeyAction, HotkeyHook, MouseHook, Process, RemapHook};
use self::storage::HotkeyStorage;
use crate::macros::button_arg::{ButtonArg, ButtonArgUnit};
use crate::runtime::Runtime;

use hookmap_core::button::Button;
use hookmap_core::event::{ButtonEvent, CursorEvent, NativeEventOperation, WheelEvent};

use std::sync::Arc;

/// Registers and installs hotkeys.
///
/// # Examples
///
/// ```no_run
/// use hookmap::prelude::*;
///
/// let mut hotkey = Hotkey::new();
/// hotkey
///     .register(Context::default())
///     .remap(buttons!(A, B), Button::C);
/// hotkey.install();
/// ```
///
#[derive(Debug, Default)]
pub struct Hotkey {
    storage: HotkeyStorage,
}

impl Hotkey {
    /// Creates a new instance of [`Hotkey`].
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::prelude::*;
    ///
    /// let mut hotkey = Hotkey::new();
    /// ```
    ///
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a [`Registrar`] to register hotkeys.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::prelude::*;
    ///
    /// let mut hotkey = Hotkey::new();
    /// hotkey
    ///     .register(Context::default())
    ///     .remap(Button::A, Button::B);
    /// ```
    ///
    pub fn register(&mut self, context: Context) -> Registrar {
        Registrar {
            storage: &mut self.storage,
            context,
        }
    }

    /// Installs hotkeys and blocks the current thread.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use hookmap::prelude::*;
    ///
    /// let mut hotkey = Hotkey::new();
    /// hotkey.install();
    /// ```
    ///
    pub fn install(self) {
        let runtime = Runtime::new(self.storage);
        runtime.start();
    }
}

/// Register hotkeys.
/// Calls [`Hotkey::register`] to get this instance.
///
/// # Examples
///
/// ```
/// use hookmap::prelude::*;
///
/// let mut hotkey = Hotkey::new();
/// hotkey
///     .register(Context::default())
///     .remap(Button::A, Button::B)
///     .on_press(Button::C, |e| println!("{:?}", e));
///
/// ```
pub struct Registrar<'a> {
    storage: &'a mut HotkeyStorage,
    context: Context,
}

impl<'a> Registrar<'a> {
    /// Makes `target` behave like a `behavior`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::prelude::*;
    ///
    /// let mut hotkey = Hotkey::new();
    /// hotkey
    ///     .register(Context::default())
    ///     .remap(Button::A, Button::B);
    /// ```
    ///
    pub fn remap(&mut self, targets: impl Into<ButtonArg>, behavior: Button) -> &mut Self {
        let targets = targets.into();
        let hook = Arc::new(RemapHook::new(self.context.to_condition(), behavior));
        assert!(targets.is_all_plain());

        for target in targets.iter_plain() {
            self.storage.register_remap(target, Arc::clone(&hook));
        }
        self
    }
    /// Run `process` when `target` is pressed.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::prelude::*;
    ///
    /// let mut hotkey = Hotkey::new();
    /// hotkey
    ///     .register(Context::default())
    ///     .on_press(buttons!(A), |e| println!("Pressed: {:?}", e));
    /// ```
    ///
    pub fn on_press(
        &mut self,
        targets: impl Into<ButtonArg>,
        process: impl Into<Process<ButtonEvent>>,
    ) -> &mut Self {
        let targets = targets.into();
        let hook = Arc::new(HotkeyHook::new(
            self.context.to_condition(),
            HotkeyAction::Process(process.into()),
            self.context.native_event_operation,
        ));

        for target in targets.iter_plain() {
            self.storage
                .register_hotkey_on_press(target, Arc::clone(&hook));
        }
        for target in targets.iter_not() {
            self.storage
                .register_hotkey_on_release(target, Arc::clone(&hook));
        }
        self
    }

    /// Run `process` when `target` is released.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::prelude::*;
    ///
    /// let mut hotkey = Hotkey::new();
    /// hotkey
    ///     .register(Context::default())
    ///     .on_release(buttons!(A), |e| println!("Released: {:?}", e));
    /// ```
    ///
    pub fn on_release(
        &mut self,
        targets: impl Into<ButtonArg>,
        process: impl Into<Process<ButtonEvent>>,
    ) -> &mut Self {
        let targets = targets.into();
        let condition = self.context.to_condition();
        let process = HotkeyAction::Process(process.into());

        if self.context.has_no_modifiers() {
            let hook = Arc::new(HotkeyHook::new(
                condition,
                process,
                self.context.native_event_operation,
            ));

            for target in targets.iter_plain() {
                self.storage
                    .register_hotkey_on_release(target, Arc::clone(&hook));
            }
            for target in targets.iter_not() {
                self.storage
                    .register_hotkey_on_press(target, Arc::clone(&hook));
            }
            return self;
        }

        for target in targets.iter() {
            let is_active = Arc::default();
            let inactivation_hook = Arc::new(HotkeyHook::new(
                Condition::Activation(Arc::clone(&is_active)),
                process.clone(),
                self.context.native_event_operation,
            ));
            let activation_hook = Arc::new(HotkeyHook::new(
                condition.clone(),
                HotkeyAction::Activate(is_active),
                NativeEventOperation::Dispatch,
            ));

            match target {
                ButtonArgUnit::Plain(target) => {
                    self.storage
                        .register_hotkey_on_press(target, Arc::clone(&activation_hook));
                    self.storage
                        .register_hotkey_on_release(target, Arc::clone(&inactivation_hook));
                }
                ButtonArgUnit::Not(target) => {
                    self.storage
                        .register_hotkey_on_release(target, Arc::clone(&activation_hook));
                    self.storage
                        .register_hotkey_on_press(target, Arc::clone(&inactivation_hook));
                }
            }

            for target in self.context.iter_pressed() {
                self.storage
                    .register_hotkey_on_release(*target, Arc::clone(&inactivation_hook));
            }
            for target in self.context.iter_released() {
                self.storage
                    .register_hotkey_on_press(*target, Arc::clone(&inactivation_hook));
            }
        }
        self
    }

    /// Run `process` when a mouse wheel is rotated.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::prelude::*;
    ///
    /// let mut hotkey = Hotkey::new();
    /// hotkey
    ///     .register(Context::default())
    ///     .mouse_wheel(|e: WheelEvent| println!("Delta: {}", e.delta));
    /// ```
    ///
    pub fn mouse_wheel(&mut self, process: impl Into<Process<WheelEvent>>) -> &mut Self {
        let hook = Arc::new(MouseHook::new(
            self.context.to_condition(),
            process.into(),
            self.context.native_event_operation,
        ));
        self.storage.register_mouse_wheel_hotkey(hook);
        self
    }

    /// Run `process` when a mouse cursor is moved.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::prelude::*;
    ///
    /// let mut hotkey = Hotkey::new();
    /// hotkey
    ///     .register(Context::default())
    ///     .mouse_cursor(|e: CursorEvent| println!("movement distance: {:?}", e.delta));
    /// ```
    ///
    pub fn mouse_cursor(&mut self, process: impl Into<Process<CursorEvent>>) -> &mut Self {
        let hook = Arc::new(MouseHook::new(
            self.context.to_condition(),
            process.into(),
            self.context.native_event_operation,
        ));
        self.storage.register_mouse_cursor_hotkey(hook);
        self
    }

    /// Disables the button and blocks events.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::prelude::*;
    ///
    /// let mut hotkey = Hotkey::new();
    /// hotkey
    ///     .register(Context::default())
    ///     .disable(buttons!(A));
    /// ```
    ///
    pub fn disable(&mut self, targets: impl Into<ButtonArg>) -> &mut Self {
        let hook = Arc::new(HotkeyHook::new(
            self.context.to_condition(),
            HotkeyAction::Noop,
            NativeEventOperation::Block,
        ));
        let targets = targets.into();
        assert!(targets.is_all_plain());

        for target in targets.iter_plain() {
            self.storage
                .register_hotkey_on_press(target, Arc::clone(&hook));
            self.storage
                .register_hotkey_on_release(target, Arc::clone(&hook));
        }

        self
    }
}
