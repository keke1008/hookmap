use hookmap_core::Button;
use std::{collections::HashSet, fmt::Debug};

#[derive(Debug, Default, Clone)]
pub struct ModifierButtonSet(pub(crate) HashSet<Button>);

impl ModifierButtonSet {
    pub fn add(&mut self, button: Button) {
        self.0.insert(button);
    }
}
