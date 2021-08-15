use super::{
    button::{ButtonWithState, DownCastableButtonState},
    cond::Conditions,
    SetEventBlock,
};
use crate::{
    handler::{ButtonCallbackMap, ButtonEventCallback, MouseEventCallBack},
    interface::All,
};
use hookmap_core::{Button, ButtonEvent, ButtonInput, ButtonState, EventBlock};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
    sync::Arc,
};

/// A struct for registering handlers for the buttons.
pub struct ButtonRegister {
    inner: ButtonRegisterInner,
    event_block: EventBlock,
}

impl ButtonRegister {
    pub(crate) fn new(
        handler: Weak<RefCell<ButtonEventCallback>>,
        conditions: Arc<Conditions>,
        button: impl DownCastableButtonState,
        event_block: EventBlock,
    ) -> Self {
        let button = Box::new(button).into_button_with_state();
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
    /// hook.bind(Button::A).on_press_or_release(|event| {
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
    pub fn like(self, button: Button) {
        self.block()
            .on_press(move |_| button.press())
            .on_release(move |_| button.release());
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

pub struct ButtonRegisterInner {
    handler: Weak<RefCell<ButtonEventCallback>>,
    conditions: Arc<Conditions>,
    button: ButtonWithState,
}

type ButtonCallback = Arc<dyn Fn(ButtonEvent) + Send + Sync>;

impl ButtonRegisterInner {
    fn bind(&self, map: &mut ButtonCallbackMap, callback: ButtonCallback, event_block: EventBlock) {
        self.button.iter_buttons().for_each(move |&button| {
            map.get_mut(button).push(
                Arc::clone(&callback),
                Arc::clone(&self.conditions),
                event_block,
            )
        });
    }

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
        handler: Weak<RefCell<ButtonEventCallback>>,
        conditions: Arc<Conditions>,
        button: ButtonWithState,
    ) -> Self {
        Self {
            handler,
            conditions,
            button,
        }
    }

    fn on_press(&self, callback: ButtonCallback, event_block: EventBlock) {
        let callback_map = self.upgrade_handler();
        let mut callback_map = &mut callback_map.borrow_mut().on_press;
        let callback = self.generate_callback(callback, All::is_pressed);
        self.bind(&mut callback_map, callback, event_block);
    }

    fn on_release(&self, callback: ButtonCallback, event_block: EventBlock) {
        let callback_map = self.upgrade_handler();
        let mut callback_map = &mut callback_map.borrow_mut().on_release;
        let callback = self.generate_callback(callback, All::is_released);
        self.bind(&mut callback_map, callback, event_block);
    }

    fn upgrade_handler(&self) -> Rc<RefCell<ButtonEventCallback>> {
        self.handler.upgrade().unwrap()
    }
}

/// A struct for registering handlers for the mouse cursor.
#[derive(Debug)]
pub struct MouseCursorRegister {
    handler: Weak<RefCell<MouseEventCallBack<(i32, i32)>>>,
    conditions: Arc<Conditions>,
    event_block: EventBlock,
}

impl MouseCursorRegister {
    pub(crate) fn new(
        handler: Weak<RefCell<MouseEventCallBack<(i32, i32)>>>,
        conditions: Arc<Conditions>,
        event_block: EventBlock,
    ) -> Self {
        Self {
            handler,
            conditions,
            event_block,
        }
    }

    /// Registers a handler called when the mouse cursor is moved.
    ///
    /// # Arguments
    ///
    /// * `callback` - A function that takes a absolute postion of the mouse cursor as an argument.
    ///
    /// # Example
    /// ```
    /// use hookmap::{Hook, SelectHandleTarget};
    /// let hook = Hook::new();
    /// hook.bind_mouse_cursor().on_move(|event| {
    ///     println!("Current mouse cursor position(x, y): {:?}", event);
    /// });
    /// ```
    pub fn on_move<F>(&self, callback: F)
    where
        F: Fn((i32, i32)) + Send + Sync + 'static,
    {
        self.handler.upgrade().unwrap().borrow_mut().push(
            Arc::new(callback),
            Arc::clone(&self.conditions),
            self.event_block,
        );
    }
}

impl SetEventBlock for MouseCursorRegister {
    fn block(mut self) -> Self {
        self.event_block = EventBlock::Block;
        self
    }

    fn unblock(mut self) -> Self {
        self.event_block = EventBlock::Unblock;
        self
    }
}
/// A struct for registering handlers for the mouse wheel.
#[derive(Debug)]
pub struct MouseWheelRegister {
    handler: Weak<RefCell<MouseEventCallBack<i32>>>,
    conditions: Arc<Conditions>,
    event_block: EventBlock,
}

impl MouseWheelRegister {
    pub(crate) fn new(
        handler: Weak<RefCell<MouseEventCallBack<i32>>>,
        conditions: Arc<Conditions>,
        event_block: EventBlock,
    ) -> Self {
        Self {
            handler,
            conditions,
            event_block,
        }
    }

    /// Registers a handler called when the mouse wheel is rotated.
    ///
    /// # Arguments
    ///
    /// * `callback` - A function that takes a rotation speed of the mouse
    /// wheel as an argument.
    ///
    /// # Example
    /// ```
    /// use hookmap::{Hook, SelectHandleTarget};
    /// let hook = Hook::new();
    /// hook.bind_mouse_wheel().on_rotate(|event| {
    ///     println!("Mouse wheel rotation speed: {}", event);
    /// });
    /// ```
    ///
    pub fn on_rotate<F>(&self, callback: F)
    where
        F: Fn(i32) + Send + Sync + 'static,
    {
        self.handler.upgrade().unwrap().borrow_mut().push(
            Arc::new(callback),
            Arc::clone(&self.conditions),
            self.event_block,
        );
    }
}

impl SetEventBlock for MouseWheelRegister {
    fn block(mut self) -> Self {
        self.event_block = EventBlock::Block;
        self
    }

    fn unblock(mut self) -> Self {
        self.event_block = EventBlock::Unblock;
        self
    }
}
