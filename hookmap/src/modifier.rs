use hookmap_core::{EmulateKeyboardInput, EmulateMouseInput, Key, MouseInput};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Default, Clone)]
pub struct Modifier {
    pub(crate) keyboard: HashSet<Key>,
    pub(crate) mouse: HashSet<MouseInput>,
}

impl Modifier {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_key(&mut self, key: Key) {
        self.keyboard.insert(key);
    }

    pub fn added_key(&self, key: Key) -> Self {
        let mut keyboard_hander = self.keyboard.clone();
        keyboard_hander.insert(key);
        Self {
            keyboard: keyboard_hander,
            mouse: self.mouse.clone(),
        }
    }

    pub fn added_mouse_button(&self, mouse_button: MouseInput) -> Self {
        let mut mouse_handler = self.mouse.clone();
        mouse_handler.insert(mouse_button);
        Self {
            keyboard: self.keyboard.clone(),
            mouse: mouse_handler,
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct ModifierChecker {
    keyboard: HashMap<Key, bool>,
    mouse: HashMap<MouseInput, bool>,
}

impl ModifierChecker {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn check(&mut self, modifier: &Modifier) -> bool {
        self.check_keyboard(modifier) && self.check_mouse(modifier)
    }

    fn check_keyboard(&mut self, modifier: &Modifier) -> bool {
        modifier.keyboard.iter().all(|key| {
            *self
                .keyboard
                .entry(*key)
                .or_insert_with(|| key.is_pressed())
        })
    }

    fn check_mouse(&mut self, modifier: &Modifier) -> bool {
        modifier.mouse.iter().all(|mouse| {
            *self
                .mouse
                .entry(*mouse)
                .or_insert_with(|| mouse.is_pressed())
        })
    }
}
