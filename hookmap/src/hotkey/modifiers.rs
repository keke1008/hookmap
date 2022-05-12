use super::button_arg::{ButtonArg, ButtonArgElementTag};
use hookmap_core::button::Button;

#[derive(Clone, Debug, Default)]
pub(crate) struct Modifiers {
    pub pressed: Vec<Button>,
    pub released: Vec<Button>,
}

impl Modifiers {
    pub fn merge(&self, other: Self) -> Self {
        Modifiers {
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

    pub(super) fn iter_pressed(&self) -> std::slice::Iter<Button> {
        self.pressed.iter()
    }

    pub(super) fn iter_released(&self) -> std::slice::Iter<Button> {
        self.released.iter()
    }
}

impl From<ButtonArg> for Modifiers {
    fn from(args: ButtonArg) -> Self {
        let mut pressed = vec![];
        let mut released = vec![];
        for arg in args.iter() {
            match arg.tag {
                ButtonArgElementTag::Direct => pressed.push(arg.value),
                ButtonArgElementTag::Inversion => released.push(arg.value),
            }
        }
        Self { pressed, released }
    }
}
