//! Items used in macros.

use crate::button::Button;
use std::iter::{self, FromIterator};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ButtonArgElementTag {
    Direct,
    Inversion,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ButtonArgElement {
    pub tag: ButtonArgElementTag,
    pub button: Button,
}

impl ButtonArgElement {
    pub fn direct(button: Button) -> Self {
        ButtonArgElement {
            tag: ButtonArgElementTag::Direct,
            button,
        }
    }

    pub fn inversion(button: Button) -> Self {
        ButtonArgElement {
            tag: ButtonArgElementTag::Inversion,
            button,
        }
    }

    pub fn invert(&self) -> Self {
        match self.tag {
            ButtonArgElementTag::Direct => ButtonArgElement::inversion(self.button),
            ButtonArgElementTag::Inversion => ButtonArgElement::direct(self.button),
        }
    }
}

/// A struct used in macros to pass multiple buttons to a function.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ButtonArg(Vec<ButtonArgElement>);

impl ButtonArg {
    pub fn iter(&self) -> impl Iterator<Item = ButtonArgElement> + '_ {
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

/// Constructs [`ButtonArgs`].
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
                ($crate::macros::ExpandButtonArg::expand_inverse($crate::button_name!($button).clone()))
            ]
            $($rest)*
        )
    };

    (@inner [ $($parsed:tt)* ] $button:tt $($rest:tt)*) => {
        $crate::buttons!(
            @inner
            [
                $($parsed)*
                ($crate::macros::ExpandButtonArg::expand($crate::button_name!($button).clone()))
            ]
            $($rest)*
        )
    };

    (@inner [ $($parsed:tt)* ]) => {
        IntoIterator::into_iter(
            [ $($parsed),* ]
        )
        .collect::<$crate::macros::ButtonArg>()
    };

    ($($args:tt)*) => {
        $crate::buttons!(@inner [] $($args)*)
    };
}

/// Expands button names.
///
/// If the argument is enclosed in square brackets, it will be expanded without any action.
///
/// # Example
/// ```no_run
/// use hookmap::{button_name, devices::Button};
/// assert_eq!(Button::Key0, button_name!(0));
/// assert_eq!(Button::A, button_name!(A));
///
/// let button_a = Button::A;
/// assert_eq!(Button::A, button_name!([button_a]));
/// ```
///
// Using `#[rustfmt_skip]` instead, the following error is generated.
// error: macro-expanded `macro_export` macros from the current crate cannot be referred to by absolute paths
#[allow(clippy::deprecated_cfg_attr)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[macro_export]
macro_rules! button_name {
    ([$button:expr]) => ($button);
    (Shift)          => ($crate::devices::SHIFT);
    (Ctrl)           => ($crate::devices::Ctrl);
    (Alt)            => ($crate::devices::Alt);
    (Meta)           => ($crate::devices::Meta);
    ($button:ident)  => ($crate::devices::Button::$button);
    (0)              => ($crate::devices::Button::Key0);
    (1)              => ($crate::devices::Button::Key1);
    (2)              => ($crate::devices::Button::Key2);
    (3)              => ($crate::devices::Button::Key3);
    (4)              => ($crate::devices::Button::Key4);
    (5)              => ($crate::devices::Button::Key5);
    (6)              => ($crate::devices::Button::Key6);
    (7)              => ($crate::devices::Button::Key7);
    (8)              => ($crate::devices::Button::Key8);
    (9)              => ($crate::devices::Button::Key9);
}

/// Sends keyboard input.
/// Unlike send!, seq! does not ignore modifier keys.
///
/// # Examples
///
/// ```no_run
/// use hookmap::*;
/// seq!(A, B);
/// ```
///
/// Use `down` and `up` to press and release keys.
///
/// ```no_run
/// use hookmap::*;
/// seq!(LCtrl down, A, LCtrl up);
/// ```
///
/// Use `with(...)` to specify the keys to hold down when sending.
///
/// ```no_run
/// use hookmap::*;
/// seq!(with(LShift, LCtrl), Tab);
/// seq!(LShift down, LCtrl down, Tab, LShift up, LCtrl up); // equals to above
/// ```
///
#[macro_export]
macro_rules! seq {
    (@with $($modifier:tt),*) => {
        vec![ $($crate::button_name!($modifier)),* ]
    };

    (@single $button:tt $op:ident) => {
        $crate::devices::SequenceOperation::$op($crate::button_name!($button))
    };

    (@button $parsed:tt) => {
        vec!$parsed
    };

    (@button $parsed:tt , $($rest:tt)*) => {
        $crate::seq!(@button $parsed $($rest)*)
    };

    (@button [ $($parsed:tt),* ] $button:tt up $($rest:tt)*) => {
        $crate::seq!(
            @button
            [ $($parsed,)* ($crate::seq!(@single $button Release)) ]
            $($rest)*
        )
    };

    (@button [ $($parsed:tt),* ] $button:tt down $($rest:tt)*) => {
        $crate::seq!(
            @button
            [ $($parsed,)* ($crate::seq!(@single $button Press)) ]
            $($rest)*
        )
    };

    (@button [ $($parsed:tt),* ] $button:tt $($rest:tt)*) => {
        $crate::seq!(
            @button
            [ $($parsed,)* ($crate::seq!(@single $button Click)) ]
            $($rest)*
        )
    };

    (with( $($modifier:tt),* ), $($button:tt)*) => {
        $crate::devices::Sequence::new(
            seq!( @with $($modifier),* ),
            seq!( @button [] $($button)* ),
        )
    };

    ($($button:tt)*) => {
        $crate::devices::Sequence::new(
            vec![],
            $crate::seq!( @button [] $($button)* ),
        )
    };
}

pub const MODIFIER_LIST: [Button; 8] = [
    Button::LShift,
    Button::RShift,
    Button::LCtrl,
    Button::RCtrl,
    Button::LAlt,
    Button::RAlt,
    Button::LMeta,
    Button::RMeta,
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::button::{Button, Sequence, SequenceOperation};

    #[test]
    fn button_args() {
        use Button::*;
        assert_eq!(buttons!(A), ButtonArg(vec![ButtonArgElement::direct(A)]));
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

    #[test]
    fn button_name_macro() {
        assert_eq!(button_name!(A), Button::A);
        assert_eq!(button_name!([Button::LShift]), Button::LShift);
    }

    #[test]
    fn seq_macro() {
        use SequenceOperation::{Click, Press, Release};
        assert_eq!(seq!(), Sequence::new(vec![], vec![]));
        assert_eq!(seq!(A), Sequence::new(vec![], vec![Click(Button::A)]));
        assert_eq!(
            seq!(A, B),
            Sequence::new(vec![], vec![Click(Button::A), Click(Button::B)])
        );
        assert_eq!(
            seq!([Button::A], [Button::B]),
            Sequence::new(vec![], vec![Click(Button::A), Click(Button::B)])
        );
        assert_eq!(seq!(A up), Sequence::new(vec![], vec![Release(Button::A)]));
        assert_eq!(
            seq!(A down, B, A up),
            Sequence::new(
                vec![],
                vec![Press(Button::A), Click(Button::B), Release(Button::A)]
            )
        );
        assert_eq!(
            seq!(with(A), C),
            Sequence::new(vec![Button::A], vec![Click(Button::C)])
        );
        assert_eq!(
            seq!(with(A, B), C),
            Sequence::new(vec![Button::A, Button::B], vec![Click(Button::C)])
        );
        assert_eq!(
            seq!(with([Button::A], B), C up),
            Sequence::new(vec![Button::A, Button::B], vec![Release(Button::C)])
        );
    }
}
