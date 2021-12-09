use crate::{
    button::ButtonSet,
    hotkey::{
        Action, HotkeyAction, HotkeyInfo, ModifierKeys, MouseEventHandler, RemapInfo, Trigger,
    },
    runtime::Register,
};
use hookmap_core::{ButtonEvent, MouseCursorEvent, MouseWheelEvent, NativeEventOperation};
use std::{cell::RefCell, rc::Weak, sync::Arc};
use typed_builder::TypedBuilder;

/// A struct for registering handlers for the buttons.
#[derive(TypedBuilder)]
pub struct ButtonEventHandlerEntry {
    register: Weak<RefCell<Register>>,
    trigger: Trigger,

    #[builder(default)]
    modifier_keys: Arc<ModifierKeys>,

    #[builder(default)]
    native_event_operation: NativeEventOperation,
}

impl ButtonEventHandlerEntry {
    fn register_hotkey(&self, action: HotkeyAction) {
        let hotkey_info = HotkeyInfo::builder()
            .trigger(self.trigger.clone())
            .modifier_keys(Arc::clone(&self.modifier_keys))
            .native_event_operation(self.native_event_operation)
            .action(action)
            .build();
        self.register
            .upgrade()
            .unwrap()
            .borrow_mut()
            .register_hotkey(hotkey_info);
    }

    /// Registers a handler called when the specified button is pressed.
    ///
    /// # Arguments
    ///
    /// * `callback` - A function that takes [`ButtonEvent`].
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::*;
    /// let hotkey = Hotkey::new();
    /// hotkey.bind(Button::A).on_press(|_| println!("The A key is pressed"));
    /// ```
    ///
    pub fn on_press<F>(&self, callback: F)
    where
        F: Fn(ButtonEvent) + Send + Sync + 'static,
    {
        self.register_hotkey(HotkeyAction::Press(callback.into()));
    }

    /// Registers a handler called when the specified button is pressed or released.
    ///
    /// # Arguments
    ///
    /// * `callback` - A function that takes [`ButtonEvent`].
    ///
    /// # Example
    /// ```
    /// use hookmap::*;
    /// let hotkey = Hotkey::new();
    /// hotkey.bind(Button::A).on_press_or_release(|event| {
    ///     match event.action {
    ///         ButtonAction::Press => println!("The A key is pressed"),
    ///         ButtonAction::Release => println!("The A key is released"),
    ///     };
    /// });
    /// ```
    ///
    pub fn on_press_or_release<F>(&self, callback: F)
    where
        F: Fn(ButtonEvent) + Send + Sync + 'static,
    {
        self.register_hotkey(HotkeyAction::PressOrRelease(callback.into()));
    }

    /// Registers a handler called when the specified button is released.
    ///
    /// # Arguments
    ///
    /// * `callback` - A function that takes [`ButtonEvent`].
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::*;
    /// let hotkey = Hotkey::new();
    /// hotkey.bind(Button::A).on_release(|_| println!("The A key is released"));
    /// ```
    ///
    pub fn on_release<F>(&self, callback: F)
    where
        F: Fn(ButtonEvent) + Send + Sync + 'static,
    {
        self.register_hotkey(HotkeyAction::Release(callback.into()));
    }

    /// Registers handlers called when the specified button is pressed or released, respectively.
    ///
    /// # Arguments
    ///
    /// * `on_press` - A function that takes [`ButtonEvent`].
    /// * `on_release` - A function that takes [`ButtonEvent`].
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::*;
    /// let hotkey = Hotkey::new();
    /// hotkey.bind(Button::A).on_press_and_release(
    ///     |_| println!("The A key is pressed"),
    ///     |_| println!("The A key is released"),
    /// );
    /// ```
    ///
    pub fn on_press_and_release<F, G>(self, on_press: F, on_release: G)
    where
        F: Fn(ButtonEvent) + Send + Sync + 'static,
        G: Fn(ButtonEvent) + Send + Sync + 'static,
    {
        self.register_hotkey(HotkeyAction::PressAndRelease {
            on_press: on_press.into(),
            on_release: on_release.into(),
        });
    }

    /// Disables the button and blocks the event.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::*;
    /// let hotkey = Hotkey::new();
    /// hotkey.bind(Button::A).disable();
    /// ```
    pub fn disable(&self) {
        let hotkey_info = HotkeyInfo::builder()
            .trigger(self.trigger.clone())
            .modifier_keys(Arc::clone(&self.modifier_keys))
            .native_event_operation(self.native_event_operation)
            .action(HotkeyAction::PressOrRelease(Action::Noop))
            .build();
        self.register
            .upgrade()
            .unwrap()
            .borrow_mut()
            .register_hotkey(hotkey_info);
    }
}

/// A struct for registering handlers for the mouse cursor.
#[derive(TypedBuilder)]
pub struct MouseCursorHotKeyEntry {
    register: Weak<RefCell<Register>>,

    #[builder(default)]
    modifier_keys: Arc<ModifierKeys>,

    #[builder(default)]
    native_event_operation: NativeEventOperation,
}

impl MouseCursorHotKeyEntry {
    /// Registers a handler called when the mouse cursor is moved.
    ///
    /// # Arguments
    ///
    /// * `callback` - A function that takes a absolute postion of the mouse cursor as an argument.
    ///
    /// # Example
    /// ```
    /// use hookmap::*;
    /// let hotkey = Hotkey::new();
    /// hotkey.bind_mouse_cursor().on_move(|event| {
    ///     println!("Current mouse cursor position(x, y): {:?}", event);
    /// });
    /// ```
    pub fn on_move<F>(&self, callback: F)
    where
        F: Fn(MouseCursorEvent) + Send + Sync + 'static,
    {
        let handler = MouseEventHandler::builder()
            .modifier_keys(Arc::clone(&self.modifier_keys))
            .native_event_operation(self.native_event_operation)
            .action(callback.into())
            .build();
        self.register
            .upgrade()
            .unwrap()
            .borrow_mut()
            .register_cursor_event_handler(handler);
    }

    /// Disables and blocks mouse move events.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::*;
    /// let hotkey = Hotkey::new();
    /// hotkey.bind_mouse_cursor().disable();
    /// ```
    ///
    pub fn disable(&self) {
        let handler = MouseEventHandler::builder()
            .modifier_keys(Arc::clone(&self.modifier_keys))
            .native_event_operation(NativeEventOperation::Block)
            .action(Action::Noop)
            .build();
        self.register
            .upgrade()
            .unwrap()
            .borrow_mut()
            .register_cursor_event_handler(handler);
    }
}

/// A struct for registering handlers for the mouse wheel.
#[derive(TypedBuilder)]
pub struct MouseWheelHotkeyEntry {
    register: Weak<RefCell<Register>>,

    #[builder(default)]
    modifier_keys: Arc<ModifierKeys>,

    #[builder(default)]
    native_event_operation: NativeEventOperation,
}

impl MouseWheelHotkeyEntry {
    /// Registers a handler called when the mouse wheel is rotated.
    ///
    /// # Arguments
    ///
    /// * `callback` - A function that takes a rotation speed of the mouse
    /// wheel as an argument.
    ///
    /// # Example
    /// ```
    /// use hookmap::*;
    /// let hotkey = Hotkey::new();
    /// hotkey.bind_mouse_wheel().on_rotate(|event| {
    ///     println!("Mouse wheel rotation speed: {}", event);
    /// });
    /// ```
    ///
    pub fn on_rotate<F>(&self, callback: F)
    where
        F: Fn(MouseWheelEvent) + Send + Sync + 'static,
    {
        let handler = MouseEventHandler::builder()
            .modifier_keys(Arc::clone(&self.modifier_keys))
            .native_event_operation(self.native_event_operation)
            .action(callback.into())
            .build();
        self.register
            .upgrade()
            .unwrap()
            .borrow_mut()
            .register_wheel_event_event_handler(handler);
    }

    /// Disables and blocks mouse wheel events.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::*;
    /// let hotkey = Hotkey::new();
    /// hotkey.bind_mouse_wheel().disable();
    /// ```
    ///
    pub fn disable(&self) {
        let handler = MouseEventHandler::builder()
            .modifier_keys(Arc::clone(&self.modifier_keys))
            .native_event_operation(NativeEventOperation::Block)
            .action(Action::Noop)
            .build();
        self.register
            .upgrade()
            .unwrap()
            .borrow_mut()
            .register_wheel_event_event_handler(handler);
    }
}

/// Register remapping information.
#[derive(TypedBuilder)]
pub struct RemapEntry {
    register: Weak<RefCell<Register>>,
    trigger: ButtonSet,

    #[builder(default)]
    modifier_keys: Arc<ModifierKeys>,
}

impl RemapEntry {
    /// Determines which key to remap to.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::*;
    /// let hotkey = Hotkey::new();
    /// hotkey.remap(Button::A).to(Button::B);
    /// ```
    ///
    pub fn to(&self, button: impl Into<ButtonSet>) {
        let remap_info = RemapInfo {
            modifier_keys: Arc::clone(&self.modifier_keys),
            trigger: self.trigger.clone(),
            target: button.into(),
        };
        self.register
            .upgrade()
            .unwrap()
            .borrow_mut()
            .register_remap(remap_info)
    }
}
