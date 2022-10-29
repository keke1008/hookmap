//! Registering Hotkeys.

pub mod condition;
pub mod flag;

mod registrar;
mod shared;

use std::cell::RefCell;
use std::sync::mpsc::{self, Receiver, SyncSender};
use std::sync::Arc;

use hookmap_core::button::Button;
use hookmap_core::event::{ButtonEvent, CursorEvent, NativeEventOperation, WheelEvent};

use crate::condition::view::View;
use crate::runtime::Runtime;
use crate::storage::action::FlagEvent;
use crate::storage::procedure::{OptionalProcedure, RequiredProcedure};
use crate::storage::ViewHookStorage;

use self::condition::{HookRegistrar, HotkeyCondition, ViewContext};
use self::registrar::{Context, InputHookRegistrar};
use self::shared::Shared;

#[derive(Debug)]
struct RuntimeArgs {
    flag_tx: SyncSender<FlagEvent>,
    flag_rx: Receiver<FlagEvent>,
}

impl Default for RuntimeArgs {
    fn default() -> Self {
        let (flag_tx, flag_rx) = mpsc::sync_channel(32);
        Self { flag_tx, flag_rx }
    }
}

/// Registers and installs hotkeys.
///
/// # Examples
///
/// ```no_run
/// use hookmap::prelude::*;
///
/// let mut hotkey = Hotkey::new();
/// hotkey.remap(Button::A, Button::C);
/// hotkey.install();
/// ```
///
#[derive(Debug, Default)]
pub struct Hotkey {
    input_registrar: Shared<RefCell<InputHookRegistrar>>,
    view_storage: Shared<RefCell<ViewHookStorage>>,
    runtime_args: Shared<RuntimeArgs>,
    context: Context,
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

    /// Installs hooks and blocks the current thread.
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
        let Self {
            input_registrar,
            view_storage,
            context,
            runtime_args,
        } = self;

        let input_registrar = input_registrar
            .into_inner()
            .map(RefCell::into_inner)
            .expect("`Hotkey::install` must be called with root `Hotkey`.");
        let view_storage = view_storage
            .into_inner()
            .map(RefCell::into_inner)
            .expect("`Hotkey::install` must be called with root `Hotkey`.");
        let runtime_args = runtime_args
            .into_inner()
            .expect("`Hotkey::install` must be called with root `Hotkey`.");
        let runtime = Runtime::new(input_registrar.into_inner(), view_storage, context.state);

        let input_rx = hookmap_core::install_hook();
        runtime.start(input_rx, runtime_args.flag_tx, runtime_args.flag_rx);
    }

    /// Remaps `target` to `behavior`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::prelude::*;
    ///
    /// let mut hotkey = Hotkey::new();
    /// hotkey
    ///     .remap(Button::A, Button::B)
    ///     .remap(Button::C, Button::D);
    /// ```
    ///
    pub fn remap(&self, target: Button, behavior: Button) -> &Self {
        self.input_registrar.apply_mut(|input_registrar| {
            self.view_storage.apply_mut(|view_storage| {
                input_registrar.remap(target, behavior, &self.context, view_storage);
            });
        });

        self
    }

    /// Registers a `procedure` to be executed when the `target` button is pressed.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::prelude::*;
    ///
    /// let mut hotkey = Hotkey::new();
    /// hotkey
    ///     .on_press(Button::A, |e| println!("Pressed: {e:?}"));
    /// ```
    ///
    pub fn on_press(
        &self,
        target: Button,
        procedure: impl Into<RequiredProcedure<ButtonEvent>>,
    ) -> &Self {
        self.input_registrar.apply_mut(|input_registrar| {
            input_registrar.on_press(target, procedure.into(), &self.context);
        });

        self
    }

    /// Registers a `procedure` to be executed when the `target` button is released.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::prelude::*;
    ///
    /// let mut hotkey = Hotkey::new();
    /// hotkey
    ///     .on_release(Button::A, |e| println!("Released: {:?}", e));
    /// ```
    ///
    pub fn on_release(
        &self,
        target: Button,
        procedure: impl Into<RequiredProcedure<ButtonEvent>>,
    ) -> &Self {
        self.input_registrar.apply_mut(|input_registrar| {
            input_registrar.on_release(target, procedure.into(), &self.context);
        });

        self
    }

    pub fn on_release_certainly(
        &self,
        target: Button,
        procedure: impl Into<OptionalProcedure<ButtonEvent>>,
    ) -> &Self {
        self.input_registrar.apply_mut(|input_registrar| {
            self.view_storage.apply_mut(|view_storage| {
                input_registrar.on_release_certainly(
                    target,
                    procedure.into(),
                    &self.context,
                    view_storage,
                );
            });
        });

        self
    }

    /// Registers a `procedure` to be executed when the mouse cursor is moved.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::prelude::*;
    ///
    /// let mut hotkey = Hotkey::new();
    /// hotkey
    ///     .mouse_cursor(|e: CursorEvent| println!("movement distance: {:?}", e.delta));
    /// ```
    ///
    pub fn mouse_cursor(&self, procedure: impl Into<RequiredProcedure<CursorEvent>>) -> &Self {
        self.input_registrar.apply_mut(|input_registrar| {
            input_registrar.mouse_cursor(procedure.into(), &self.context);
        });

        self
    }

    /// Registers a `procedure` to be executed when the mouse wheel is rotated.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::prelude::*;
    ///
    /// let mut hotkey = Hotkey::new();
    /// hotkey
    ///     .mouse_wheel(|e: WheelEvent| println!("Delta: {}", e.delta));
    /// ```
    ///
    pub fn mouse_wheel(&self, procedure: impl Into<RequiredProcedure<WheelEvent>>) -> &Self {
        self.input_registrar.apply_mut(|input_registrar| {
            input_registrar.mouse_wheel(procedure.into(), &self.context);
        });

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
    /// hotkey.disable(Button::A);
    /// ```
    ///
    pub fn disable(&self, target: Button) -> &Self {
        self.input_registrar.apply_mut(|input_registrar| {
            input_registrar.disable(target, &self.context);
        });

        self
    }

    fn clone_with_context(&self, context: Context) -> Self {
        Hotkey {
            input_registrar: self.input_registrar.weak(),
            view_storage: self.view_storage.weak(),
            runtime_args: self.runtime_args.weak(),
            context,
        }
    }

    /// Ensure that events are blocked when hooks registered through the return value of this
    /// function are executed.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::prelude::*;
    /// use Button::*;
    ///
    /// let hotkey = Hotkey::new();
    /// let blocked = hotkey.block();
    ///
    /// blocked.on_press(A, |e| println!("{e:?}"));
    /// ```
    ///
    pub fn block(&self) -> Hotkey {
        self.clone_with_context(self.context.replace_native(NativeEventOperation::Block))
    }

    /// Ensure that events are not blocked when hooks registered through the return value of this
    /// function are executed.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::prelude::*;
    /// use Button::*;
    ///
    /// let hotkey = Hotkey::new();
    /// let blocked = hotkey.dispatch();
    ///
    /// blocked.on_press(A, |e| println!("{e:?}"));
    /// ```
    ///
    pub fn dispatch(&self) -> Hotkey {
        self.clone_with_context(self.context.replace_native(NativeEventOperation::Dispatch))
    }

    pub fn conditional(&self, mut condition: impl HotkeyCondition) -> Self {
        let mut hotkey = HookRegistrar::new(
            self.input_registrar.weak(),
            self.view_storage.weak(),
            Arc::clone(&self.context.state),
        );

        let flag_tx = self.runtime_args.apply(|a| a.flag_tx.clone());
        let mut context = ViewContext::new(
            Arc::clone(&self.context.state),
            flag_tx,
            Arc::default(),
            Arc::clone(&self.context.view),
        );

        let view = View::new()
            .merge(&self.context.view)
            .merge(&condition.view(&mut hotkey, &mut context));

        self.clone_with_context(self.context.replace_view(view.into()))
    }
}
