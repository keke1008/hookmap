use crate::button::ButtonState;
use hookmap_core::Button;

#[derive(Clone, Debug, Default)]
pub struct ModifierKeys {
    pub pressed: Vec<Button>,
    pub released: Vec<Button>,
}

impl ModifierKeys {
    pub fn new(pressed: &[Button], released: &[Button]) -> Self {
        Self {
            pressed: pressed.to_owned(),
            released: released.to_owned(),
        }
    }

    pub fn merge(&self, other: &Self) -> Self {
        ModifierKeys {
            pressed: self
                .pressed
                .iter()
                .chain(other.pressed.iter())
                .cloned()
                .collect(),
            released: self
                .released
                .iter()
                .chain(other.released.iter())
                .cloned()
                .collect(),
        }
    }

    pub(super) fn meets_conditions(&self) -> bool {
        self.pressed.iter().all(|button| button.is_pressed())
            && self.released.iter().all(|button| button.is_released())
    }
}
