use crate::{button::ButtonSet, ButtonState};

#[derive(Clone, Debug)]
pub(super) struct ModifierKeys {
    pressed: Vec<ButtonSet>,
    released: Vec<ButtonSet>,
}

impl ModifierKeys {
    pub(super) fn new(pressed: Vec<ButtonSet>, released: Vec<ButtonSet>) -> Self {
        Self { pressed, released }
    }

    pub(super) fn push_pressed(&mut self, button: ButtonSet) {
        self.pressed.push(button);
    }

    pub(super) fn push_released(&mut self, button: ButtonSet) {
        self.released.push(button);
    }

    pub(super) fn meets_conditions(&self) -> bool {
        self.pressed.iter().all(|button| button.is_pressed())
            && self.released.iter().all(|button| button.is_released())
    }
}
