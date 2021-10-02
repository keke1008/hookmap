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
/// * [bind](#bind)
/// * [on_press](#on_press)
/// * [on_release](#on_release)
/// * [on_press_or_release](#on_press_or_release)
/// * [on_press_and_release](#on_press_and_release)
/// * [disable](#disable)
/// * [mouse_cursor](#mouse_cursor)
/// * [mouse_wheel](#mouse_wheel)
/// * [modifier](#modifier)
/// * [block_event](#block_event)
/// * [unblock_event](#unblock_event)
/// * [call](#call)
///
/// ## bind
///
/// Binds the specified button as another button.
///
/// ```no_run
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
/// ## on_press_or_release
///
/// Registers a function to be called when the specified button is pressed or releaesd.
///
/// ```no_run
/// use hookmap::*;
/// let hotkey = Hotkey::new();
/// hotkey!(hotkey => {
///     on_press_or_release A => |event| {};
/// });
/// ```
///
/// ## on_press_and_reelase
///
/// Registers a function to be called when the specified button is pressed or releaesd, respectively.
///
/// ```no_run
/// use hookmap::*;
/// let hotkey = Hotkey::new();
/// hotkey!(hotkey => {
///     on_press_and_release A => {
///         on_press => |event| {};
///         on_release => |event| {};
///     }
/// });
/// ```
///
/// ## disable
///
/// Disables the specified button, `MouseMove`, or `MouseWheel`.
///
/// ```no_run
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
/// ## modifier (modifier, ...) { ... }
///
/// Adds modifier keys to hotkeys defined enclosed in Curly brackets.
/// The "!" in front of the button indicates that the button is released.
///
/// ```no_run
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
/// ```no_run
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
/// ```no_run
/// use hookmap::*;
/// let hotkey = Hotkey::new();
/// hotkey!(hotkey => {
///     unblock_event {
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
/// trait BindAsTab: SelectHandleTarget {
///     fn bind_as_tab(&self, target: Button) {
///         hotkey!(self => {
///             bind [target] => Tab;
///         });
///     }
/// }
/// impl<T: SelectHandleTarget> BindAsTab for T {}
///
/// let hotkey = Hotkey::new();
/// hotkey!(hotkey => {
///     call bind_as_tab(A);
/// });
/// ```
///
/// # The difference between on_press_and_release and (on_press, on_release)
///
/// These will behave differently when the trigger key is released with the modifier key specified.
///
/// In the case of normal `on_release`, the specified function will be called when the trigger key
/// is released while the modifier key is pressed.
///
///
/// On the other hand, the function specified as on_release in `on_press_or_release` will be
/// called when the following conditions are met.
///
/// 1. The trigger key was pressed while the modifier key was pressed.
/// 2. Neither the trigger key nor the modifier key has been released since then.
/// 3. And when the trigger key or modifier key was released.
///
#[macro_export]
macro_rules! hotkey {
    {
        $hotkey:expr => {
            $($cmd:tt)*
        }
    } => {{
        #[allow(unused_variables)]
        let hotkey = &$hotkey;
        $crate::hotkey!(@command hotkey $($cmd)*);
    }};

    (@command $hotkey:ident) => {};

    // Matches `bind`.
    (@command $hotkey:ident bind $lhs:tt => $rhs:tt; $($rest:tt)*) => {
        $hotkey.bind($crate::button_name!($lhs)).like($crate::button_name!($rhs));
        $crate::hotkey!(@command $hotkey $($rest)*)
    };

    // Matches `on_perss`.
    (@command $hotkey:ident on_press $lhs:tt => $rhs:expr; $($rest:tt)*) => {
        $hotkey.bind($crate::button_name!($lhs)).on_press($rhs);
        $crate::hotkey!(@command $hotkey $($rest)*)
    };

    // Matches `on_release`.
    (@command $hotkey:ident on_release $lhs:tt => $rhs:expr; $($rest:tt)*) => {
        $hotkey.bind($crate::button_name!($lhs)).on_release($rhs);
        $crate::hotkey!(@command $hotkey $($rest)*)
    };

    // Matches `on_press_or_release`.
    (@command $hotkey:ident on_press_or_release $lhs:tt => $rhs:expr; $($rest:tt)*) => {
        $hotkey.bind($crate::button_name!($lhs)).on_press_or_release($rhs);
        $crate::hotkey!(@command $hotkey $($rest)*)
    };

    //Matches `on_press_and_release`
    (@command $hotkey:ident
     on_press_and_release $button:tt => {
        on_press => $press:expr;
        on_release => $release:expr;
    }
    $($rest:tt)*) => {
        $hotkey.bind($crate::button_name!($button)).on_press_and_release($press, $release);
        $crate::hotkey!(@command $hotkey $($rest)*)
    };

    // Matches `disable MouseMove`.
    (@command $hotkey:ident disable MouseMove; $($rest:tt)*) => {
        $hotkey.bind_mouse_cursor().disable();
        $crate::hotkey!(@command $hotkey $($rest)*)
    };

    // Matches `disable MouseWheel`.
    (@command $hotkey:ident disable MouseWheel; $($rest:tt)*) => {
        $hotkey.bind_mouse_wheel().disable();
        $crate::hotkey!(@command $hotkey $($rest)*)
    };

    // Matches `disable $button`.
    (@command $hotkey:ident disable $lhs:tt; $($rest:tt)*) => {
        $hotkey.bind($crate::button_name!($lhs)).disable();
        $crate::hotkey!(@command $hotkey $($rest)*)
    };

    // Matches `mouse_cursor`.
    (@command $hotkey:ident mouse_cursor => $lhs:expr; $($rest:tt)*) => {
        $hotkey.bind_mouse_cursor().on_move($lhs);
        $crate::hotkey!(@command $hotkey $($rest)*)
    };

    // Matches `modifier`.
    (@command $hotkey:ident modifier ($($button:tt)*) { $($cmd:tt)* } $($rest:tt)*) => {
        {
            #[allow(unused_variables)]
            let $hotkey = $hotkey.add_modifiers($crate::hotkey!(@modifier ([], []) $($button)*));
            $crate::hotkey!(@command $hotkey $($cmd)*);
        }
        $crate::hotkey!(@command $hotkey $($rest)*);
    };

    // Matches `modifier(...)`
    (@modifier ([$($pressed:tt),*], [$($released:tt),*])) => {
        (
            &[$($crate::button::ButtonSet::from($pressed)),*],
            &[$($crate::button::ButtonSet::from($released)),*]
        )
    };

    // Matches `modifier(...)`
    (@modifier ([$($pressed:tt),*], [$($released:tt),*]) !$button:tt $(, $($rest:tt)*)?) => {
        $crate::hotkey!(@modifier ([$($pressed),*], [$($released,)* ($crate::button_name!($button))]) $($($rest)*)?)
    };

    // Matches `modifier(...)`
    (@modifier ([$($pressed:tt),*], [$($released:tt),*]) $button:tt $(,)? $(, $($rest:tt)*)?) => {
        $crate::hotkey!(@modifier ([$($pressed,)* ($crate::button_name!($button))], [$($released),*]) $($($rest)*)?)
    };

    // Matches `mouse_wheel`.
    (@command $hotkey:ident mouse_wheel => $lhs:expr; $($rest:tt)*) => {
        $hotkey.bind_mouse_wheel().on_rotate($lhs);
        $crate::hotkey!(@command $hotkey $($rest)*)
    };

    // Matches `block_event`.
    (@command $hotkey:ident block_event { $($cmd:tt)* } $($rest:tt)*) => {
        {
            #[allow(unused_variables)]
            let $hotkey = $hotkey.block();
            $crate::hotkey!(@command $hotkey $($cmd)*);
        }
        $crate::hotkey!(@command $hotkey $($rest)*);
    };

    // Matches `unblock_event`.
    (@command $hotkey:ident unblock_event { $($cmd:tt)* } $($rest:tt)*) => {
        {
            #[allow(unused_variables)]
            let $hotkey = $hotkey.unblock();
            $crate::hotkey!(@command $hotkey $($cmd)*);
        }
        $crate::hotkey!(@command $hotkey $($rest)*);
    };

    // Matches `call`.
    (@command $hotkey:ident call $name:ident($($arg:tt),*);) => {
        $hotkey.$name(
            $($crate::button_name!($arg)),*
        );
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

/// Creates ButtonSet::Any.
///
/// # Example
///
/// ```no_run
/// use hookmap::*;
/// let hotkey = Hotkey::new();
/// let a_or_b = any!(A, B);
/// hotkey!(hotkey => {
///     on_press [a_or_b] => |e| println!("{:?} key was pressed.", e.target);
/// });
/// ```
///
#[macro_export]
macro_rules! any {
    ($($button:tt),* $(,)?) => {
        $crate::button::ButtonSet::Any(
            vec![$($crate::button_name!($button)),*]
        )
    };
}

/// Creates ButtonSet::All.
/// # Example
///
/// ```no_run
/// use hookmap::*;
/// let hotkey = Hotkey::new();
/// let a_and_b = all!(A, B);
/// hotkey!(hotkey => {
///     on_press [a_and_b] => |_| println!("A key and B key was pressed");
/// })
/// ```
#[macro_export]
macro_rules! all {
    ($($button:tt),* $(,)?) => {
        $crate::button::ButtonSet::All(
            vec![$($crate::button_name!($button)),*]
        )
    };
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn bind_command() {
        hotkey!(Hotkey::new() => {
            bind A => B;
            bind [Button::A] => [Button::B];
            bind [&SHIFT] => [&CTRL];
        });
    }

    #[test]
    fn on_press_command() {
        hotkey!(Hotkey::new() => {
            on_press A => |_| {};
            on_press [Button::A] => |_| {};
            on_press [&SHIFT] => |_| {};
        });
    }

    #[test]
    fn on_press_and_release_command() {
        hotkey!(Hotkey::new() => {
            on_press_and_release A => {
                on_press => |_| {};
                on_release => |_| {};
            }
            on_press_and_release [Button::A] => {
                on_press => |_| {};
                on_release => |_| {};
            }
            on_press_and_release [&SHIFT] => {
                on_press => |_| {};
                on_release => |_| {};
            }
        });
    }

    #[test]
    fn disable_command() {
        hotkey!(Hotkey::new() => {
            disable A;
            disable [Button::A];
            disable [&SHIFT];
        });
    }

    #[test]
    fn modifier_command() {
        hotkey!(Hotkey::new() => {
            modifier () {}
            modifier (A) {}
            modifier (!A) {}
            modifier (A, !A) {}
            modifier ([Button::A], ![Button::B]) {}
            modifier (![&SHIFT], [&CTRL], ![&ALT]) {}
            modifier (![&META]) {
                modifier (A) {}
            }
            modifier () {
                bind A => B;
            }
        });
    }

    #[test]
    fn block_event_command() {
        hotkey!(Hotkey::new() => {
            block_event {}
            block_event {
                unblock_event {
                    bind A => B;
                }
            }
        });
    }

    #[test]
    fn unblock_event_command() {
        hotkey!(Hotkey::new() => {
            unblock_event {}
            unblock_event {
                block_event {
                    bind A => B;
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
        seq!([&CTRL], [&SHIFT]);
        seq!(A up, B down, [&CTRL] up);
        seq!(with(A));
        seq!(with(A),);
        seq!(with(A), C,);
        seq!(with(A, B), C);
        seq!(with([Button::A], [&SHIFT]), [&CTRL]);
    }

    #[test]
    #[ignore = "This function sends keyboard input"]
    fn send_macro() {
        send!();
        send!(A, B);
        send!(A,);
        send!([Button::A], [Button::B]);
        send!([&CTRL], [&SHIFT]);
        send!(A up, B down, [&CTRL] up);
        send!(with(A));
        send!(with(A),);
        send!(with(A), C,);
        send!(with(A, B), C);
        send!(with([Button::A], [&SHIFT]), [&CTRL]);
    }

    #[test]
    fn any_macro() {
        use crate::button::ButtonSet;
        any!();
        any!(A,);
        assert_eq!(any!(A, B), ButtonSet::Any(vec![Button::A, Button::B]));
        assert_eq!(any!([Button::LShift]), ButtonSet::Any(vec![Button::LShift]));
    }

    #[test]
    fn all_macro() {
        use crate::button::ButtonSet;
        all!();
        all!(A,);
        assert_eq!(all!(A, B), ButtonSet::All(vec![Button::A, Button::B]));
        assert_eq!(all!([Button::LShift]), ButtonSet::All(vec![Button::LShift]));
    }
}
