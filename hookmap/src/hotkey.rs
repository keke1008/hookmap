//! Registering Hotkeys.

mod layer;
mod registrar;

use std::borrow::BorrowMut;
use std::sync::mpsc::Receiver;

use hookmap_core::event::{ButtonEvent, CursorEvent, WheelEvent};
use layer::Layer;
use registrar::{Context, Registrar};

use hookmap_core::button::Button;

use crate::hook::hook::Procedure;
use crate::hook::layer::{LayerIndex, LayerState};
use crate::hook::storage::{HotkeyStorage, LayerHookStorage};
use crate::runtime::hook::{layer_query_channel, LayerQuery};
use crate::runtime::Runtime;

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
#[derive(Debug)]
pub struct Hotkey<T: BorrowMut<Registrar>> {
    registrar: T,
    context: Context,
    rx: Option<Receiver<LayerQuery<LayerIndex>>>,
}

impl Hotkey<Registrar> {
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
        let mut state = LayerState::new();
        let context = Context::new(
            state.create_root_layer(true),
            hookmap_core::event::NativeEventOperation::Dispatch,
        );

        let (tx, rx) = layer_query_channel();
        let registrar = Registrar::new(state, LayerHookStorage::new(), HotkeyStorage::new(), tx);

        Self {
            registrar,
            context,
            rx: Some(rx),
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
        let Registrar {
            state,
            layer_storage,
            hotkey_storage,
            ..
        } = self.registrar;

        let runtime = Runtime::new(layer_storage, hotkey_storage, state);
        let input_rx = hookmap_core::install_hook();
        runtime.start(self.rx.unwrap(), input_rx);
    }
}

impl<T: BorrowMut<Registrar>> Hotkey<T> {
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
    pub fn remap(&mut self, target: Button, behavior: Button) -> &mut Self {
        self.registrar
            .borrow_mut()
            .remap(&self.context, target, behavior);
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
    pub fn on_press(&mut self, target: Button, procedure: Procedure<ButtonEvent>) -> &mut Self {
        self.registrar
            .borrow_mut()
            .on_press(&self.context, target, procedure);
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
        target: Button,
        procedure: Procedure<Option<ButtonEvent>>,
    ) -> &mut Self {
        self.registrar
            .borrow_mut()
            .on_release(&self.context, target, procedure);
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
    pub fn mouse_cursor(&mut self, procedure: Procedure<CursorEvent>) -> &mut Self {
        self.registrar
            .borrow_mut()
            .mouse_cursor(&self.context, procedure);
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
    pub fn mouse_wheel(&mut self, procedure: Procedure<WheelEvent>) -> &mut Self {
        self.registrar
            .borrow_mut()
            .mouse_wheel(&self.context, procedure);
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
    pub fn disable(&mut self, target: Button) -> &mut Self {
        self.registrar.borrow_mut().disable(&self.context, target);
        self
    }

    pub fn layer(&mut self, init_state: bool) -> (Layer, Hotkey<&mut Registrar>) {
        let layer = self.registrar.borrow_mut().layer(&self.context, init_state);
        let context = self.context.replace_layer_id(layer.id());
        (
            layer,
            Hotkey {
                registrar: self.registrar.borrow_mut(),
                context,
                rx: None,
            },
        )
    }
}

impl Default for Hotkey<Registrar> {
    fn default() -> Self {
        Self::new()
    }
}
