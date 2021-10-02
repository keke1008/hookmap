use super::{ButtonInput, ButtonState};
use hookmap_core::Button;
use once_cell::sync::Lazy;
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

impl From<&Lazy<ButtonSet>> for ButtonSet {
    fn from(buttons: &Lazy<ButtonSet>) -> Self {
        (*buttons).clone()
    }
}

pub static SHIFT: Lazy<ButtonSet> = Lazy::new(|| crate::any!(LShift, RShift));
pub static CTRL: Lazy<ButtonSet> = Lazy::new(|| crate::any!(LCtrl, RCtrl));
pub static ALT: Lazy<ButtonSet> = Lazy::new(|| crate::any!(LAlt, RAlt));
pub static META: Lazy<ButtonSet> = Lazy::new(|| crate::any!(LMeta, RMeta));

impl ButtonInput for Lazy<ButtonSet> {
    fn press(&self) {
        (**self).press();
    }

    fn release(&self) {
        (**self).release();
    }

    fn press_recursive(&self) {
        (**self).press_recursive();
    }

    fn release_recursive(&self) {
        (**self).release_recursive();
    }
}

impl<T: ButtonInput> ButtonInput for &T {
    fn press(&self) {
        (**self).press();
    }

    fn release(&self) {
        (**self).release();
    }

    fn press_recursive(&self) {
        (**self).press_recursive();
    }

    fn release_recursive(&self) {
        (**self).release_recursive();
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
