/// Expands button names.
///
/// If the argument is enclosed in square brackets, it will be expanded without any action.
///
/// # Example
/// ```
/// use hookmap::*;
/// assert_eq!(Button::Key0, button_name!(0));
/// assert_eq!(Button::A, button_name!(A));
///
/// let button_a = Button::A;
/// assert_eq!(Button::A, button_name!([button_a]));
/// ```
///
#[rustfmt::skip]
#[macro_export]
macro_rules! button_name {
    ([$button:expr]) => ($button);
    ($button:ident)  => ($crate::Button::$button);
    (0)              => ($crate::Button::Key0);
    (1)              => ($crate::Button::Key1);
    (2)              => ($crate::Button::Key2);
    (3)              => ($crate::Button::Key3);
    (4)              => ($crate::Button::Key4);
    (5)              => ($crate::Button::Key5);
    (6)              => ($crate::Button::Key6);
    (7)              => ($crate::Button::Key7);
    (8)              => ($crate::Button::Key8);
    (9)              => ($crate::Button::Key9);
    (;)              => ($crate::Button::SemiColon);
    (-)              => ($crate::Button::Minus);
    (/)              => ($crate::Button::Slash);

}

/// Presses and releases the keys in sequence.
///
///
/// ```no_run
/// use hookmap::*;
/// seq!(0, 1, 2, 3, A, B, C);
/// ```
///
#[macro_export]
macro_rules! seq {
    ($($button:tt,)+) => {
        seq!($($button),+)
    };

    ($($button:tt),*) => {
        seq!(@full_name $(button_name!($button)),*)
    };

    (@full_name $($button:expr),*) => {{
        $($button.click();)*
    }};
}

/// While holding down the keys after "with", clicks the keys before "with".
///
/// ```
/// use hookmap::*;
/// press!([Delete] with [LCtrl, RAlt]);
/// ```
///
#[macro_export]
macro_rules! press {
    (@full_name [$($button:expr),*] with [$($modifier:expr),*]) => {{
        $($modifier.press();)*
        seq!(@full_name $($button),*);
        $($modifier.release();)*
    }};

    ([$($button:tt),*] with [$($modifier:tt),*]) => {
        press!(@full_name [$(button_name!($button)),*] with [$(button_name!($modifier)),*])
    };

}

#[macro_export]
macro_rules! press_v{
    ([$($button:expr),*] with [$($modifier:expr),*]) => {
        press!(@full_name [$($button),*] with [$($modifier),*])
    };
}
