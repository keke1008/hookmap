use crate::modifier::ModifierButtonSet;
use hookmap_core::{Key, Mouse};
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub(crate) struct AloneModifierMap<B: Eq + Hash>(HashMap<B, bool>);

impl<B: Eq + Hash> AloneModifierMap<B> {
    /// Called when a button is pressed.
    pub(crate) fn press_event(&mut self, button: B) {
        match self.0.get_mut(&button) {
            Some(is_alone) => *is_alone = true,
            None => self.0.values_mut().for_each(|is_alone| *is_alone = false),
        }
    }

    /// Called when a button is released.
    pub(crate) fn is_alone(&self, button: B) -> bool {
        matches!(self.0.get(&button), Some(true))
    }
}

#[derive(Debug)]
pub(crate) struct AloneModifierList {
    pub(crate) keyboard_alone_modifiers: AloneModifierMap<Key>,
    pub(crate) mouse_alone_modifiers: AloneModifierMap<Mouse>,
}

impl From<ModifierButtonSet> for AloneModifierList {
    fn from(modifier_button_set: ModifierButtonSet) -> Self {
        let keyboard = modifier_button_set
            .keyboard
            .iter()
            .map(|modifier| (modifier.button, false))
            .collect();
        let mouse = modifier_button_set
            .mouse
            .iter()
            .map(|modifier| (modifier.button, false))
            .collect();
        Self {
            keyboard_alone_modifiers: AloneModifierMap(keyboard),
            mouse_alone_modifiers: AloneModifierMap(mouse),
        }
    }
}
