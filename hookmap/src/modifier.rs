use hookmap_core::{Button, ButtonState};
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

#[derive(Debug, Default, Clone)]
pub struct ModifierButtonSet(pub(crate) HashSet<Button>);

impl ModifierButtonSet {
    pub fn add(&mut self, button: Button) {
        self.0.insert(button);
    }
}

#[derive(Debug, Default)]
pub(crate) struct ModifierChecker {
    cache: HashMap<Button, bool>,
}

impl ModifierChecker {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn check(&mut self, modifier: &ModifierButtonSet) -> bool {
        modifier.0.iter().all(|button| {
            *self
                .cache
                .entry(*button)
                .or_insert_with(|| button.is_pressed())
        })
    }
}
