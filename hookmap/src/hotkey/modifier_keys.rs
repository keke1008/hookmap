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

    pub(super) fn add_pressed(&self, button: ButtonSet) -> Self {
        let mut pressed = self.pressed.clone();
        pressed.push(button);
        Self {
            pressed,
            released: self.released.clone(),
        }
    }

    pub(super) fn add_released(&self, button: ButtonSet) -> Self {
        let mut released = self.released.clone();
        released.push(button);
        Self {
            pressed: self.pressed.clone(),
            released,
        }
    }

    pub(super) fn meets_conditions(&self) -> bool {
        self.pressed.iter().all(|button| button.is_pressed())
            && self.released.iter().all(|button| button.is_released())
    }
}
