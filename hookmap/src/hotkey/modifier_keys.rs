use crate::{
    button::ButtonState,
    macros::{ButtonArg, ButtonArgs},
};
use hookmap_core::Button;

#[derive(Clone, Debug, Default)]
pub struct ModifierKeys {
    pub pressed: Vec<Button>,
    pub released: Vec<Button>,
}

impl ModifierKeys {
    pub fn new(pressed: Vec<Button>, released: Vec<Button>) -> Self {
        Self { pressed, released }
    }

    pub fn merge(&self, other: Self) -> Self {
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

impl From<ButtonArgs> for ModifierKeys {
    fn from(args: ButtonArgs) -> Self {
        let mut pressed = vec![];
        let mut released = vec![];
        for arg in args.iter() {
            match arg {
                ButtonArg::Direct(button) => pressed.push(button),
                ButtonArg::Inversion(button) => released.push(button),
            }
        }
        Self { pressed, released }
    }
}
