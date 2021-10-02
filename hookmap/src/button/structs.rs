use super::{ButtonInput, ButtonState};
use hookmap_core::Button;
use std::iter;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum ButtonSet {
    Single(Button),
    Any(Vec<Button>),
    All(Vec<Button>),
}

impl ButtonSet {
    pub(crate) fn iter(&self) -> Box<dyn Iterator<Item = &Button> + '_> {
        match self {
            ButtonSet::Single(ref button) => Box::new(iter::once(button)),
            ButtonSet::Any(ref buttons) => Box::new(buttons.iter()),
            ButtonSet::All(ref buttons) => Box::new(buttons.iter()),
        }
    }
}

impl From<Button> for ButtonSet {
    fn from(button: Button) -> Self {
        Self::Single(button)
    }
}

impl<T: Into<ButtonSet> + Clone> From<&T> for ButtonSet {
    fn from(button: &T) -> Self {
        <T as Into<ButtonSet>>::into(button.clone())
    }
}

#[derive(Debug, Clone)]
pub struct ConstantAny(&'static [Button]);

impl Into<ButtonSet> for ConstantAny {
    fn into(self) -> ButtonSet {
        ButtonSet::Any(self.0.into())
    }
}

impl ButtonInput for ConstantAny {
    fn press(&self) {
        self.0[0].press();
    }

    fn release(&self) {
        self.0[0].release();
    }

    fn press_recursive(&self) {
        self.0[0].press_recursive();
    }

    fn release_recursive(&self) {
        self.0[0].release_recursive();
    }
}

impl ButtonState for ConstantAny {
    fn is_pressed(&self) -> bool {
        self.0.iter().any(Button::is_pressed)
    }

    fn is_released(&self) -> bool {
        self.0.iter().any(Button::is_released)
    }
}

pub static SHIFT: ConstantAny = ConstantAny(&[Button::LShift, Button::RShift]);
pub static CTRL: ConstantAny = ConstantAny(&[Button::LCtrl, Button::RCtrl]);
pub static ALT: ConstantAny = ConstantAny(&[Button::LAlt, Button::RAlt]);
pub static META: ConstantAny = ConstantAny(&[Button::LMeta, Button::RMeta]);

impl<T: ButtonInput> ButtonInput for &T {
    fn press(&self) {
        (**self).press()
    }

    fn release(&self) {
        (**self).release()
    }

    fn press_recursive(&self) {
        (**self).press_recursive()
    }

    fn release_recursive(&self) {
        (**self).release_recursive()
    }
}

impl<T: ButtonState> ButtonState for &T {
    fn is_pressed(&self) -> bool {
        (**self).is_pressed()
    }

    fn is_released(&self) -> bool {
        (**self).is_released()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SelectHandleTarget;

    #[test]
    fn use_modifier_static_variable_in_condition() {
        crate::hotkey!(crate::Hotkey::new() => {
            modifier ([&SHIFT], [&CTRL], [&ALT], [&META]) {}
            modifier (![&SHIFT], ![&CTRL], ![&ALT], ![&META]) {}
        });
    }

    #[test]
    fn use_modifier_static_variable_in_bind() {
        crate::hotkey!(crate::Hotkey::new() => {
            bind [&SHIFT] => [&CTRL];
        });
    }
}
