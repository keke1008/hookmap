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
// Using `#[rustfmt_skip]` instead, the following error is generated.
// error: macro-expanded `macro_export` macros from the current crate cannot be referred to by absolute paths
#[allow(clippy::deprecated_cfg_attr)]
#[cfg_attr(rustfmt, rustfmt_skip)]
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
/// * modifier
/// * block_event
/// * unblock_event
///
/// ## bind
///
/// Binds the specified button as another button.
///
/// ```
/// use hookmap::*;
/// let hotkey = Hotkey::new();
/// hotkey!(hotkey => {
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
/// let hotkey = Hotkey::new();
/// hotkey!(hotkey => {
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
/// let hotkey = Hotkey::new();
/// hotkey!(hotkey => {
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
/// let hotkey = Hotkey::new();
/// hotkey!(hotkey => {
///     on_press_or_release A => |event| {};
/// });
/// ```
///
/// ## disable
///
/// Disables the specified button, `MouseMove`, or `MouseWheel`.
///
/// ```
/// use hookmap::*;
/// let hotkey = Hotkey::new();
/// hotkey!(hotkey => {
///     disable A;
///     disable MouseMove;
///     disable MouseWheel;
/// });
/// ```
///
/// ## mouse_cursor
///
/// Registers a function to be called when the mouse cursor is moved.
///
/// ```
/// use hookmap::*;
/// let hotkey = Hotkey::new();
/// hotkey!(hotkey => {
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
/// let hotkey = Hotkey::new();
/// hotkey!(hotkey => {
///     mouse_wheel => |speed| {};
/// });
/// ```
///
/// ## modifier (modifier, ...) { ... }
///
/// Adds modifier keys to hotkeys defined enclosed in Curly brackets.
/// The "!" in front of the button indicates that the button is released.
///
/// ```
/// use hookmap::*;
/// let hotkey = Hotkey::new();
/// hotkey!(hotkey => {
///     modifier (LShift, !RCtrl) {
///         bind A => B;
///     }
/// })
/// ```
///
/// ## block_event
///
/// The button/mouse event will be blocked if the hotkey defined in this statement is executed.
///
/// ```
/// use hookmap::*;
/// let hotkey = Hotkey::new();
/// hotkey!(hotkey => {
///     block_event {
///         on_press A => |_| {};
///     }
/// });
/// ```
///
/// ## unblock_event
///
/// The button/mouse event will not be blocked if the hotkey defined in this statement is executed.
///
/// If the hotkeys defined in the `block_event` statement are executed at the same time,
/// the button event will be blocked.
///
/// ```
/// use hookmap::*;
/// let hotkey = Hotkey::new();
/// hotkey!(hotkey => {
///     unblock_event {
///         on_press A => |_| {};
///     }
/// });
/// ```
///
#[macro_export]
macro_rules! hotkey {
    {
        $hotkey:expr => {
            $($cmd:tt)*
        }
    } => {{
        let hotkey = &$hotkey;
        hotkey!(@command hotkey $($cmd)*);
    }};

    (@command $hotkey:ident) => {};

    // Matches `bind`.
    (@command $hotkey:ident bind $lhs:tt => $rhs:tt; $($rest:tt)*) => {
        $hotkey.bind(button_name!($lhs)).like(button_name!($rhs));
        hotkey!(@command $hotkey $($rest)*)
    };

    // Matches `on_perss`.
    (@command $hotkey:ident on_press $lhs:tt => $rhs:expr; $($rest:tt)*) => {
        $hotkey.bind(button_name!($lhs)).on_press($rhs);
        hotkey!(@command $hotkey $($rest)*)
    };

    // Matches `on_release`.
    (@command $hotkey:ident on_release $lhs:tt => $rhs:expr; $($rest:tt)*) => {
        $hotkey.bind(button_name!($lhs)).on_release($rhs);
        hotkey!(@command $hotkey $($rest)*)
    };

    // Matches `on_press_or_release`.
    (@command $hotkey:ident on_press_or_release $lhs:tt => $rhs:expr; $($rest:tt)*) => {
        $hotkey.bind(button_name!($lhs)).on_press_or_release($rhs);
        hotkey!(@command $hotkey $($rest)*)
    };

    // Matches `disable MouseMove`.
    (@command $hotkey:ident disable MouseMove; $($rest:tt)*) => {
        $hotkey.bind_mouse_cursor().disable();
        hotkey!(@command $hotkey $($rest)*)
    };

    // Matches `disable MouseWheel`.
    (@command $hotkey:ident disable MouseWheel; $($rest:tt)*) => {
        $hotkey.bind_mouse_wheel().disable();
        hotkey!(@command $hotkey $($rest)*)
    };

    // Matches `disable $button`.
    (@command $hotkey:ident disable $lhs:tt; $($rest:tt)*) => {
        $hotkey.bind(button_name!($lhs)).disable();
        hotkey!(@command $hotkey $($rest)*)
    };

    // Matches `mouse_cursor`.
    (@command $hotkey:ident mouse_cursor => $lhs:expr; $($rest:tt)*) => {
        $hotkey.bind_mouse_cursor().on_move($lhs);
        hotkey!(@command $hotkey $($rest)*)
    };

    // Matches `modifier`.
    (@command $hotkey:ident modifier ($($button:tt)*) { $($cmd:tt)* } $($rest:tt)*) => {
        {
            let $hotkey = $hotkey.add_modifiers(hotkey!(@modifier ([], []) $($button)*));
            hotkey!(@command $hotkey $($cmd)*);
        }
        hotkey!(@command $hotkey $($rest)*);
    };

    // Matches `modifier(...)`
    (@modifier ([$($pressed:tt),*], [$($released:tt),*])) => {
        (&[$($pressed),*], &[$($released),*])
    };

    // Matches `modifier(...)`
    (@modifier ([$($pressed:tt),*], [$($released:tt),*]) !$button:tt $(, $($rest:tt)*)?) => {
        hotkey!(@modifier ([$($pressed),*], [$($released,)* (button_name!($button))]) $($($rest)*)?)
    };

    // Matches `modifier(...)`
    (@modifier ([$($pressed:tt),*], [$($released:tt),*]) $button:tt $(,)? $(, $($rest:tt)*)?) => {
        hotkey!(@modifier ([$($pressed,)* (button_name!($button))], [$($released),*]) $($($rest)*)?)
    };

    // Matches `mouse_wheel`.
    (@command $hotkey:ident mouse_wheel => $lhs:expr; $($rest:tt)*) => {
        $hotkey.bind_mouse_wheel().on_rotate($lhs);
        hotkey!(@command $hotkey $($rest)*)
    };

    // Matches `block_event`.
    (@command $hotkey:ident block_event { $($cmd:tt)* } $($rest:tt)*) => {
        {
            let $hotkey = $hotkey.block();
            hotkey!(@command $hotkey $($cmd)*);
        }
        hotkey!(@command $hotkey $($rest)*);
    };

    // Matches `unblock_event`.
    (@command $hotkey:ident unblock_event { $($cmd:tt)* } $($rest:tt)*) => {
        {
            let $hotkey = $hotkey.unblock();
            hotkey!(@command $hotkey $($cmd)*);
        }
        hotkey!(@command $hotkey $($rest)*);
    }
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
            .filter(|button| button.is_pressed())
            .collect::<Vec<_>>();
        pressed_modifiers.iter().for_each(|button| button.release());
        $(
            send!(@single button_name!($button) $(, $modifier)?);
        )*
        pressed_modifiers.iter().for_each(|button| button.press());

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

#[macro_export]
macro_rules! any {
    ($($button:tt),* $(,)?) => {
        $crate::button::Any::new([$($crate::button_name!($button)),*])
    };
}

#[macro_export]
macro_rules! all {
    ($($button:tt),* $(,)?) => {
        $crate::button::All::new([$(button_name!($button)),*])
    };
}
