//! Items used in macros.

use crate::button::Button;

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
#[doc(hidden)]
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

#[doc(hidden)]
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
    use crate::button::{Button, Sequence, SequenceOperation};

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
