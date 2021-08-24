/// Expands button names.
///
/// # Example
/// ```
/// use hookmap::*;
/// assert_eq!(Button::Key0, button_name!(0));
/// assert_eq!(Button::A, button_name!(A));
/// ```
///
#[rustfmt::skip]
#[macro_export]
macro_rules! button_name {
    ($button:ident) => (Button::$button);
    (0)             => (Button::Key0);
    (1)             => (Button::Key1);
    (2)             => (Button::Key2);
    (3)             => (Button::Key3);
    (4)             => (Button::Key4);
    (5)             => (Button::Key5);
    (6)             => (Button::Key6);
    (7)             => (Button::Key7);
    (8)             => (Button::Key8);
    (9)             => (Button::Key9);
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
/// press!(Delete with [Ctrl, Alt]);
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
