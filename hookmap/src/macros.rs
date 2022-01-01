use crate::button::Button;
use std::iter::{self, FromIterator};

#[derive(Clone, Copy)]
pub enum ButtonArg {
    Direct(Button),
    Inversion(Button),
}

impl ButtonArg {
    fn invert(&self) -> Self {
        match *self {
            ButtonArg::Direct(button) => ButtonArg::Inversion(button),
            ButtonArg::Inversion(button) => ButtonArg::Direct(button),
        }
    }
}

/// A struct used in macros to pass multiple buttons to a function.
#[derive(Clone)]
pub struct ButtonArgs(Vec<ButtonArg>);

impl ButtonArgs {
    pub fn iter(&self) -> impl Iterator<Item = ButtonArg> + '_ {
        self.0.iter().copied()
    }

    pub fn iter_inverse(&self) -> impl Iterator<Item = ButtonArg> + '_ {
        self.iter().map(|arg| arg.invert())
    }
}

impl FromIterator<Box<dyn Iterator<Item = ButtonArg>>> for ButtonArgs {
    fn from_iter<T: IntoIterator<Item = Box<dyn Iterator<Item = ButtonArg>>>>(iter: T) -> Self {
        ButtonArgs(Vec::from_iter(iter.into_iter().flatten()))
    }
}

pub trait ExpandButtonArg {
    fn expand(&self) -> Box<dyn Iterator<Item = ButtonArg> + '_>;
    fn expand_inverse(&self) -> Box<dyn Iterator<Item = ButtonArg> + '_> {
        Box::new(self.expand().map(|e| e.invert()))
    }
}

impl ExpandButtonArg for ButtonArgs {
    fn expand(&self) -> Box<dyn Iterator<Item = ButtonArg> + '_> {
        Box::new(self.0.iter().copied())
    }
}

impl ExpandButtonArg for Button {
    fn expand(&self) -> Box<dyn Iterator<Item = ButtonArg> + '_> {
        Box::new(iter::once(ButtonArg::Direct(*self)))
    }
}

#[macro_export]
macro_rules! parse_button_args {
    (@inner $parsed:tt , $($rest:tt)*) => {
        $crate::parse_button_args!(@inner $parsed $($rest)*)
    };

    (@inner [ $($parsed:tt)* ] !$button:tt $($rest:tt)*) => {
        $crate::parse_button_args!(@inner [ $($parsed)* ($crate::macros::ExpandButtonArg::expand_inverse(&$crate::button_name!($button))) ] $($rest)*)
    };

    (@inner [ $($parsed:tt)* ] $button:tt $($rest:tt)*) => {
        $crate::parse_button_args!(@inner [ $($parsed)* ($crate::macros::ExpandButtonArg::expand(&$crate::button_name!($button))) ] $($rest)*)
    };

    (@inner [ $($parsed:tt)* ]) => {
        IntoIterator::into_iter(
            [ $($parsed),* ]
        )
        .collect::<$crate::macros::ButtonArgs>()
    };

    ($($args:tt)*) => {
        $crate::parse_button_args!(@inner [] $($args)*)
    };
}

/// Expands button names.
///
/// If the argument is enclosed in square brackets, it will be expanded without any action.
///
/// # Example
/// ```no_run
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
    (Shift)          => ($crate::button::SHIFT);
    (Ctrl)           => ($crate::button::Ctrl);
    (Alt)            => ($crate::button::Alt);
    (Meta)           => ($crate::button::Meta);
    ($button:ident)  => ($crate::button::Button::$button);
    (0)              => ($crate::button::Button::Key0);
    (1)              => ($crate::button::Button::Key1);
    (2)              => ($crate::button::Button::Key2);
    (3)              => ($crate::button::Button::Key3);
    (4)              => ($crate::button::Button::Key4);
    (5)              => ($crate::button::Button::Key5);
    (6)              => ($crate::button::Button::Key6);
    (7)              => ($crate::button::Button::Key7);
    (8)              => ($crate::button::Button::Key8);
    (9)              => ($crate::button::Button::Key9);
    (;)              => ($crate::button::Button::SemiColon);
    (-)              => ($crate::button::Button::Minus);
    (/)              => ($crate::button::Button::Slash);
}

/// Registers hotkeys.
///
/// # Commands
///
/// * [remap](#remap)
/// * [on_press](#on_press)
/// * [on_release](#on_release)
/// * [disable](#disable)
/// * [mouse_cursor](#mouse_cursor)
/// * [mouse_wheel](#mouse_wheel)
/// * [modifier](#modifier)
/// * [block](#block)
/// * [dispatch](#dispatch)
/// * [call](#call)
///
/// ## remap
///
/// Remap keys.
///
/// ```no_run
/// use hookmap::*;
/// let hotkey = Hotkey::new();
/// hotkey!(hotkey => {
///     remap A => B;
/// });
/// ```
///
/// ## on_press
///
/// Registers a function to be called when the specified button is pressed.
///
/// ```no_run
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
/// ```no_run
/// use hookmap::*;
/// let hotkey = Hotkey::new();
/// hotkey!(hotkey => {
///     on_release A => |event| {};
/// });
/// ```
///
/// ## disable
///
/// Disables the specified button.
///
/// ```no_run
/// use hookmap::*;
/// let hotkey = Hotkey::new();
/// hotkey!(hotkey => {
///     disable A;
/// });
/// ```
///
/// ## mouse_cursor
///
/// Registers a function to be called when the mouse cursor is moved.
///
/// ```no_run
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
/// ```no_run
/// use hookmap::*;
/// let hotkey = Hotkey::new();
/// hotkey!(hotkey => {
///     mouse_wheel => |speed| {};
/// });
/// ```
///
/// ## modifier
///
/// Adds modifier keys to hotkeys defined enclosed in Curly brackets.
/// The "!" in front of the button indicates that the button is released.
///
/// ```no_run
/// use hookmap::*;
/// let hotkey = Hotkey::new();
/// hotkey!(hotkey => {
///     modifier (LShift, !RCtrl) {
///         remap A => B;
///     }
/// })
/// ```
///
/// ## block
///
/// The button/mouse event will be blocked if the hotkey defined in this statement is executed.
///
/// ```no_run
/// use hookmap::*;
/// let hotkey = Hotkey::new();
/// hotkey!(hotkey => {
///     block {
///         on_press A => |_| {};
///     }
/// });
/// ```
///
/// ## dispatch
///
/// The button/mouse event will not be blocked if the hotkey defined in this statement is executed.
///
/// If the hotkeys defined in the `block_event` statement are executed at the same time,
/// the button event will be blocked.
///
/// ```no_run
/// use hookmap::*;
/// let hotkey = Hotkey::new();
/// hotkey!(hotkey => {
///     dispatch {
///         on_press A => |_| {};
///     }
/// });
/// ```
///
/// ## call
///
/// Calls associated functions of [`SelectHandleTarget`].
///
/// [`SelectHandleTarget`]: crate::SelectHandleTarget
///
/// ```no_run
/// use hookmap::*;
/// trait RemapAsTab: SelectHandleTarget {
///     fn remap_as_tab(&self, target: Button) {
///         hotkey!(self => {
///             remap [target] => Tab;
///         });
///     }
/// }
/// impl<T: SelectHandleTarget> RemapAsTab for T {}
///
/// let hotkey = Hotkey::new();
/// hotkey!(hotkey => {
///     call remap_as_tab(A);
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
        $crate::hotkey!(@command hotkey $($cmd)*);
    }};

    // Terminate
    (@command $hotkey:ident) => {};

    // Ignored token: =>
    (@parse_button_args_until_ignored_tokens $hotkey:ident $command:ident [ $($collected:tt)* ] => $($rest:tt)*) => {
        $crate::hotkey!(@$command $hotkey ( $crate::parse_button_args!($($collected)*) ) $($rest)*)
    };

    // Ignored token: ;
    (@parse_button_args_until_ignored_tokens $hotkey:ident $command:ident [ $($collected:tt)* ]; $($rest:tt)*) => {
        $crate::hotkey!(@$command $hotkey ( $crate::parse_button_args!($($collected)*) ) $($rest)*)
    };

    // Ignored token: { }
    (@parse_button_args_until_ignored_tokens $hotkey:ident $command:ident [ $($collected:tt)* ] { $($rest1:tt)* } $($rest2:tt)*) => {
        $crate::hotkey!(@$command $hotkey ( $crate::parse_button_args!($($collected)*) ) { $($rest1)* } $($rest2)*)
    };

    // Munch tokens
    (@parse_button_args_until_ignored_tokens $hotkey:ident $command:ident [ $($collected:tt)* ] $button:tt $($rest:tt)*) => {
        $crate::hotkey!(@parse_button_args_until_ignored_tokens $hotkey $command [ $($collected)* $button ] $($rest)*)
    };

    // Matches `remap`
    (@remap $hotkey:ident $parsed:tt $rhs:tt; $($rest:tt)*) => {
        $hotkey.remap($parsed, $crate::button_name!($rhs));
        $crate::hotkey!(@command $hotkey $($rest)*);
    };

    // Matches `remap`.
    (@command $hotkey:ident remap $($rest:tt)*) => {
        $crate::hotkey!(@parse_button_args_until_ignored_tokens $hotkey remap [] $($rest)*)
    };

    // Matches `on_perss`.
    (@on_press $hotkey:ident $parsed:tt $rhs:expr; $($rest:tt)*) => {
        $hotkey.on_press($parsed, std::sync::Arc::new($rhs));
        $crate::hotkey!(@command $hotkey $($rest)*)
    };

    // Matches `on_perss`.
    (@command $hotkey:ident on_press $($rest:tt)*) => {
        $crate::hotkey!(@parse_button_args_until_ignored_tokens $hotkey on_press [] $($rest)*)
    };

    // Matches `on_release`.
    (@on_release $hotkey:ident $parsed:tt $rhs:expr; $($rest:tt)*) => {
        $hotkey.on_release($parsed, std::sync::Arc::new($rhs));
        $crate::hotkey!(@command $hotkey $($rest)*)
    };

    // Matches `on_release`.
    (@command $hotkey:ident on_release $($rest:tt)*) => {
        $crate::hotkey!(@parse_button_args_until_ignored_tokens $hotkey on_release [] $($rest)*)
    };

    // Matches `disable`.
    (@disable $hotkey:ident $parsed:tt $($rest:tt)*) => {
        $hotkey.disable($parsed);
        $crate::hotkey!(@command $hotkey $($rest)*)
    };

    // Matches `disable`.
    (@command $hotkey:ident disable $($rest:tt)*) => {
        $crate::hotkey!(@parse_button_args_until_ignored_tokens $hotkey disable [] $($rest)*)
    };

    // Matches `mouse_cursor`.
    (@command $hotkey:ident mouse_cursor => $lhs:expr; $($rest:tt)*) => {
        $hotkey.mouse_cursor(std::sync::Arc::new($lhs));
        $crate::hotkey!(@command $hotkey $($rest)*)
    };

    // Matches `mouse_wheel`.
    (@command $hotkey:ident mouse_wheel => $lhs:expr; $($rest:tt)*) => {
        $hotkey.mouse_wheel(std::sync::Arc::new($lhs));
        $crate::hotkey!(@command $hotkey $($rest)*)
    };

    // Matches `modifier`
    (@modifier $hotkey:ident $parsed:tt { $($cmd:tt)* } $($rest:tt)*) => {
        {
            let modifier_keys = $crate::hotkey::ModifierKeys::new($parsed);
            #[allow(unused_variables)]
            let $hotkey = $hotkey.add_modifier_keys(modifier_keys);
            $crate::hotkey!(@command $hotkey $($cmd)*);
        }
        $crate::hotkey!(@command $hotkey $($rest)*);
    };

    // Matches `modifier`
    (@command $hotkey:ident modifier $($rest:tt)*) => {
        $crate::hotkey!(@parse_button_args_until_ignored_tokens $hotkey modifier [] $($rest)*)
    };

    // Matches `block`
    (@command $hotkey:ident block { $($cmd:tt)* } $($rest:tt)*) => {
        {
            #[allow(unused_variables)]
            let $hotkey = $hotkey.change_native_event_operation($crate::event::NativeEventOperation::Block);
            $crate::hotkey!(@command $hotkey $($cmd)*);
        }
        $crate::hotkey!(@command $hotkey $($rest)*);
    };

    // Matches `dispatch`.
    (@command $hotkey:ident dispatch { $($cmd:tt)* } $($rest:tt)*) => {
        {
            #[allow(unused_variables)]
            let $hotkey = $hotkey.change_native_event_operation($crate::event::NativeEventOperation::Dispatch);
            $crate::hotkey!(@command $hotkey $($cmd)*);
        }
        $crate::hotkey!(@command $hotkey $($rest)*);
    };

    // Matches `call`.
    (@command $hotkey:ident call $name:ident($($arg:tt),*); $($rest:tt)*) => {
        $hotkey.$name(
            $($crate::button_name!($arg)),*
        );
        $crate::hotkey!(@command $hotkey $($rest)*);
    };
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
    // trailing comma case
    (with($($modifier:tt)*) $(, $($button:tt $($action:ident)?),*)? ,) => {
        $crate::seq!(with($($modifier)*) $(, $($button$($action)?),*)?)
    };

    (with($($modifier:tt),*) $(, $($rest:tt)*)?) => {
        $crate::seq!($($modifier down,)* $($($rest)*,)? $($modifier up),*)
    };

    ($($button:tt $($action:ident)?),* $(,)?) => {
        $(
            $crate::seq!(@single $crate::button_name!($button) $(, $action)?);
        )*
    };

    (@single $button:expr) => {
        $crate::button::ButtonInput::click(&$button);
    };

    (@single $button:expr, down) => {
        $crate::button::ButtonInput::press(&$button);
    };

    (@single $button:expr, up) => {
        $crate::button::ButtonInput::release(&$button);
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

/// Ignores the modifier keys and sends the input events.
///
/// # Examples
///
/// ```no_run
/// use hookmap::*;
/// send!(A, B, C);
/// ```
///
/// Use `down` and `up` to press and release keys.
///
/// ```no_run
/// use hookmap::*;
/// send!(LCtrl down, A, LCtrl up);
/// ```
///
/// Use `with(...)` to specify the keys to hold down when sending.
///
/// ```no_run
/// use hookmap::*;
/// send!(with(LShift, LCtrl), Tab);
/// send!(LShift down, LCtrl down, Tab, LShift up, LCtrl up); // equals to above
/// ```
///
#[macro_export]
macro_rules! send {
    ($($input:tt)*) => {{
        let pressed_modifiers = $crate::macros::MODIFIER_LIST
            .iter()
            .filter(|button| $crate::button::ButtonState::is_pressed(button))
            .collect::<Vec<_>>();
        pressed_modifiers.iter().for_each(|button| $crate::button::ButtonInput::release(button));
        $crate::seq!($($input)*);
        pressed_modifiers.iter().for_each(|button| $crate::button::ButtonInput::press(button));
    }};
}

#[cfg(test)]
mod tests {
    use crate::{
        button::{Button, ALT, CTRL, META, SHIFT},
        hotkey::{Hotkey, RegisterHotkey},
    };

    #[test]
    fn remap() {
        hotkey!(Hotkey::new() => {
            remap A => B;
            remap A, B => C;
            remap Shift => B;
            remap [Button::A], [SHIFT] => [Button::B];
            remap A, [Button::B], [SHIFT] => A;
        });
    }

    #[test]
    fn on_press_command() {
        hotkey!(Hotkey::new() => {
            on_press A => |_| {};
            on_press A, B => |_| {};
            on_press [Button::A] => |_| {};
            on_press [Button::A], [Button::B] => |_| {};
            on_press [SHIFT] => |_| {};
            on_press A, [Button::B], [SHIFT] => |_| {};
        });
    }

    #[test]
    fn on_release_command() {
        hotkey!(Hotkey::new() => {
            on_press A => |_| {};
            on_press A, B => |_| {};
            on_press [Button::A] => |_| {};
            on_press [Button::A], [Button::B] => |_| {};
            on_press [SHIFT] => |_| {};
            on_press A, [Button::B], [SHIFT] => |_| {};
        });
    }

    #[test]
    fn disable_command() {
        hotkey!(Hotkey::new() => {
            disable A;
            disable A, B;
            disable [Button::A];
            disable [Button::A], [Button::B];
            disable [SHIFT];
            disable A, [Button::B], [SHIFT];
        });
    }

    #[test]
    fn mouse_cursor_command() {
        hotkey!(Hotkey::new() => {
            mouse_cursor => |_| {};
        });
    }

    #[test]
    fn mouse_wheel_command() {
        hotkey!(Hotkey::new() => {
            mouse_wheel => |_| {};
        });
    }

    #[test]
    fn modifier_command() {
        hotkey!(Hotkey::new() => {
            modifier A {}
            modifier A, B {}
            modifier !A {}
            modifier A, !A, !B, C {}
            modifier [Button::A], ![Button::B] {}
            modifier ![SHIFT], [CTRL], ![ALT] {}
            modifier ![META] {
                modifier A {}
            }
            modifier A {
                remap A => B;
            }
        });
    }

    #[test]
    fn block_command() {
        hotkey!(Hotkey::new() => {
            block {}
            block {
                dispatch {
                    remap A => B;
                }
            }
        });
    }

    #[test]
    fn dispatch_command() {
        hotkey!(Hotkey::new() => {
            dispatch {}
            dispatch {
                block {
                    remap A => B;
                }
            }
        });
    }

    #[test]
    fn button_name_macro() {
        assert_eq!(button_name!(A), Button::A);
        assert_eq!(button_name!([Button::LShift]), Button::LShift);
    }

    #[test]
    #[ignore = "This function sends keyboard input"]
    fn seq_macro() {
        seq!();
        seq!(A, B);
        seq!(A,);
        seq!([Button::A], [Button::B]);
        seq!([CTRL], [SHIFT]);
        seq!(A up, B down, [CTRL] up);
        seq!(with(A));
        seq!(with(A),);
        seq!(with(A), C,);
        seq!(with(A, B), C);
        seq!(with([Button::A], [SHIFT]), [CTRL]);
    }

    #[test]
    #[ignore = "This function sends keyboard input"]
    fn send_macro() {
        send!();
        send!(A, B);
        send!(A,);
        send!([Button::A], [Button::B]);
        send!([CTRL], [SHIFT]);
        send!(A up, B down, [CTRL] up);
        send!(with(A));
        send!(with(A),);
        send!(with(A), C,);
        send!(with(A, B), C);
        send!(with([Button::A], [SHIFT]), [CTRL]);
    }
}
