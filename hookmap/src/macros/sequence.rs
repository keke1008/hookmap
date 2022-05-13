pub use hookmap_core::button::{Button, ButtonAction};
pub use hookmap_core::event::ButtonEvent;

/// Emulates button input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SequenceOperation {
    Click(Button),
    Press(Button),
    Release(Button),
}

impl SequenceOperation {
    fn operate(&self) {
        match self {
            SequenceOperation::Click(button) => button.click(),
            SequenceOperation::Press(button) => button.press(),
            SequenceOperation::Release(button) => button.release(),
        }
    }

    fn operate_recursive(&self) {
        match self {
            SequenceOperation::Click(button) => button.click_recursive(),
            SequenceOperation::Press(button) => button.press_recursive(),
            SequenceOperation::Release(button) => button.release_recursive(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sequence {
    with: Vec<Button>,
    seq: Vec<SequenceOperation>,
}

impl Sequence {
    const MODIFIER_LIST: [Button; 8] = [
        Button::LShift,
        Button::RShift,
        Button::LCtrl,
        Button::RCtrl,
        Button::LAlt,
        Button::RAlt,
        Button::LSuper,
        Button::RSuper,
    ];

    pub fn new(with: Vec<Button>, seq: Vec<SequenceOperation>) -> Self {
        Self { with, seq }
    }

    fn operate_with_keys(&self, operation: fn(Button)) {
        self.with.iter().copied().for_each(operation);
    }

    fn send_inner(
        &self,
        press: fn(Button),
        release: fn(Button),
        operation: fn(&SequenceOperation),
    ) {
        self.operate_with_keys(press);
        self.seq.iter().for_each(operation);
        self.operate_with_keys(release);
    }

    pub fn send(&self) {
        self.send_inner(Button::press, Button::release, SequenceOperation::operate);
    }

    pub fn send_recursive(&self) {
        self.send_inner(
            Button::press_recursive,
            Button::release_recursive,
            SequenceOperation::operate_recursive,
        );
    }

    fn send_ignore_modifiers_inner(
        &self,
        press: fn(Button),
        release: fn(Button),
        operation: fn(&SequenceOperation),
    ) {
        let pressed_modifiers: Vec<_> = Self::MODIFIER_LIST
            .iter()
            .copied()
            .filter(|button| button.is_pressed())
            .collect();

        pressed_modifiers.iter().copied().for_each(release);
        self.send_inner(press, release, operation);
        pressed_modifiers.iter().copied().for_each(press);
    }

    pub fn send_ignore_modifiers(&self) {
        self.send_ignore_modifiers_inner(
            Button::press,
            Button::release,
            SequenceOperation::operate,
        );
    }

    pub fn send_ignore_modifiers_recursive(&self) {
        self.send_ignore_modifiers_inner(
            Button::press_recursive,
            Button::release_recursive,
            SequenceOperation::operate_recursive,
        );
    }
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
        $crate::macros::sequence::SequenceOperation::$op($crate::button_name!($button))
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
        $crate::macros::sequence::Sequence::new(
            seq!( @with $($modifier),* ),
            seq!( @button [] $($button)* ),
        )
    };

    ($($button:tt)*) => {
        $crate::macros::sequence::Sequence::new(
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
    Button::LSuper,
    Button::RSuper,
];

#[cfg(test)]
mod tests {
    use super::{Sequence, SequenceOperation};
    use crate::button_name;
    use crate::device::Button;

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
