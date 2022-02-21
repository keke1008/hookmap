use crate::button::Button;
use std::iter::{self, FromIterator};

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

/// A struct used in macros to pass multiple buttons to a function.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct ButtonArg(Vec<ButtonArgElement<Button>>);

impl ButtonArg {
    pub fn iter(&self) -> impl Iterator<Item = ButtonArgElement<Button>> + '_ {
        self.0.iter().copied()
    }
}

impl From<Button> for ButtonArg {
    fn from(button: Button) -> Self {
        ButtonArg(vec![ButtonArgElement::direct(button)])
    }
}

impl FromIterator<Box<dyn Iterator<Item = ButtonArgElement>>> for ButtonArg {
    fn from_iter<T: IntoIterator<Item = Box<dyn Iterator<Item = ButtonArgElement>>>>(
        iter: T,
    ) -> Self {
        ButtonArg(Vec::from_iter(iter.into_iter().flatten()))
    }
}

impl Into<ButtonArg> for &ButtonArg {
    fn into(self) -> ButtonArg {
        self.clone()
    }
}

pub trait ExpandButtonArg: Sized {
    fn expand(self) -> Box<dyn Iterator<Item = ButtonArgElement>>;
    fn expand_inverse(self) -> Box<dyn Iterator<Item = ButtonArgElement>> {
        Box::new(self.expand().map(|e| e.invert()))
    }
}

impl ExpandButtonArg for ButtonArg {
    fn expand(self) -> Box<dyn Iterator<Item = ButtonArgElement>> {
        Box::new(self.0.into_iter())
    }
}

impl ExpandButtonArg for Button {
    fn expand(self) -> Box<dyn Iterator<Item = ButtonArgElement>> {
        Box::new(iter::once(ButtonArgElement::direct(self)))
    }
}

/// Constructs [`ButtonArg`].
#[macro_export]
macro_rules! buttons {
    (@inner $parsed:tt , $($rest:tt)*) => {
        $crate::buttons!(@inner $parsed $($rest)*)
    };

    (@inner [ $($parsed:tt)* ] !$button:tt $($rest:tt)*) => {
        $crate::buttons!(
            @inner
            [
                $($parsed)*
                ($crate::hotkey::button_arg::ExpandButtonArg::expand_inverse($crate::button_name!($button).clone()))
            ]
            $($rest)*
        )
    };

    (@inner [ $($parsed:tt)* ] $button:tt $($rest:tt)*) => {
        $crate::buttons!(
            @inner
            [
                $($parsed)*
                ($crate::hotkey::button_arg::ExpandButtonArg::expand($crate::button_name!($button).clone()))
            ]
            $($rest)*
        )
    };

    (@inner [ $($parsed:tt)* ]) => {
        IntoIterator::into_iter(
            [ $($parsed),* ]
        )
        .collect::<$crate::hotkey::button_arg::ButtonArg>()
    };

    ($($args:tt)*) => {
        $crate::buttons!(@inner [] $($args)*)
    };
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
        assert_eq!(buttons!([button_args]), button_args);
        assert_eq!(
            buttons!([button_args], C, !D),
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
