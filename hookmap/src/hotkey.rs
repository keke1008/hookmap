//! Registering Hotkeys.

pub mod args;
pub mod interruption;
pub mod layer;

mod registrar;
mod storage;

use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::sync::Arc;

use hookmap_core::button::Button;
use hookmap_core::event::{ButtonEvent, CursorEvent, NativeEventOperation, WheelEvent};

use crate::layer::LayerIndex;
use crate::runtime::hook::{HookAction, OptionalProcedure, RequiredProcedure};
use crate::runtime::Runtime;

use self::args::ButtonArgs;
use self::interruption::Interruption;
use self::layer::{Layer, LayerCreator};
use self::registrar::HotkeyStorage;

#[derive(Debug, Default)]
struct Inner {
    storage: HotkeyStorage,
    layer_creator: LayerCreator,
}

#[derive(Debug)]
enum InnerMut {
    Strong(Rc<RefCell<Inner>>),
    Weak(Weak<RefCell<Inner>>),
}

impl Default for InnerMut {
    fn default() -> Self {
        Self::Strong(Rc::default())
    }
}

impl InnerMut {
    fn new() -> Self {
        Self::default()
    }

    fn apply<R>(&self, f: impl FnOnce(&mut Inner) -> R) -> R {
        match self {
            Self::Strong(rc) => f(&mut rc.borrow_mut()),
            Self::Weak(weak) => f(&mut weak.upgrade().unwrap().borrow_mut()),
        }
    }

    fn weak(&self) -> Self {
        match self {
            Self::Strong(rc) => Self::Weak(Rc::downgrade(rc)),
            Self::Weak(weak) => Self::Weak(weak.clone()),
        }
    }

    fn into_inner(self) -> Option<Inner> {
        match self {
            Self::Strong(rc) => Some(Rc::try_unwrap(rc).unwrap().into_inner()),
            Self::Weak(_) => None,
        }
    }
}

#[derive(Debug, Clone)]
struct Context {
    current_layer: LayerIndex,
    native: NativeEventOperation,
}

impl Context {
    fn replace_native(&self, native: NativeEventOperation) -> Self {
        Self {
            native,
            ..self.clone()
        }
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
#[derive(Debug)]
pub struct Hotkey {
    inner: InnerMut,
    context: Context,
}

impl Default for Hotkey {
    fn default() -> Self {
        let mut layer_creator = LayerCreator::new();
        let root_layer = layer_creator.create_independent_layer(true);
        let context = Context {
            current_layer: root_layer,
            native: NativeEventOperation::Dispatch,
        };

        Self {
            inner: InnerMut::new(),
            context,
        }
    }
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
        let Inner {
            storage,
            layer_creator,
        } = self
            .inner
            .into_inner()
            .expect("`Hotkey::install` must be called with root `Hotkey`.");
        let (layer_facade, layer_state, layer_tx, layer_rx) = layer_creator.destruct();
        let (input_storage, interruption_storage, layer_storage) = storage.destruct();

        let runtime = Runtime::new(
            input_storage,
            interruption_storage,
            layer_storage,
            layer_state,
            layer_facade,
        );

        let input_rx = hookmap_core::install_hook();
        runtime.start(input_rx, layer_tx, layer_rx);
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
    pub fn remap(self, target: impl Into<ButtonArgs>, behavior: Button) -> Self {
        self.inner.apply(|inner| match &target.into() {
            ButtonArgs::Each(targets) => inner.storage.remap(
                self.context.current_layer,
                targets,
                behavior,
                &mut inner.layer_creator,
            ),
            ButtonArgs::Not(ignore) => {
                inner.storage.remap_any(
                    self.context.current_layer,
                    Some(Arc::clone(ignore)),
                    behavior,
                    &mut inner.layer_creator,
                );
            }
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
        self,
        target: impl Into<ButtonArgs>,
        procedure: impl Into<RequiredProcedure<ButtonEvent>>,
    ) -> Self {
        let action = HookAction::Procedure {
            procedure: procedure.into().into(),
            native: self.context.native,
        };

        self.inner.apply(|inner| match &target.into() {
            ButtonArgs::Each(targets) => {
                inner
                    .storage
                    .on_press(self.context.current_layer, targets, action);
            }
            ButtonArgs::Not(ignore) => {
                inner.storage.on_press_any(
                    self.context.current_layer,
                    Some(Arc::clone(ignore)),
                    action,
                );
            }
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
        self,
        target: impl Into<ButtonArgs>,
        procedure: impl Into<OptionalProcedure<ButtonEvent>>,
    ) -> Self {
        let action = HookAction::Procedure {
            procedure: procedure.into().into(),
            native: self.context.native,
        };

        self.inner.apply(|inner| match target.into() {
            ButtonArgs::Each(ref targets) => {
                inner.storage.on_release(
                    self.context.current_layer,
                    targets,
                    action,
                    &mut inner.layer_creator,
                );
            }
            ButtonArgs::Not(ignore) => {
                inner.storage.on_release_any(
                    self.context.current_layer,
                    Some(ignore),
                    action,
                    &mut inner.layer_creator,
                );
            }
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
    pub fn mouse_cursor(self, procedure: impl Into<RequiredProcedure<CursorEvent>>) -> Self {
        let action = HookAction::Procedure {
            procedure: procedure.into().into(),
            native: self.context.native,
        };

        self.inner.apply(|inner| {
            inner
                .storage
                .mouse_cursor(self.context.current_layer, action);
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
    pub fn mouse_wheel(self, procedure: impl Into<RequiredProcedure<WheelEvent>>) -> Self {
        let action = HookAction::Procedure {
            procedure: procedure.into().into(),
            native: self.context.native,
        };

        self.inner.apply(|inner| {
            inner
                .storage
                .mouse_wheel(self.context.current_layer, action);
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
    pub fn disable(self, target: impl Into<ButtonArgs>) -> Self {
        self.inner.apply(|inner| match target.into() {
            ButtonArgs::Each(ref targets) => {
                inner.storage.disable(self.context.current_layer, targets)
            }
            ButtonArgs::Not(ignore) => {
                inner
                    .storage
                    .disable_any(self.context.current_layer, Some(ignore));
            }
        });

        self
    }

    /// Registers a `procedure` that will run when the current layer becomes active.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::prelude::*;
    ///
    /// let mut hotkey = Hotkey::new();
    ///
    /// let (layer, child) = hotkey.create_layer(false);
    /// child.on_layer_activated(|_| println!("Activated"));
    ///
    /// hotkey.on_press(Button::A, move |_| layer.enable());
    /// ```
    ///
    pub fn on_layer_activated(self, procedure: impl Into<OptionalProcedure<ButtonEvent>>) -> Self {
        let action = HookAction::Procedure {
            procedure: procedure.into().into(),
            native: self.context.native,
        };

        self.inner.apply(|inner| {
            inner
                .storage
                .on_layer_activated(self.context.current_layer, action);
        });

        self
    }

    /// Registers a `procedure` to be executed when the current layer becomes inactive.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::prelude::*;
    ///
    /// let mut hotkey = Hotkey::new();
    ///
    /// let (layer, child) = hotkey.create_layer(true);
    /// child.on_layer_inactivated(|_|println!("Inactivated"));
    ///
    /// hotkey.on_press(Button::A, move |_| layer.disable());
    /// ```
    ///
    pub fn on_layer_inactivated(
        self,
        procedure: impl Into<OptionalProcedure<ButtonEvent>>,
    ) -> Self {
        let action = HookAction::Procedure {
            procedure: procedure.into().into(),
            native: self.context.native,
        };

        self.inner.apply(|inner| {
            inner
                .storage
                .on_layer_inactivated(self.context.current_layer, action);
        });

        self
    }

    /// Creates a new independent layer and returns values to control the new layer and to register hooks
    /// on the new layer.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::prelude::*;
    /// use Button::*;
    ///
    /// let mut hotkey = Hotkey::new();
    ///
    /// let (layer, child) = hotkey.create_layer(true);
    /// child.remap(Button::A, Button::B);
    /// child.on_press(C, move |_| layer.disable());
    /// ```
    ///
    pub fn create_independent_layer(&self, init_state: bool) -> Layer {
        self.inner.apply(|inner| {
            let index = inner.layer_creator.create_independent_layer(init_state);
            inner.layer_creator.wrap_layer(index)
        })
    }

    /// Creates a new child layer and returns values to control the new layer and to register hooks
    /// on the new layer.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::prelude::*;
    /// use Button::*;
    ///
    /// let mut hotkey = Hotkey::new();
    ///
    /// let (layer, child) = hotkey.create_child_layer(true);
    /// child.remap(Button::A, Button::B);
    /// child.on_press(C, move |_| layer.disable());
    /// ```
    ///
    pub fn create_child_layer(&self, init_state: bool) -> Layer {
        self.inner.apply(|inner| {
            let index = inner
                .layer_creator
                .create_child_layer(self.context.current_layer, init_state);
            inner.layer_creator.wrap_layer(index)
        })
    }

    /// Creates a new sync layer and returns values to control the new layer and to
    /// register hooks on the new layer.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::prelude::*;
    /// use Button::*;
    ///
    /// let mut hotkey = Hotkey::new();
    ///
    /// let (layer, child) = hotkey.create_sync_layer(true);
    /// child.remap(Button::A, Button::B);
    /// child.on_press(C, move |_| layer.disable());
    /// ```
    ///
    pub fn create_sync_layer(&self, init_state: bool) -> Layer {
        self.inner.apply(|inner| {
            let index = inner
                .layer_creator
                .create_sync_layer(self.context.current_layer, init_state);
            inner.layer_creator.wrap_layer(index)
        })
    }

    /// Gets keyboard events dynamically.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::prelude::*;
    /// use Button::*;
    ///
    /// let hotkey = Hotkey::new();
    /// let i = hotkey.interruption();
    /// i.spawn(|int| {
    ///     int.iter().take(3).for_each(|e| println!("{e:?}"));
    /// });
    /// ```
    ///
    pub fn interruption(&self) -> Interruption {
        self.inner.apply(|inner| inner.storage.interruption())
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
        Hotkey {
            inner: self.inner.weak(),
            context: self.context.replace_native(NativeEventOperation::Block),
        }
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
        Hotkey {
            inner: self.inner.weak(),
            context: self.context.replace_native(NativeEventOperation::Dispatch),
        }
    }
}
