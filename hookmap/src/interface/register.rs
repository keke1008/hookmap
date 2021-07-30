use crate::{
    event::EventInfo,
    handler::{ButtonHandler, HandlerVec},
    modifier::ModifierSet,
};
use hookmap_core::{ButtonAction, EmulateButtonInput, Key, Mouse};
use std::{cell::RefCell, hash::Hash, rc::Weak, sync::Arc};

pub struct ButtonRegister<B: Eq + Hash + Copy> {
    handler: Weak<RefCell<ButtonHandler<B>>>,
    modifier: Arc<ModifierSet>,
    button: B,
}

impl<B: Eq + Hash + Copy> ButtonRegister<B> {
    pub(crate) fn new(
        handler: Weak<RefCell<ButtonHandler<B>>>,
        modifier: Arc<ModifierSet>,
        button: B,
    ) -> Self {
        Self {
            handler,
            modifier,
            button,
        }
    }

    /// Registers a handler called when the specified button is pressed.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::{Hook, Key, SelectHandleTarget};
    /// let hook = Hook::new();
    /// hook.bind_key(Key::A).on_press(|_| println!("The A key is pressed"));
    /// ```
    ///
    pub fn on_press<F>(&self, callback: F)
    where
        F: FnMut(EventInfo<()>) + Send + 'static,
    {
        self.handler
            .upgrade()
            .unwrap()
            .borrow_mut()
            .on_press
            .get(self.button)
            .push(Box::new(callback), Arc::clone(&self.modifier));
    }

    /// Registers a handler called when the specified button is pressed or released.
    ///
    /// # Arguments
    ///
    /// * `callback` - A function that takes `EventInfo` containing whether the specified key
    /// is pressed or released as an argument.
    ///
    /// # Example
    /// ```
    /// use hookmap::{ButtonAction, Hook, Key, SelectHandleTarget};
    /// let hook = Hook::new();
    /// hook.bind_key(Key::A).on_press_or_release(|event| {
    ///     match event.info {
    ///         ButtonAction::Press => println!("The A key is pressed"),
    ///         ButtonAction::Release => println!("The A key is released"),
    ///     };
    /// });
    /// ```
    ///
    pub fn on_press_or_release<F>(&self, callback: F)
    where
        F: FnMut(EventInfo<ButtonAction>) + Send + 'static,
    {
        self.handler
            .upgrade()
            .unwrap()
            .borrow_mut()
            .on_press_or_release
            .get(self.button)
            .push(Box::new(callback), Arc::clone(&self.modifier));
    }

    /// Registers a handler called when the specified button is released.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::{Hook, Key, SelectHandleTarget};
    /// let hook = Hook::new();
    /// hook.bind_key(Key::A).on_release(|_| println!("The A key is released"));
    /// ```
    ///
    pub fn on_release<F>(&self, callback: F)
    where
        F: FnMut(EventInfo<()>) + Send + 'static,
    {
        self.handler
            .upgrade()
            .unwrap()
            .borrow_mut()
            .on_release
            .get(self.button)
            .push(Box::new(callback), Arc::clone(&self.modifier));
    }

    /// When the specific button is pressed, the key passed in the argument will be pressed.
    /// The same applies when the button is released.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::{Hook, Key, SelectHandleTarget};
    /// let hook = Hook::new();
    /// hook.bind_key(Key::H).as_key(Key::LeftArrow);
    /// ```
    ///
    pub fn as_key(&self, key: Key) {
        self.on_press(move |mut e| {
            key.press();
            e.block_event();
        });
        self.on_release(move |mut e| {
            key.release();
            e.block_event();
        });
    }

    /// When the specified button is pressed, the mouse button passed in the argument will be pressed.
    /// The same applies when the button is released.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::{Hook, Key, Mouse, SelectHandleTarget};
    /// let hook = Hook::new();
    /// hook.bind_key(Key::A).as_mouse(Mouse::LButton);
    /// ```
    ///
    pub fn as_mouse(&self, mouse: Mouse) {
        self.on_press(move |mut e| {
            mouse.press();
            e.block_event();
        });
        self.on_release(move |mut e| {
            mouse.press();
            e.block_event();
        });
    }
}

/// A struct for registering a handler for the mouse cursor.
#[derive(Debug)]
pub struct MouseCursorRegister {
    handler: Weak<RefCell<HandlerVec<(i32, i32)>>>,
    modifier: Arc<ModifierSet>,
}

impl MouseCursorRegister {
    pub(crate) fn new(
        handler: Weak<RefCell<HandlerVec<(i32, i32)>>>,
        modifier: Arc<ModifierSet>,
    ) -> Self {
        Self { handler, modifier }
    }

    /// Registers a handler called when the mouse cursor is moved.
    ///
    ///
    /// # Example
    /// ```
    /// use hookmap::{Hook, SelectHandleTarget};
    /// let hook = Hook::new();
    /// hook.bind_mouse_cursor().on_move(|event| {
    ///     println!("Current mouse cursor position(x, y): ({}, {})", event.info.0, event.info.1);
    /// });
    /// ```
    pub fn on_move<F>(&self, callback: F)
    where
        F: FnMut(EventInfo<(i32, i32)>) + Send + 'static,
    {
        self.handler
            .upgrade()
            .unwrap()
            .borrow_mut()
            .push(Box::new(callback), Arc::clone(&self.modifier));
    }
}

/// A struct for registering a handler for the mouse wheel.
#[derive(Debug)]
pub struct MouseWheelRegister {
    handler: Weak<RefCell<HandlerVec<i32>>>,
    modifier: Arc<ModifierSet>,
}

impl MouseWheelRegister {
    pub(crate) fn new(handler: Weak<RefCell<HandlerVec<i32>>>, modifier: Arc<ModifierSet>) -> Self {
        Self { handler, modifier }
    }

    /// Registers a handler called when the mouse wheel is rotated.
    ///
    /// # Arguments
    ///
    /// * `callback` - A function that takes `EventInfo` containing a rotation speed of the mouse
    /// wheel as an argument.
    ///
    /// # Example
    /// ```
    /// use hookmap::{Hook, SelectHandleTarget};
    /// let hook = Hook::new();
    /// hook.bind_mouse_wheel().on_rotate(|event| {
    ///     println!("Mouse wheel rotation speed: {}", event.info);
    /// });
    /// ```
    ///
    pub fn on_rotate<F>(&self, callback: F)
    where
        F: FnMut(EventInfo<i32>) + Send + 'static,
    {
        self.handler
            .upgrade()
            .unwrap()
            .borrow_mut()
            .push(Box::new(callback), Arc::clone(&self.modifier));
    }
}
