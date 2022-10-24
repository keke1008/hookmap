//! Registering Hotkeys.

pub mod condition;
pub mod flag;

mod hook;
mod registrar;
mod storage;

use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};

use hookmap_core::button::Button;
use hookmap_core::event::{ButtonEvent, CursorEvent, NativeEventOperation, WheelEvent};

use crate::condition::flag::FlagState;
use crate::condition::view::View;
use crate::runtime::hook::{FlagEvent, HookAction, OptionalProcedure, RequiredProcedure};
use crate::runtime::Runtime;

use self::condition::HotkeyCondition;
use self::registrar::HotkeyStorage;

#[derive(Debug)]
struct Inner {
    storage: HotkeyStorage,
    state: Arc<Mutex<FlagState>>,
    flag_tx: Sender<FlagEvent>,
    flag_rx: Receiver<FlagEvent>,
}

impl Default for Inner {
    fn default() -> Self {
        let (flag_tx, flag_rx) = mpsc::channel();
        Inner {
            storage: HotkeyStorage::default(),
            state: Arc::default(),
            flag_tx,
            flag_rx,
        }
    }
}

#[derive(Debug)]
struct InnerMut {
    strong: Option<Rc<RefCell<Inner>>>,
    weak: Weak<RefCell<Inner>>,
}

impl InnerMut {
    fn new() -> Self {
        let inner = Rc::default();
        let weak = Rc::downgrade(&inner);
        InnerMut {
            strong: Some(inner),
            weak,
        }
    }

    fn weak(&self) -> Self {
        InnerMut {
            strong: None,
            weak: self.weak.clone(),
        }
    }

    fn apply<R>(&self, f: impl FnOnce(&mut Inner) -> R) -> R {
        f(&mut self.weak.upgrade().unwrap().borrow_mut())
    }

    fn into_inner(self) -> Option<Inner> {
        Rc::try_unwrap(self.strong?).map(RefCell::into_inner).ok()
    }
}

#[derive(Debug, Clone)]
struct Context {
    view: Arc<View>,
    native: NativeEventOperation,
}

impl Context {
    fn replace_view(&self, view: Arc<View>) -> Self {
        Self {
            view,
            ..self.clone()
        }
    }

    fn replace_native(&self, native: NativeEventOperation) -> Self {
        Self {
            native,
            ..self.clone()
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            view: Arc::default(),
            native: NativeEventOperation::Dispatch,
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
        let context = Context {
            view: Arc::default(),
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
            state,
            flag_tx,
            flag_rx,
        } = self
            .inner
            .into_inner()
            .expect("`Hotkey::install` must be called with root `Hotkey`.");
        let (input_storage, flag_storage) = storage.destruct();

        let runtime = Runtime::new(input_storage, flag_storage, state);

        let input_rx = hookmap_core::install_hook();
        runtime.start(input_rx, flag_tx, flag_rx);
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
        self.inner.apply(|inner| {
            inner.storage.remap(
                Arc::clone(&self.context.view),
                target,
                behavior,
                &mut inner.state.lock().unwrap(),
            )
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
        let action = HookAction::Procedure {
            procedure: procedure.into().into(),
            native: self.context.native,
        };
        self.inner.apply(|inner| {
            inner
                .storage
                .on_press(Arc::clone(&self.context.view), target, action);
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
        procedure: impl Into<OptionalProcedure<ButtonEvent>>,
    ) -> &Self {
        let action = HookAction::Procedure {
            procedure: procedure.into().into(),
            native: self.context.native,
        };
        self.inner.apply(|inner| {
            inner.storage.on_release(
                Arc::clone(&self.context.view),
                target,
                action,
                &mut inner.state.lock().unwrap(),
            )
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
        let action = HookAction::Procedure {
            procedure: procedure.into().into(),
            native: self.context.native,
        };

        self.inner.apply(|inner| {
            inner
                .storage
                .mouse_cursor(Arc::clone(&self.context.view), action);
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
        let action = HookAction::Procedure {
            procedure: procedure.into().into(),
            native: self.context.native,
        };

        self.inner.apply(|inner| {
            inner
                .storage
                .mouse_wheel(Arc::clone(&self.context.view), action);
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
        self.inner.apply(|inner| {
            inner
                .storage
                .disable(Arc::clone(&self.context.view), target)
        });

        self
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

    pub fn conditional(&self, condition: &mut impl HotkeyCondition) -> Self {
        let mut root = Hotkey {
            inner: self.inner.weak(),
            context: Context::default(),
        };

        Hotkey {
            inner: self.inner.weak(),
            context: self
                .context
                .replace_view(condition.view(&mut root, todo!())),
        }
    }
}
