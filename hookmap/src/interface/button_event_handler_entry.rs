use super::hotkey_info::PartialHotkeyInfo;
use crate::button::ButtonInput;
use crate::hotkey::TriggerAction;
use crate::runtime::Register;
use hookmap_core::{ButtonEvent, EventBlock};
use std::cell::RefCell;
use std::{borrow::Borrow, rc::Weak};

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
    /// use hookmap::{Hook, Button, SelectHandleTarget};
    /// let hook = Hook::new();
    /// hook.bind(Button::A).on_press(|_| println!("The A key is pressed"));
    /// ```
    ///
    pub fn on_press<F>(&self, callback: F)
    where
        F: Fn(ButtonEvent) + Send + Sync + 'static,
    {
        let hotkey = self
            .partial_hotkey
            .clone()
            .build_hotkey_info(TriggerAction::Press, callback.into());
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
    /// use hookmap::{ButtonAction, Button, Hook, SelectHandleTarget};
    /// let hook = Hook::new();
    /// hook.bind(Button::A).on_press_or_release(|event| {
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
        let hotkey = self
            .partial_hotkey
            .clone()
            .build_hotkey_info(TriggerAction::PressOrRelease, callback.into());
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
    /// use hookmap::{Hook, Button, SelectHandleTarget};
    /// let hook = Hook::new();
    /// hook.bind(Button::A).on_release(|_| println!("The A key is released"));
    /// ```
    ///
    pub fn on_release<F>(&self, callback: F)
    where
        F: Fn(ButtonEvent) + Send + Sync + 'static,
    {
        let hotkey = self
            .partial_hotkey
            .clone()
            .build_hotkey_info(TriggerAction::Release, callback.into());
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
    /// use hookmap::{Hook, Button, SelectHandleTarget};
    /// let hook = Hook::new();
    /// hook.bind(Button::H).like(Button::LeftArrow);
    /// ```
    ///
    pub fn like<B, R>(&self, button: B)
    where
        B: Borrow<R>,
        R: ButtonInput + Clone + Send + Sync + 'static,
    {
        let register = self.register.upgrade().unwrap();
        let mut partial_hotkey = self.partial_hotkey.clone();
        partial_hotkey.event_block = EventBlock::Block;
        {
            let button = button.borrow().clone();
            register.borrow_mut().register_hotkey(
                partial_hotkey
                    .clone()
                    .build_hotkey_info(TriggerAction::Press, (move |_| button.press()).into()),
            );
        }
        let button = button.borrow().clone();
        register.borrow_mut().register_hotkey(
            partial_hotkey
                .build_hotkey_info(TriggerAction::Release, (move |_| button.release()).into()),
        );
    }

    /// Disables the button and blocks the event.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::{Hook, Button, SelectHandleTarget};
    /// let hook = Hook::new();
    /// hook.bind(Button::A).disable();
    /// ```
    pub fn disable(&self) {
        let mut partial_hotkey = self.partial_hotkey.clone();
        partial_hotkey.event_block = EventBlock::Block;
        self.register
            .upgrade()
            .unwrap()
            .borrow_mut()
            .register_hotkey(
                partial_hotkey.build_hotkey_info(TriggerAction::PressOrRelease, (|_| {}).into()),
            );
    }
}
