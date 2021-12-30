use crate::{button::ButtonSet, ButtonState};

#[derive(Clone, Debug, Default)]
pub(super) struct ModifierKeys {
    pub(super) pressed: Vec<ButtonSet>,
    pub(super) released: Vec<ButtonSet>,
}

impl ModifierKeys {
    pub(super) fn new(pressed: Vec<ButtonSet>, released: Vec<ButtonSet>) -> Self {
        Self { pressed, released }
    }

    pub(super) fn meets_conditions(&self) -> bool {
        self.pressed.iter().all(|button| button.is_pressed())
            && self.released.iter().all(|button| button.is_released())
    }
}
