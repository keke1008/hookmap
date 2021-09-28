use hookmap_core::Button;
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum ButtonSet {
    Single(Button),
    Any(Vec<Button>),
    All(Vec<Button>),
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

pub static SHIFT: Lazy<ButtonSet> = Lazy::new(|| crate::any!(LShift, RShift));
pub static CTRL: Lazy<ButtonSet> = Lazy::new(|| crate::any!(LCtrl, RCtrl));
pub static ALT: Lazy<ButtonSet> = Lazy::new(|| crate::any!(LAlt, RAlt));
pub static META: Lazy<ButtonSet> = Lazy::new(|| crate::any!(LMeta, RMeta));
