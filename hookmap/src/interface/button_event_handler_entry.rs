use crate::{
    hotkey::{Action, HotkeyAction, HotkeyInfo, ModifierKeys, Trigger},
    runtime::Register,
};
use hookmap_core::{ButtonEvent, EventBlock};
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
    event_block: EventBlock,
}

impl ButtonEventHandlerEntry {
    fn register_hotkey(&self, action: HotkeyAction) {
        let hotkey_info = HotkeyInfo::builder()
            .trigger(self.trigger.clone())
            .modifier_keys(Arc::clone(&self.modifier_keys))
            .event_block(self.event_block)
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
            .event_block(self.event_block)
            .action(HotkeyAction::PressOrRelease(Action::Noop))
            .build();
        self.register
            .upgrade()
            .unwrap()
            .borrow_mut()
            .register_hotkey(hotkey_info);
    }
}
