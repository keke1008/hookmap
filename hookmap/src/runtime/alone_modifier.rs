use crate::modifier::ModifierButtonSet;
use hookmap_core::Button;
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct AloneModifierMap(HashMap<Button, bool>);

impl AloneModifierMap {
    /// Called when a button is pressed.
    pub(crate) fn press_event(&mut self, button: Button) {
        match self.0.get_mut(&button) {
            Some(is_alone) => *is_alone = true,
            None => self.0.values_mut().for_each(|is_alone| *is_alone = false),
        }
    }

    /// Called when a button is released.
    pub(crate) fn is_alone(&self, button: Button) -> bool {
        matches!(self.0.get(&button), Some(true))
    }
}

impl From<ModifierButtonSet> for AloneModifierMap {
    fn from(modifier_button_set: ModifierButtonSet) -> Self {
        let map = modifier_button_set
            .0
            .into_iter()
            .map(|button| (button, false))
            .collect();
        Self(map)
    }
}
