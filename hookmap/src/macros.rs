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

/// Registers hotkeys.
///
/// # Commands
///
/// * bind
/// * on_press
/// * on_release
/// * on_press_or_release
/// * disable
/// * mouse_cursor
/// * mouse_wheel
///
/// ## bind
///
/// Binds the specified button as another button.
///
/// ```
/// use hookmap::*;
/// let hook = Hook::new();
/// hotkey!(hook => {
///     bind A => B;
/// });
/// ```
///
/// ## on_press
///
/// Registers a function to be called when the specified button is pressed.
///
/// ```
/// use hookmap::*;
/// let hook = Hook::new();
/// hotkey!(hook => {
///     on_press A => |event| {};
/// });
/// ```
///
/// ## on_release
///
/// Registers a function to be called when the specified button is released.
///
/// ```
/// use hookmap::*;
/// let hook = Hook::new();
/// hotkey!(hook => {
///     on_release A => |event| {};
/// });
/// ```
///
/// ## on_press_or_release
///
/// Registers a function to be called when the specified button is pressed or releaesd.
///
/// ```
/// use hookmap::*;
/// let hook = Hook::new();
/// hotkey!(hook => {
///     on_press_or_release A => |event| {};
/// });
/// ```
///
/// ## disable
///
/// Disables the specified button.
///
/// ```
/// use hookmap::*;
/// let hook = Hook::new();
/// hotkey!(hook => {
///     disable A;
/// });
/// ```
///
/// ## mouse_cursor
///
/// Registers a function to be called when the mouse cursor is moved.
///
/// ```
/// use hookmap::*;
/// let hook = Hook::new();
/// hotkey!(hook => {
///     mouse_cursor => |(x, y)| {};
/// })
/// ```
///
/// ## mouse_wheel
///
/// Registers a function to be called when the mouse wheel is rotated.
///
/// ```
/// use hookmap::*;
/// let hook = Hook::new();
/// hotkey!(hook => {
///     mouse_wheel => |speed| {};
/// });
/// ```
///
#[macro_export]
macro_rules! hotkey {
    {
        $hook:expr => {
            $($cmd:tt)*
        }
    } => {{
        let hook = &$hook;
        hotkey!(hook $($cmd)*);
    }};

    ($hook:ident) => {};

    ($hook:ident bind $lhs:tt => $rhs:tt; $($rest:tt)*) => {
        $hook.bind(button_name!($lhs)).like(button_name!($rhs));
        hotkey!($hook $($rest)*)
    };

    ($hook:ident on_press $lhs:tt => $rhs:expr; $($rest:tt)*) => {
        $hook.bind(button_name!($lhs)).on_press($rhs);
        hotkey!($hook $($rest)*)
    };

    ($hook:ident on_release $lhs:tt => $rhs:expr; $($rest:tt)*) => {
        $hook.bind(button_name!($lhs)).on_release($rhs);
        hotkey!($hook $($rest)*)
    };

    ($hook:ident on_press_or_release $lhs:tt => $rhs:expr; $($rest:tt)*) => {
        $hook.bind(button_name!($lhs)).on_press_or_release($rhs);
        hotkey!($hook $($rest)*)
    };

    ($hook:ident disable $lhs:tt; $($rest:tt)*) => {
        $hook.bind(button_name!($lhs)).disable();
        hotkey!($hook $($rest)*)
    };

    ($hook:ident mouse_cursor => $lhs:expr; $($rest:tt)*) => {
        $hook.bind_mouse_cursor().on_move($lhs);
        hotkey!($hook $($rest)*)
    };

    ($hook:ident mouse_wheel => $lhs:expr; $($rest:tt)*) => {
        $hook.bind_mouse_wheel().on_rotate($lhs);
        hotkey!($hook $($rest)*)
    };
}

use hookmap_core::Button;

pub static MODIFIER_LIST: [Button; 8] = [
    Button::LShift,
    Button::RShift,
    Button::LCtrl,
    Button::RCtrl,
    Button::LAlt,
    Button::RAlt,
    Button::LMeta,
    Button::RMeta,
];

/// Ignores the modifier keys and sends the input events.
///
/// # Example
///
/// ```no_run
/// use hookmap::*;
/// send!(A, B, C);
/// ```
///
/// Variables can be used by enclosing them in square brackets.
///
/// ```no_run
/// use hookmap::*;
/// let btn = Button::A;
/// send!([btn]);
/// ```
///
/// Use `down` and `up` to press and release keys.
///
/// ```no_run
/// use hookmap::*;
/// send!(LCtrl down, A, LCtrl up);
/// ```
///
#[macro_export]
macro_rules! send {
    ($($button:tt $($modifier:ident)?),*) => {{
        let pressed_modifiers = $crate::macros::MODIFIER_LIST
            .iter()
            .filter(|button| button.is_pressed());
        pressed_modifiers.clone().for_each(ButtonInput::release);
        $(
            send!(@single button_name!($button) $(, $modifier)?);
        )*
        pressed_modifiers.for_each(ButtonInput::press);

    }};

    (@single $button:expr) => {
        $button.click()
    };

    (@single $button:expr, down) => {
        $button.press()
    };

    (@single $button:expr, up) => {
        $button.release()
    };
}
