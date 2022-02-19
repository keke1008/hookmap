use super::button_arg::{ButtonArg, ButtonArgElementTag};
use crate::button::ButtonState;
use hookmap_core::Button;

#[derive(Clone, Debug, Default)]
pub(crate) struct ModifierKeys {
    pub pressed: Vec<Button>,
    pub released: Vec<Button>,
}

impl ModifierKeys {
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

impl From<ButtonArg> for ModifierKeys {
    fn from(args: ButtonArg) -> Self {
        let mut pressed = vec![];
        let mut released = vec![];
        for arg in args.iter() {
            match arg.tag {
                ButtonArgElementTag::Direct => pressed.push(arg.button),
                ButtonArgElementTag::Inversion => released.push(arg.button),
            }
        }
        Self { pressed, released }
    }
}
