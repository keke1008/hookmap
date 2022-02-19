//! Definition of utility hotkeys.

use crate::{buttons, hotkey::button_arg::ButtonArg, hotkey::RegisterHotkey, seq};
use std::sync::atomic::{AtomicBool, Ordering};

static IS_ALT_TAB_WORKING: AtomicBool = AtomicBool::new(false);

/// Utility hotkeys.
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
    fn bind_alt_tab(&self, alt: ButtonArg, tab: ButtonArg) {
        self.on_release(alt.clone(), move |_| {
            IS_ALT_TAB_WORKING.store(false, Ordering::SeqCst);
            seq!(LAlt up).send();
        });
        let mod_alt = self.add_modifier_keys(buttons!([alt]));
        mod_alt.disable(tab.clone());
        mod_alt.on_press(tab, move |_| {
            if !IS_ALT_TAB_WORKING.swap(true, Ordering::SeqCst) {
                seq!(LAlt down).send();
            }
            seq!(Tab).send();
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
    fn bind_shift_alt_tab(&self, alt: ButtonArg, tab: ButtonArg) {
        self.on_release(alt.clone(), move |_| {
            IS_ALT_TAB_WORKING.store(false, Ordering::SeqCst);
            seq!(LAlt up).send();
        });
        let mod_alt = self.add_modifier_keys(buttons!([alt]));
        mod_alt.disable(tab.clone());
        mod_alt.on_press(tab, move |_| {
            if !IS_ALT_TAB_WORKING.swap(true, Ordering::SeqCst) {
                seq!(with(LShift), LAlt down).send();
            }
            seq!(Tab).send();
        });
    }
}

impl<T: RegisterHotkey> Utils for T {}
