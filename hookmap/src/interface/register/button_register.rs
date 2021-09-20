use super::super::{
    button::{ButtonInput, ButtonSet, ToButtonSet},
    cond::Conditions,
};
use crate::handler::{Callback, Register as HandlerRegister};
use hookmap_core::{ButtonEvent, EventBlock};
use std::{borrow::Borrow, fmt::Debug, rc::Weak, sync::Arc};

pub struct ButtonRegisterInner {
    handler_register: Weak<HandlerRegister>,
    conditions: Arc<Conditions>,
    button: ButtonSet,
}

impl ButtonRegisterInner {
    fn new(
        handler_register: Weak<HandlerRegister>,
        conditions: Arc<Conditions>,
        button: ButtonSet,
    ) -> Self {
        Self {
            handler_register,
            conditions,
            button,
        }
    }

    fn on_press(&self, callback: Callback<ButtonEvent>, event_block: EventBlock) {
        let handler_register = self.handler_register.upgrade().unwrap();
        handler_register.register_button_on_press(
            &self.button,
            callback,
            self.conditions.clone(),
            event_block,
        );
    }

    fn on_release(&self, callback: Callback<ButtonEvent>, event_block: EventBlock) {
        let handler_register = self.handler_register.upgrade().unwrap();
        handler_register.register_button_on_release(
            &self.button,
            callback,
            self.conditions.clone(),
            event_block,
        );
    }
}

/// A struct for registering handlers for the buttons.
pub struct ButtonRegister {
    inner: ButtonRegisterInner,
    event_block: EventBlock,
}

impl ButtonRegister {
    pub(crate) fn new(
        handler: Weak<HandlerRegister>,
        conditions: Arc<Conditions>,
        button: impl ToButtonSet,
        event_block: EventBlock,
    ) -> Self {
        let button = button.to_button_set();
        Self {
            inner: ButtonRegisterInner::new(handler, conditions, button),
            event_block,
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
        self.inner.on_press(Arc::new(callback), self.event_block);
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
        let callback: Callback<ButtonEvent> = Arc::new(callback);
        self.inner.on_press(Arc::clone(&callback), self.event_block);
        self.inner.on_release(callback, self.event_block);
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
    pub fn on_release<F>(self, callback: F) -> Self
    where
        F: Fn(ButtonEvent) + Send + Sync + 'static,
    {
        self.inner.on_release(Arc::new(callback), self.event_block);
        self
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
    pub fn like<B, R>(self, button: B)
    where
        B: Borrow<R>,
        R: ButtonInput + Clone + Send + Sync + 'static,
    {
        let button = button.borrow().clone();
        {
            let button = button.clone();
            self.inner
                .on_press(Arc::new(move |_| button.press()), EventBlock::Block);
        }
        self.inner
            .on_release(Arc::new(move |_| button.release()), EventBlock::Block);
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
        let callback: Callback<ButtonEvent> = Arc::new(|_| {});
        self.inner
            .on_press(Arc::clone(&callback), EventBlock::Block);
        self.inner.on_release(callback, EventBlock::Block);
    }
}

impl Debug for ButtonRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ButtonRegister")
            .field("button", &self.inner.button)
            .field("conditions", &self.inner.conditions)
            .field("event_block", &self.event_block)
            .finish()
    }
}
