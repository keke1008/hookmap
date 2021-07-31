use hookmap_core::{EmulateButtonInput, EventBlock, Key, Mouse};
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

#[derive(Debug, Default)]
pub struct ModifierSet {
    pub(crate) keyboard: HashSet<Key>,
    pub(crate) mouse: HashSet<Mouse>,
}

impl ModifierSet {
    pub fn added_key(&self, key: Key) -> Self {
        let mut keyboard_handler = self.keyboard.clone();
        keyboard_handler.insert(key);
        Self {
            keyboard: keyboard_handler,
            mouse: self.mouse.clone(),
        }
    }

    pub fn added_mouse_button(&self, mouse_button: Mouse) -> Self {
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
    mouse: HashMap<Mouse, bool>,
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub(crate) struct ModifierButton<B: Eq + Hash + Debug> {
    pub(crate) button: B,
    pub(crate) event_block: EventBlock,
}

impl<B: Eq + Hash + Debug> ModifierButton<B> {
    fn new(button: B, event_block: EventBlock) -> Self {
        Self {
            button,
            event_block,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub(crate) struct ModifierButtonSet {
    pub(crate) keyboard: HashSet<ModifierButton<Key>>,
    pub(crate) mouse: HashSet<ModifierButton<Mouse>>,
}

impl ModifierButtonSet {
    pub(crate) fn add_keyboard(&mut self, key: Key, event_block: EventBlock) {
        self.keyboard.insert(ModifierButton::new(key, event_block));
    }

    pub(crate) fn add_mouse(&mut self, mouse: Mouse, event_block: EventBlock) {
        self.mouse.insert(ModifierButton::new(mouse, event_block));
    }
}
