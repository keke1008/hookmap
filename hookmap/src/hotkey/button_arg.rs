use crate::button::Button;
use std::borrow::Borrow;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ButtonArgElementTag {
    Direct,
    Inversion,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ButtonArgElement<T> {
    pub tag: ButtonArgElementTag,
    pub value: T,
}

impl<T> ButtonArgElement<T> {
    pub fn direct(value: T) -> Self {
        ButtonArgElement {
            tag: ButtonArgElementTag::Direct,
            value,
        }
    }

    pub fn inversion(value: T) -> Self {
        ButtonArgElement {
            tag: ButtonArgElementTag::Inversion,
            value,
        }
    }
}

impl<T: Clone> ButtonArgElement<T> {
    pub fn invert(&self) -> Self {
        match self.tag {
            ButtonArgElementTag::Direct => ButtonArgElement::inversion(self.value.clone()),
            ButtonArgElementTag::Inversion => ButtonArgElement::direct(self.value.clone()),
        }
    }
}

/// A struct to pass multiple buttons to a function.
/// This struct constructs by [`buttons!`].
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct ButtonArg(Vec<ButtonArgElement<Button>>);

impl ButtonArg {
    pub(super) fn iter(&self) -> impl Iterator<Item = ButtonArgElement<Button>> + '_ {
        self.0.iter().copied()
    }
}

impl From<Button> for ButtonArg {
    fn from(button: Button) -> Self {
        ButtonArg(vec![ButtonArgElement::direct(button)])
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
    fn chain(self, next: ButtonArgElement<T>) -> ButtonArg;
}

impl ButtonArgChain<Button> for ButtonArg {
    fn chain(mut self, next: ButtonArgElement<Button>) -> ButtonArg {
        self.0.push(next);
        self
    }
}

impl<T> ButtonArgChain<T> for ButtonArg
where
    T: Borrow<ButtonArg>,
{
    fn chain(mut self, next: ButtonArgElement<T>) -> ButtonArg {
        let element = next.value.borrow();
        match next.tag {
            ButtonArgElementTag::Direct => {
                self.0.extend(element.iter());
            }
            ButtonArgElementTag::Inversion => {
                self.0.extend(element.iter().map(|x| x.invert()));
            }
        }
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
        let element = ButtonArgElement::inversion($crate::button_name!($button));
        $crate::buttons!(@inner ($chain.chain(element)) $($tail)*)
    }};

    (@inner $chain:tt $button:tt $($tail:tt)*) => {{
        let element = ButtonArgElement::direct($crate::button_name!($button));
        $crate::buttons!(@inner ($chain.chain(element)) $($tail)*)
    }};

    (@inner $chain:tt) => {
        $chain
    };

    ($($args:tt)*) => {{
        #[allow(unused_imports)]
        use $crate::hotkey::button_arg::{ButtonArgElement, ButtonArgChain, ButtonArg};
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
            ButtonArg(vec![ButtonArgElement::direct(Button::A)])
        );
        assert_eq!(
            buttons!(!A),
            ButtonArg(vec![ButtonArgElement::inversion(A)])
        );
        assert_eq!(
            buttons!(A, !B),
            ButtonArg(vec![
                ButtonArgElement::direct(A),
                ButtonArgElement::inversion(B)
            ])
        );
        assert_eq!(
            buttons!(A, !B),
            ButtonArg(vec![
                ButtonArgElement::direct(A),
                ButtonArgElement::inversion(B)
            ])
        );
        let button_args = ButtonArg(vec![
            ButtonArgElement::direct(A),
            ButtonArgElement::inversion(B),
        ]);
        assert_eq!(buttons!([&button_args]), button_args);
        assert_eq!(
            buttons!([&button_args], C, !D),
            ButtonArg(vec![
                ButtonArgElement::direct(A),
                ButtonArgElement::inversion(B),
                ButtonArgElement::direct(C),
                ButtonArgElement::inversion(D)
            ])
        );
        assert_eq!(
            buttons!(C, !D, [button_args]),
            ButtonArg(vec![
                ButtonArgElement::direct(C),
                ButtonArgElement::inversion(D),
                ButtonArgElement::direct(A),
                ButtonArgElement::inversion(B)
            ]),
        );
    }
}
