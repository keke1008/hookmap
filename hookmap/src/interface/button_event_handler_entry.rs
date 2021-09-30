use super::hotkey_info::PartialHotkeyInfo;
use crate::{
    button::{ButtonInput, ButtonSet},
    hotkey::{Action, HotkeyAction},
    runtime::Register,
};
use hookmap_core::{ButtonEvent, EventBlock};
use std::{cell::RefCell, rc::Weak};

/// A struct for registering handlers for the buttons.
pub struct ButtonEventHandlerEntry {
    register: Weak<RefCell<Register>>,
    partial_hotkey: PartialHotkeyInfo,
}

impl ButtonEventHandlerEntry {
    pub(super) fn new(
        register: Weak<RefCell<Register>>,
        partial_hotkey: PartialHotkeyInfo,
    ) -> Self {
        Self {
            register,
            partial_hotkey,
        }
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
        let action = HotkeyAction::Press(callback.into());
        let hotkey = self.partial_hotkey.clone().build_hotkey_info(action);
        self.register
            .upgrade()
            .unwrap()
            .borrow_mut()
            .register_hotkey(hotkey);
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
        let action = HotkeyAction::PressOrRelease(callback.into());
        let hotkey = self.partial_hotkey.clone().build_hotkey_info(action);
        self.register
            .upgrade()
            .unwrap()
            .borrow_mut()
            .register_hotkey(hotkey);
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
        let action = HotkeyAction::Release(callback.into());
        let hotkey = self.partial_hotkey.clone().build_hotkey_info(action);
        self.register
            .upgrade()
            .unwrap()
            .borrow_mut()
            .register_hotkey(hotkey);
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
    /// use hokmap::*;
    /// let hotkey = Hotkey::new();
    /// hotkey.bind(Button::A).on_press_and_release(
    ///     |_| println!("The A key is pressed"),
    ///     |_| println!("The A key is released"),
    /// );
    /// ```
    ///
    pub fn on_press_and_release<F>(self, on_press: F, on_release: F)
    where
        F: Fn(ButtonEvent) + Send + Sync + 'static,
    {
        let action = HotkeyAction::PressAndRelease {
            on_press: on_press.into(),
            on_release: on_release.into(),
        };
        let hotkey = self.partial_hotkey.build_hotkey_info(action);
        self.register
            .upgrade()
            .unwrap()
            .borrow_mut()
            .register_hotkey(hotkey);
    }

    /// When the specified button is pressed, the key passed in the argument will be pressed.
    /// The same applies when the button is released.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::*;
    /// let hotkey = Hotkey::new();
    /// hotkey.bind(Button::H).like(Button::LeftArrow);
    /// ```
    ///
    pub fn like<T>(&self, button: T)
    where
        T: Into<ButtonSet> + Send + Sync,
    {
        let button = button.into();
        let partial_hotkey = {
            let mut partial_hotkey = self.partial_hotkey.clone();
            partial_hotkey.event_block = EventBlock::Block;
            partial_hotkey
        };
        let on_press = {
            let button = button.clone();
            (move |_| button.press()).into()
        };
        let on_release = (move |_| button.release()).into();
        let action = HotkeyAction::PressAndRelease {
            on_press,
            on_release,
        };
        let hotkey = partial_hotkey.build_hotkey_info(action);
        self.register
            .upgrade()
            .unwrap()
            .borrow_mut()
            .register_hotkey(hotkey);
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
        let partial_hotkey = {
            let mut partial_hotkey = self.partial_hotkey.clone();
            partial_hotkey.event_block = EventBlock::Block;
            partial_hotkey
        };
        let action = HotkeyAction::PressOrRelease(Action::Noop);
        let hotkey = partial_hotkey.build_hotkey_info(action);
        self.register
            .upgrade()
            .unwrap()
            .borrow_mut()
            .register_hotkey(hotkey);
    }
}
