use super::super::{
    button::{ButtonWithState, ToButtonWithState},
    cond::Conditions,
    SetEventBlock,
};
use crate::{
    handler::{Handler, Register as HandlerRegister},
    interface::All,
};
use hookmap_core::{ButtonEvent, ButtonInput, ButtonState, EventBlock};
use std::{fmt::Debug, rc::Weak, sync::Arc};

type ButtonCallback = Arc<dyn Fn(ButtonEvent) + Send + Sync>;

/// A struct for registering handlers for the buttons.
pub struct ButtonRegister {
    inner: ButtonRegisterInner,
    event_block: EventBlock,
}

impl ButtonRegister {
    pub(crate) fn new(
        handler: Weak<HandlerRegister>,
        conditions: Arc<Conditions>,
        button: impl ToButtonWithState,
        event_block: EventBlock,
    ) -> Self {
        let button = button.to_button_with_state();
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
    /// hook.bind(&Button::A).on_press(|_| println!("The A key is pressed"));
    /// ```
    ///
    pub fn on_press<F>(self, callback: F) -> Self
    where
        F: Fn(ButtonEvent) + Send + Sync + 'static,
    {
        self.inner.on_press(Arc::new(callback), self.event_block);
        self
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
    /// hook.bind(&Button::A).on_press_or_release(|event| {
    ///     match event.action {
    ///         ButtonAction::Press => println!("The A key is pressed"),
    ///         ButtonAction::Release => println!("The A key is released"),
    ///     };
    /// });
    /// ```
    ///
    pub fn on_press_or_release<F>(self, callback: F) -> Self
    where
        F: Fn(ButtonEvent) + Send + Sync + 'static,
    {
        let callback: ButtonCallback = Arc::new(callback);
        self.inner.on_press(Arc::clone(&callback), self.event_block);
        self.inner.on_release(callback, self.event_block);
        self
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
    /// hook.bind(&Button::A).on_release(|_| println!("The A key is released"));
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
    /// hook.bind(&Button::H).like(&Button::LeftArrow);
    /// ```
    ///
    pub fn like<B>(self, button: &B)
    where
        B: ButtonInput + Clone + Send + Sync + 'static,
    {
        let this = {
            let button = button.clone();
            self.block().on_press(move |_| button.press())
        };
        let button = button.clone();
        this.on_release(move |_| button.release());
    }

    /// Disables the button and blocks the event.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::{Hook, Button, SelectHandleTarget};
    /// let hook = Hook::new();
    /// hook.bind(&Button::A).disable();
    /// ```
    pub fn disable(self) -> Self {
        self.block().on_press_or_release(|_| {})
    }
}

impl SetEventBlock for ButtonRegister {
    fn block(mut self) -> Self {
        self.event_block = EventBlock::Block;
        self
    }

    fn unblock(mut self) -> Self {
        self.event_block = EventBlock::Unblock;
        self
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

pub struct ButtonRegisterInner {
    handler_register: Weak<HandlerRegister>,
    conditions: Arc<Conditions>,
    button: ButtonWithState,
}

impl ButtonRegisterInner {
    fn generate_callback(
        &self,
        callback: ButtonCallback,
        predict: fn(&All) -> bool,
    ) -> ButtonCallback {
        if let ButtonWithState::All(ref all) = self.button {
            let all = all.clone();
            let callback = move |e| {
                if predict(&all) {
                    callback(e)
                }
            };
            Arc::new(callback)
        } else {
            callback
        }
    }

    fn new(
        handler_register: Weak<HandlerRegister>,
        conditions: Arc<Conditions>,
        button: ButtonWithState,
    ) -> Self {
        Self {
            handler_register,
            conditions,
            button,
        }
    }

    fn on_press(&self, callback: ButtonCallback, event_block: EventBlock) {
        let callback = self.generate_callback(callback, All::is_pressed);
        let handler = Arc::new(Handler::new(
            callback,
            Arc::clone(&self.conditions),
            event_block,
        ));
        let handler_register = self.handler_register.upgrade().unwrap();
        self.button.iter_buttons().for_each(move |&button| {
            handler_register.register_button_on_press(button, Arc::clone(&handler))
        });
    }

    fn on_release(&self, callback: ButtonCallback, event_block: EventBlock) {
        let callback = self.generate_callback(callback, All::is_released);
        let handler = Arc::new(Handler::new(
            callback,
            Arc::clone(&self.conditions),
            event_block,
        ));
        let handler_register = self.handler_register.upgrade().unwrap();
        self.button.iter_buttons().for_each(|&button| {
            handler_register.register_button_on_release(button, Arc::clone(&handler))
        });
    }
}
