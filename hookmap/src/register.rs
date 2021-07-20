use crate::{
    event::{Button, EventInfo},
    handler::Handler,
    modifier::Modifier,
};
use derive_new::new;
use hookmap_core::{keyboard::Key, mouse::MouseInput};
use std::sync::{Arc, Mutex};

#[derive(new, Debug)]
pub struct KeyboardRegister {
    key: Key,
    modifier: Modifier,
    handler: Arc<Mutex<Handler>>,
}

impl KeyboardRegister {
    pub fn on_press<F>(self, callback: F)
    where
        F: FnMut(EventInfo<()>) + Send + 'static,
    {
        self.handler
            .lock()
            .unwrap()
            .keyboard
            .on_press
            .get(self.key)
            .push(callback, self.modifier);
    }

    pub fn on_press_or_release<F>(self, callback: F)
    where
        F: FnMut(EventInfo<Button>) + Send + 'static,
    {
        self.handler
            .lock()
            .unwrap()
            .keyboard
            .on_press_or_release
            .get(self.key)
            .push(callback, self.modifier);
    }

    pub fn on_release<F>(self, callback: F)
    where
        F: FnMut(EventInfo<()>) + Send + 'static,
    {
        self.handler
            .lock()
            .unwrap()
            .keyboard
            .on_release
            .get(self.key)
            .push(callback, self.modifier);
    }
}

fn is_button(mouse: MouseInput) -> bool {
    mouse == MouseInput::LButton
        || mouse == MouseInput::RButton
        || mouse == MouseInput::MButton
        || mouse == MouseInput::SideButton1
        || mouse == MouseInput::SideButton2
}

#[derive(new, Debug)]
pub struct MouseRegister {
    mouse: MouseInput,
    modifier: Modifier,
    handler: Arc<Mutex<Handler>>,
}

impl MouseRegister {
    pub fn on_move<F>(self, callback: F)
    where
        F: FnMut(EventInfo<(i32, i32)>) + Send + 'static,
    {
        assert_eq!(self.mouse, MouseInput::Move);
        self.handler
            .lock()
            .unwrap()
            .mouse
            .cursor
            .push(callback, self.modifier);
    }

    pub fn on_press<F>(self, callback: F)
    where
        F: FnMut(EventInfo<()>) + Send + 'static,
    {
        assert!(is_button(self.mouse));
        self.handler
            .lock()
            .unwrap()
            .mouse
            .on_press
            .get(self.mouse)
            .push(callback, self.modifier);
    }

    pub fn on_press_or_release<F>(self, callback: F)
    where
        F: FnMut(EventInfo<Button>) + Send + 'static,
    {
        assert!(is_button(self.mouse));
        self.handler
            .lock()
            .unwrap()
            .mouse
            .on_press_or_release
            .get(self.mouse)
            .push(callback, self.modifier);
    }

    pub fn on_release<F>(self, callback: F)
    where
        F: FnMut(EventInfo<()>) + Send + 'static,
    {
        assert!(is_button(self.mouse));
        self.handler
            .lock()
            .unwrap()
            .mouse
            .on_release
            .get(self.mouse)
            .push(callback, self.modifier);
    }

    pub fn on_rotate<F>(self, callback: F)
    where
        F: FnMut(EventInfo<i32>) + Send + 'static,
    {
        assert_eq!(self.mouse, MouseInput::Wheel);
        self.handler
            .lock()
            .unwrap()
            .mouse
            .wheel
            .push(callback, self.modifier);
    }
}
