use hookmap_core::{Button, ButtonInput, ButtonState};
use std::{collections::HashSet, sync::Arc};

#[derive(Debug, Default, Clone)]
pub struct ButtonSet(Arc<HashSet<Button>>);

impl ButtonSet {
    pub fn new(buttons: &[Button]) -> Self {
        let set = buttons.iter().copied().collect();
        Self(Arc::new(set))
    }

    pub fn insert(&self, button: Button) -> Self {
        let mut set = (*self.0).clone();
        set.insert(button);
        Self(Arc::new(set))
    }

    pub fn remove(&self, button: &Button) -> Self {
        let mut set = (*self.0).clone();
        set.remove(button);
        Self(Arc::new(set))
    }

    pub fn any(&self) -> Any {
        Any(Arc::clone(&self.0))
    }

    pub fn all(&self) -> All {
        All(Arc::clone(&self.0))
    }
}

pub struct Any(Arc<HashSet<Button>>);

impl ButtonState for Any {
    fn is_pressed(&self) -> bool {
        self.0.iter().any(Button::is_pressed)
    }
}

pub struct All(Arc<HashSet<Button>>);

impl ButtonState for All {
    fn is_pressed(&self) -> bool {
        self.0.iter().all(Button::is_pressed)
    }
}

impl ButtonInput for All {
    fn press(&self) {
        self.0.iter().for_each(Button::press);
    }

    fn release(&self) {
        self.0.iter().for_each(Button::release);
    }
}
