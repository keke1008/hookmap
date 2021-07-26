use crate::{
    event::{Button, EventInfo},
    handler::Handler,
    modifier::ModifierSet,
};
use derive_new::new;
use hookmap_core::{EmulateKeyboardInput, EmulateMouseInput, Key, MouseInput};
use std::{cell::RefCell, rc::Weak, sync::Arc};

/// A struct to register keyboard handlers.
#[derive(new, Debug)]
pub struct KeyboardRegister {
    handler: Weak<RefCell<Handler>>,
    modifier: Arc<ModifierSet>,
    key: Key,
}

impl KeyboardRegister {
    /// Registers a handler called when the specified key is pressed.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::{Hook, Key};
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
            .keyboard
            .on_press
            .get(self.key)
            .push(callback, Arc::clone(&self.modifier));
    }

    /// Registers a handler called when the specified key is pressed or released.
    ///
    /// # Arguments
    ///
    /// * `callback` - A function that takes `EventInfo` containing whether the specified key
    /// is pressed or released as an argument.
    ///
    /// # Example
    /// ```
    /// use hookmap::{Button, Hook, Key};
    /// let hook = Hook::new();
    /// hook.bind_key(Key::A).on_press_or_release(|event| {
    ///     match event.info {
    ///         Button::Press => println!("The A key is pressed"),
    ///         Button::Release => println!("The A key is released"),
    ///     };
    /// });
    /// ```
    ///
    pub fn on_press_or_release<F>(&self, callback: F)
    where
        F: FnMut(EventInfo<Button>) + Send + 'static,
    {
        self.handler
            .upgrade()
            .unwrap()
            .borrow_mut()
            .keyboard
            .on_press_or_release
            .get(self.key)
            .push(callback, Arc::clone(&self.modifier));
    }

    /// Registers a handler called when the specified key is released.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::{Hook, Key};
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
            .keyboard
            .on_release
            .get(self.key)
            .push(callback, Arc::clone(&self.modifier));
    }

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

    pub fn as_mouse(&self, mouse: MouseInput) {
        is_button(mouse);
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

fn is_button(mouse: MouseInput) -> bool {
    mouse == MouseInput::LButton
        || mouse == MouseInput::RButton
        || mouse == MouseInput::MButton
        || mouse == MouseInput::SideButton1
        || mouse == MouseInput::SideButton2
}

/// A struct to register mouse handlers.
#[derive(new, Debug)]
pub struct MouseRegister {
    handler: Weak<RefCell<Handler>>,
    modifier: Arc<ModifierSet>,
    mouse: MouseInput,
}

impl MouseRegister {
    /// Registers a handler called when the mouse cursor is moved.
    ///
    /// # Panics
    ///
    /// Panics if `MouseInput::Move` is not specified.
    ///
    /// # Example
    /// ```
    /// use hookmap::{Hook, MouseInput};
    /// let hook = Hook::new();
    /// hook.bind_mouse(MouseInput::Move).on_move(|event| {
    ///     println!("Current mouse cursor position(x, y): ({}, {})", event.info.0, event.info.1);
    /// });
    /// ```
    pub fn on_move<F>(&self, callback: F)
    where
        F: FnMut(EventInfo<(i32, i32)>) + Send + 'static,
    {
        assert_eq!(self.mouse, MouseInput::Move);
        self.handler
            .upgrade()
            .unwrap()
            .borrow_mut()
            .mouse
            .cursor
            .push(callback, Arc::clone(&self.modifier));
    }

    /// Registers a handler called when the specified mouse button is pressed.
    ///
    /// # Panics
    ///
    /// Panics if no mouse button is specified.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::{Hook, MouseInput};
    /// let hook = Hook::new();
    /// hook.bind_mouse(MouseInput::LButton)
    ///     .on_press(|_| println!("The left mouse button is pressed"));
    /// ```
    ///
    pub fn on_press<F>(&self, callback: F)
    where
        F: FnMut(EventInfo<()>) + Send + 'static,
    {
        assert!(is_button(self.mouse));
        self.handler
            .upgrade()
            .unwrap()
            .borrow_mut()
            .mouse
            .on_press
            .get(self.mouse)
            .push(callback, Arc::clone(&self.modifier));
    }

    /// Registers a handler called when the specified mouse button is pressed or released.
    ///
    /// # Arguments
    ///
    /// * `callback` - A function that takes `EventInfo` containing whether the specified mouse
    /// button is pressed or released as an argument.
    ///
    /// # Panics
    ///
    /// Panics if no mouse button is specified.
    ///
    /// # Example
    /// ```
    /// use hookmap::{Button, Hook, MouseInput};
    /// let hook = Hook::new();
    /// hook.bind_mouse(MouseInput::LButton).on_press_or_release(|event| {
    ///     match event.info {
    ///         Button::Press => println!("The left mouse button is pressed"),
    ///         Button::Release => println!("The left mouse button is released"),
    ///     };
    /// });
    /// ```
    ///
    pub fn on_press_or_release<F>(&self, callback: F)
    where
        F: FnMut(EventInfo<Button>) + Send + 'static,
    {
        assert!(is_button(self.mouse));
        self.handler
            .upgrade()
            .unwrap()
            .borrow_mut()
            .mouse
            .on_press_or_release
            .get(self.mouse)
            .push(callback, Arc::clone(&self.modifier));
    }

    /// Registers a handler called when the specified mouse button is released.
    ///
    /// # Panics
    ///
    /// Panics if no mouse button is specified.
    ///
    /// # Example
    /// ```
    /// use hookmap::{Hook, MouseInput};
    /// let hook = Hook::new();
    /// hook.bind_mouse(MouseInput::LButton)
    ///     .on_release(|_| println!("The left mouse button is released"));
    /// ```
    ///
    pub fn on_release<F>(&self, callback: F)
    where
        F: FnMut(EventInfo<()>) + Send + 'static,
    {
        assert!(is_button(self.mouse));
        self.handler
            .upgrade()
            .unwrap()
            .borrow_mut()
            .mouse
            .on_release
            .get(self.mouse)
            .push(callback, Arc::clone(&self.modifier));
    }

    /// Registers a handler called when the mouse wheel is rotated.
    ///
    /// # Arguments
    ///
    /// * `callback` - A function that takes `EventInfo` containing a rotation speed of the mouse
    /// wheel as an argument.
    ///
    /// # Panics
    ///
    /// Panics if `MouseInput::Wheel` is not specified.
    ///
    /// # Example
    /// ```
    /// use hookmap::{Hook, MouseInput};
    /// let hook = Hook::new();
    /// hook.bind_mouse(MouseInput::Wheel).on_rotate(|event| {
    ///     println!("Mouse wheel rotation speed: {}", event.info);
    /// });
    /// ```
    ///
    pub fn on_rotate<F>(&self, callback: F)
    where
        F: FnMut(EventInfo<i32>) + Send + 'static,
    {
        assert_eq!(self.mouse, MouseInput::Wheel);
        self.handler
            .upgrade()
            .unwrap()
            .borrow_mut()
            .mouse
            .wheel
            .push(callback, Arc::clone(&self.modifier));
    }

    pub fn as_key(&self, key: Key) {
        match self.mouse {
            MouseInput::Move => self.on_move(move |_| key.click()),
            MouseInput::Wheel => self.on_rotate(move |_| key.click()),
            _ => {
                self.on_press(move |mut e| {
                    key.press();
                    e.block_event();
                });
                self.on_release(move |mut e| {
                    key.press();
                    e.block_event();
                });
            }
        }
    }

    pub fn as_mouse(&self, mouse: MouseInput) {
        is_button(mouse);
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
