use hookmap_core::{EmulateKeyboardInput, EmulateMouseInput, EventBlock, Key, MouseInput};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Default, Clone)]
pub struct ModifierSet {
    pub(crate) keyboard: HashSet<Key>,
    pub(crate) mouse: HashSet<MouseInput>,
}

impl ModifierSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn added_key(&self, key: Key) -> Self {
        let mut keyboard_handler = self.keyboard.clone();
        keyboard_handler.insert(key);
        Self {
            keyboard: keyboard_handler,
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

    pub(crate) fn check(&mut self, modifier: &ModifierSet) -> bool {
        self.check_keyboard(modifier) && self.check_mouse(modifier)
    }

    fn check_keyboard(&mut self, modifier: &ModifierSet) -> bool {
        modifier.keyboard.iter().all(|key| {
            *self
                .keyboard
                .entry(*key)
                .or_insert_with(|| key.is_pressed())
        })
    }

    fn check_mouse(&mut self, modifier: &ModifierSet) -> bool {
        modifier.mouse.iter().all(|mouse| {
            *self
                .mouse
                .entry(*mouse)
                .or_insert_with(|| mouse.is_pressed())
        })
    }
}

#[derive(Debug, Default)]
pub(crate) struct ModifierEventBlock {
    pub(crate) keyboard: HashMap<Key, EventBlock>,
    pub(crate) mouse: HashMap<MouseInput, EventBlock>,
}
