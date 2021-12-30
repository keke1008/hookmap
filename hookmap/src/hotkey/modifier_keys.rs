use crate::{button::ButtonSet, ButtonState};

#[derive(Clone, Debug, Default)]
pub struct ModifierKeys {
    pub pressed: Vec<ButtonSet>,
    pub released: Vec<ButtonSet>,
}

impl ModifierKeys {
    pub fn new(pressed: Vec<ButtonSet>, released: Vec<ButtonSet>) -> Self {
        Self { pressed, released }
    }

    pub(super) fn meets_conditions(&self) -> bool {
        self.pressed.iter().all(|button| button.is_pressed())
            && self.released.iter().all(|button| button.is_released())
    }
}
