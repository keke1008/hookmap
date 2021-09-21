use super::ButtonState;
use crate::any;
use hookmap_core::Button;
use once_cell::sync::Lazy;
use std::borrow::Borrow;
use std::{collections::HashSet, fmt::Debug};

macro_rules! impl_any_or_all {
    ($name:ident) => {
        impl $name {
            pub fn new<T: Borrow<[Button]>>(buttons: T) -> Self {
                let inner = buttons.borrow().iter().copied().collect();
                Self(inner)
            }

            pub fn append<T: Borrow<Button>>(&self, button: T) -> Self {
                let mut inner = self.0.clone();
                inner.insert(*button.borrow());
                Self(inner)
            }

            pub fn remove<T: Borrow<Button>>(&self, button: T) -> Self {
                let mut inner = self.0.clone();
                inner.remove(&button.borrow());
                Self(inner)
            }
        }
    };
}

/// A struct foe operating any buttons.
///
/// # Example
///
/// ```
/// use hookmap::*;
/// let hook = Hook::new();
/// let any = any!(A, B);
/// hook.bind(any)
///     .on_press(|e| {
///         assert!(e.target == Button::A || e.target == Button::B);
///     });
/// ```
///
#[derive(Debug, Clone)]
pub struct Any(pub(super) HashSet<Button>);
impl_any_or_all!(Any);

/// A struct for operating all buttons.
///
/// # Example
///
/// ```
/// use hookmap::*;
/// let hook = Hook::new();
/// let all = all!(A, B);
/// hook.bind(all)
///     .on_press(|e| {
///         assert!(e.target == Button::A || e.target == Button::B);
///         assert!(Button::A.is_pressed() && Button::B.is_pressed());
///     });
/// ```
///
#[derive(Debug, Clone)]
pub struct All(pub(super) HashSet<Button>);
impl_any_or_all!(All);

pub enum ButtonSet {
    Single(Button),
    Any(Any),
    All(All),
}

impl ButtonSet {
    pub(crate) fn iter_buttons(&self) -> impl Iterator<Item = &Button> + '_ {
        match self {
            ButtonSet::Single(button) => Iter::Once(Some(button)),
            ButtonSet::Any(any) => Iter::Set(any.0.iter()),
            ButtonSet::All(all) => Iter::Set(all.0.iter()),
        }
    }
}

impl Debug for ButtonSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ButtonSet::Single(button) => f.write_fmt(format_args!("{:?}", button)),
            ButtonSet::Any(any) => f.write_fmt(format_args!("{:?}", any)),
            ButtonSet::All(all) => f.write_fmt(format_args!("{:?}", all)),
        }
    }
}

pub(crate) enum Iter<'a> {
    Once(Option<&'a Button>),
    Set(std::collections::hash_set::Iter<'a, Button>),
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Button;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Iter::Once(button) => button.take(),
            Iter::Set(iter) => iter.next(),
        }
    }
}

impl ButtonState for ButtonSet {
    fn is_pressed(&self) -> bool {
        match self {
            ButtonSet::Single(button) => button.is_pressed(),
            ButtonSet::Any(any) => any.is_pressed(),
            ButtonSet::All(all) => all.is_pressed(),
        }
    }

    fn is_released(&self) -> bool {
        match self {
            ButtonSet::Single(button) => button.is_released(),
            ButtonSet::Any(any) => any.is_released(),
            ButtonSet::All(all) => all.is_released(),
        }
    }
}

pub static SHIFT: Lazy<Any> = Lazy::new(|| any!(LShift, RShift));
pub static CTRL: Lazy<Any> = Lazy::new(|| any!(LCtrl, RCtrl));
pub static ALT: Lazy<Any> = Lazy::new(|| any!(LAlt, RAlt));
pub static META: Lazy<Any> = Lazy::new(|| any!(LMeta, RMeta));

#[cfg(test)]
mod tests {
    use super::super::traits::*;
    use super::*;

    #[test]
    fn any_to_iterator() {
        let any = any!(A, B).to_button_set();
        let mut iter = any.iter_buttons();
        assert!(matches!(iter.next(), Some(&Button::A | &Button::B)));
        assert!(matches!(iter.next(), Some(&Button::A | &Button::B)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn single_to_iterator() {
        let mut iter = ButtonSet::Single(Button::A).iter_buttons();
        assert_eq!(iter.next(), Some(&Button::A));
        assert_eq!(iter.next(), None);
    }
}
