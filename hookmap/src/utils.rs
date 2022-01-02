use crate::{hotkey, hotkey::RegisterHotkey, macros::ButtonArgs, seq};
use std::sync::atomic::{AtomicBool, Ordering};

static IS_ALT_TAB_WORKING: AtomicBool = AtomicBool::new(false);

/// Utility function.
pub trait Utils: RegisterHotkey {
    /// Alt-Tab hotkey.
    ///
    /// # Arguments
    ///
    /// * `alt` - A button that act like Alt key.
    /// * `tab` - A button that act like tab key.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::prelude::*;
    ///
    /// let hotkey = Hotkey::new();
    /// hotkey!(hotkey => {
    ///     call bind_alt_tab([button_args!(A)], [button_args!(T)]);
    /// });
    /// ```
    ///
    fn bind_alt_tab(&self, alt: ButtonArgs, tab: ButtonArgs) {
        hotkey!(self => {
            on_release [alt] => move |_| {
                IS_ALT_TAB_WORKING.store(false, Ordering::SeqCst);
                seq!(LAlt up);
            };

            modifier [alt] {
                disable [tab];
                on_press [tab] => move |_| {
                    if !IS_ALT_TAB_WORKING.swap(true, Ordering::SeqCst) {
                        seq!(LAlt down);
                    }
                    seq!(Tab);
                };
            }
        });
    }

    /// Shift-Alt-Tab hotkey.
    ///
    /// # Arguments
    ///
    /// * `alt` - A button that act like Alt key.
    /// * `tab` - A button that act like tab key.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::prelude::*;
    ///
    /// let hotkey = Hotkey::new();
    /// hotkey!(hotkey => {
    ///     call bind_shift_alt_tab([button_args!(A)], [button_args!(T)]);
    /// });
    /// ```
    ///
    fn bind_shift_alt_tab(&self, alt: ButtonArgs, tab: ButtonArgs) {
        hotkey!(self => {
            on_release [alt] => move |_| {
                IS_ALT_TAB_WORKING.store(false, Ordering::SeqCst);
                seq!(LAlt up);
            };

            modifier [alt] {
                disable [tab];
                on_press [tab] => move |_| {
                    if !IS_ALT_TAB_WORKING.swap(true, Ordering::SeqCst) {
                        seq!(LAlt down);
                    }
                    seq!(with(LShift), Tab);
                };
            }
        });
    }
}

impl<T: RegisterHotkey> Utils for T {}
