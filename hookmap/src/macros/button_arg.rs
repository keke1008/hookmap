use hookmap_core::button::Button;
use std::borrow::Borrow;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonArgUnit<T> {
    Plain(T),
    Not(T),
}

impl<T: Clone> ButtonArgUnit<T> {
    pub fn invert(&self) -> Self {
        match self {
            Self::Plain(v) => Self::Not(v.clone()),
            Self::Not(v) => Self::Plain(v.clone()),
        }
    }
}

/// A struct to pass multiple buttons to a function.
/// This struct constructs by [`buttons!`].
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct ButtonArg(Vec<ButtonArgUnit<Button>>);

impl ButtonArg {
    pub(crate) fn invert(&self) -> ButtonArg {
        let inner = self.0.iter().map(|unit| unit.invert()).collect();
        ButtonArg(inner)
    }

    pub(crate) fn is_all_plain(&self) -> bool {
        self.0
            .iter()
            .all(|unit| matches!(unit, ButtonArgUnit::Plain(_)))
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = ButtonArgUnit<Button>> + '_ {
        self.0.iter().copied()
    }

    pub(crate) fn iter_plain(&self) -> impl Iterator<Item = Button> + '_ {
        self.0.iter().filter_map(|unit| match unit {
            ButtonArgUnit::Plain(button) => Some(*button),
            ButtonArgUnit::Not(_) => None,
        })
    }

    pub(crate) fn iter_not(&self) -> impl Iterator<Item = Button> + '_ {
        self.0.iter().filter_map(|unit| match unit {
            ButtonArgUnit::Not(button) => Some(*button),
            ButtonArgUnit::Plain(_) => None,
        })
    }
}

impl From<Button> for ButtonArg {
    fn from(button: Button) -> Self {
        ButtonArg(vec![ButtonArgUnit::Plain(button)])
    }
}

impl<T> From<&T> for ButtonArg
where
    T: Clone + Into<ButtonArg>,
{
    fn from(arg: &T) -> Self {
        (*arg).clone().into()
    }
}

#[doc(hidden)]
pub trait ButtonArgChain<T> {
    fn chain(self, next: ButtonArgUnit<T>) -> ButtonArg;
}

impl ButtonArgChain<Button> for ButtonArg {
    fn chain(mut self, next: ButtonArgUnit<Button>) -> ButtonArg {
        self.0.push(next);
        self
    }
}

impl<T> ButtonArgChain<T> for ButtonArg
where
    T: Borrow<ButtonArg>,
{
    fn chain(mut self, next: ButtonArgUnit<T>) -> ButtonArg {
        let mut next = match next {
            ButtonArgUnit::Plain(v) => v.borrow().clone(),
            ButtonArgUnit::Not(v) => v.borrow().invert(),
        };
        self.0.append(&mut next.0);
        self
    }
}

/// Passing multiple buttons as an argument for functions like [`RegisterHotkey::remap`].
///
/// This macro produces a value of type [`ButtonArg`].
///
/// # Syntax
///
/// This macro receives variant names of [`Button`] separated by commas,
///
/// ```
/// # use hookmap::prelude::*;
/// buttons!(A, B, Esc, F12);
/// ```
///
/// Or number letarals corresponding to numeric keys.
///
/// ```
/// # use hookmap::prelude::*;
/// buttons!(0, 1, 2);
/// ```
///
/// Variables can be used by enclosing them in square brackets.
///
/// ```
/// # use hookmap::prelude::*;
/// let a = Button::A;
/// buttons!([a], B, C);
/// ```
///
/// The prefix `!` means that this key is released.
///
/// ```
/// # use hookmap::prelude::*;
/// let b = Button::B;
/// buttons!(!A, ![b], !0);
/// ```
///
/// Nested `buttons!(..)` will be flattened.
///
/// ```
/// # use hookmap::prelude::*;
/// let arg = buttons!(A, B, C);
/// assert_eq!(
///     buttons!([arg], D, E),
///     buttons!(A, B, C, D, E),
/// );
/// ```
///
/// The prefix `!` of nested `buttons!(..)` will affect each button.
///
/// ```
/// # use hookmap::prelude::*;
/// let arg = buttons!(A, B, C);
/// assert_eq!(
///     buttons!(![arg], D, E),
///     buttons!(!A, !B, !C, D, E),
/// );
/// ```
///
/// [`RegisterHotkey::remap`]: super::RegisterHotkey::remap
///
#[macro_export]
macro_rules! buttons {
    (@inner $chain:tt , $($tail:tt)*) => {
        $crate::buttons!(@inner $chain $($tail)*)
    };

    (@inner $chain:tt !$button:tt $($tail:tt)*) => {{
        let element = ButtonArgUnit::Not($crate::button_name!($button));
        $crate::buttons!(@inner ($chain.chain(element)) $($tail)*)
    }};

    (@inner $chain:tt $button:tt $($tail:tt)*) => {{
        let element = ButtonArgUnit::Plain($crate::button_name!($button));
        $crate::buttons!(@inner ($chain.chain(element)) $($tail)*)
    }};

    (@inner $chain:tt) => {
        $chain
    };

    ($($args:tt)*) => {{
        #[allow(unused_imports)]
        use $crate::macros::button_arg::{ButtonArgChain, ButtonArg, ButtonArgUnit};
        $crate::buttons!(@inner (ButtonArg::default()) $($args)*)
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn button_args() {
        use Button::*;
        assert_eq!(
            buttons!(A),
            ButtonArg(vec![ButtonArgUnit::Plain(Button::A)])
        );
        assert_eq!(buttons!(!A), ButtonArg(vec![ButtonArgUnit::Not(A)]));
        assert_eq!(
            buttons!(A, !B),
            ButtonArg(vec![ButtonArgUnit::Plain(A), ButtonArgUnit::Not(B)])
        );
        assert_eq!(
            buttons!(A, !B),
            ButtonArg(vec![ButtonArgUnit::Plain(A), ButtonArgUnit::Not(B)])
        );
        let button_args = ButtonArg(vec![ButtonArgUnit::Plain(A), ButtonArgUnit::Not(B)]);
        assert_eq!(buttons!([&button_args]), button_args);
        assert_eq!(
            buttons!([&button_args], C, !D),
            ButtonArg(vec![
                ButtonArgUnit::Plain(A),
                ButtonArgUnit::Not(B),
                ButtonArgUnit::Plain(C),
                ButtonArgUnit::Not(D)
            ])
        );
        assert_eq!(
            buttons!(C, !D, [button_args]),
            ButtonArg(vec![
                ButtonArgUnit::Plain(C),
                ButtonArgUnit::Not(D),
                ButtonArgUnit::Plain(A),
                ButtonArgUnit::Not(B)
            ]),
        );
    }
}
