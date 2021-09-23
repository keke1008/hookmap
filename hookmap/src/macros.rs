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
/// * if
/// * block_event
/// * unblock_event
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
/// Disables the specified button, `MouseMove`, or `MouseWheel`.
///
/// ```
/// use hookmap::*;
/// let hook = Hook::new();
/// hotkey!(hook => {
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
/// ## if
///
/// Adds conditions to the hotkeys defined in this statement.
/// The Following commands can be used in the condition statement.
///
/// * pressed - Checks whether the specified buttons have been pressed or not.
/// * released - Checks whether the specified buttons have been released or not.
///
/// ```
/// use hookmap::*;
/// let hook = Hook::new();
/// hotkey!(hook => {
///     if ( pressed LCtrl && released LAlt ) {
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
/// let hook = Hook::new();
/// hotkey!(hook => {
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
/// let hook = Hook::new();
/// hotkey!(hook => {
///     unblock_event {
///         on_press A => |_| {};
///     }
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

    // Matches `bind`.
    ($hook:ident bind $lhs:tt => $rhs:tt; $($rest:tt)*) => {
        $hook.bind(button_name!($lhs)).like(button_name!($rhs));
        hotkey!($hook $($rest)*)
    };

    // Matches `on_perss`.
    ($hook:ident on_press $lhs:tt => $rhs:expr; $($rest:tt)*) => {
        $hook.bind(button_name!($lhs)).on_press($rhs);
        hotkey!($hook $($rest)*)
    };

    // Matches `on_release`.
    ($hook:ident on_release $lhs:tt => $rhs:expr; $($rest:tt)*) => {
        $hook.bind(button_name!($lhs)).on_release($rhs);
        hotkey!($hook $($rest)*)
    };

    // Matches `on_press_or_release`.
    ($hook:ident on_press_or_release $lhs:tt => $rhs:expr; $($rest:tt)*) => {
        $hook.bind(button_name!($lhs)).on_press_or_release($rhs);
        hotkey!($hook $($rest)*)
    };

    // Matches `disable MouseMove`.
    ($hook:ident disable MouseMove; $($rest:tt)*) => {
        $hook.bind_mouse_cursor().disable();
        hotkey!($hook $($rest)*)
    };

    // Matches `disable MouseWheel`.
    ($hook:ident disable MouseWheel; $($rest:tt)*) => {
        $hook.bind_mouse_wheel().disable();
        hotkey!($hook $($rest)*)
    };

    // Matches `disable $button`.
    ($hook:ident disable $lhs:tt; $($rest:tt)*) => {
        $hook.bind(button_name!($lhs)).disable();
        hotkey!($hook $($rest)*)
    };

    // Matches `mouse_cursor`.
    ($hook:ident mouse_cursor => $lhs:expr; $($rest:tt)*) => {
        $hook.bind_mouse_cursor().on_move($lhs);
        hotkey!($hook $($rest)*)
    };

    // Matches `mouse_wheel`.
    ($hook:ident mouse_wheel => $lhs:expr; $($rest:tt)*) => {
        $hook.bind_mouse_wheel().on_rotate($lhs);
        hotkey!($hook $($rest)*)
    };

    // Matches `if`.
    ($hook:ident if ($($cond:tt)*) { $($cmd:tt)* } $($rest:tt)*) => {
        {
            hotkey!(@cond $hook $($cond)*);
            hotkey!($hook $($cmd)*);
        }
        hotkey!($hook $($rest)*)

    };

    // Matches `pressed` in if condition.
    (@cond $hook:ident pressed $button:tt $(&& $($rest:tt)+)?) => {
        let $hook = $hook.cond($crate::hotkey::ConditionUnit::Pressed(button_name!($button)));
        $(hotkey!(@cond $hook $($rest)+))?
    };

    // Matches `released` in if condition`
    (@cond $hook:ident released $button:tt $(&& $($rest:tt)+)?) => {
        let $hook = $hook.cond($crate::hotkey::ConditionUnit::Released(button_name!($button)));
        $(hotkey!(@cond $hook $($rest)+))?
    };

    // Matches `block_event`.
    ($hook:ident block_event { $($cmd:tt)* } $($rest:tt)*) => {
        {
            let $hook = $hook.block();
            hotkey!($hook $($cmd)*);
        }
        hotkey!($hook $($rest)*);
    };

    // Matches `unblock_event`.
    ($hook:ident unblock_event { $($cmd:tt)* } $($rest:tt)*) => {
        {
            let $hook = $hook.unblock();
            hotkey!($hook $($cmd)*);
        }
        hotkey!($hook $($rest)*);
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
