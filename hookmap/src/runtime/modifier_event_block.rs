use crate::modifier::ModifierButtonSet;
use hookmap_core::{EventBlock, Key, Mouse};
use std::{collections::HashMap, fmt::Debug};

#[derive(Debug)]
pub(crate) struct ModifierEventBlock {
    pub(crate) keyboard: HashMap<Key, EventBlock>,
    pub(crate) mouse: HashMap<Mouse, EventBlock>,
}

impl From<ModifierButtonSet> for ModifierEventBlock {
    fn from(modifier_button_set: ModifierButtonSet) -> Self {
        let keyboard = modifier_button_set
            .keyboard
            .iter()
            .map(|modifier| (modifier.button, modifier.event_block))
            .collect();
        let mouse = modifier_button_set
            .mouse
            .iter()
            .map(|modifier| (modifier.button, modifier.event_block))
            .collect();
        Self { keyboard, mouse }
    }
}
