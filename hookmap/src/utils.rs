//! Definition of utility hotkeys.

use crate::{hotkey::button_arg::ButtonArg, hotkey::RegisterHotkey, seq};
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
    /// hotkey.bind_alt_tab(Button::A, Button::T);
    /// ```
    ///
    fn bind_alt_tab(&self, alt: impl Into<ButtonArg>, tab: impl Into<ButtonArg>) {
        let alt = alt.into();
        let tab = tab.into();
        self.on_release(&alt, move |_| {
            IS_ALT_TAB_WORKING.store(false, Ordering::SeqCst);
            seq!(LAlt up).send();
        });
        let mod_alt = self.add_modifier_keys(alt);
        mod_alt.disable(&tab);
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
    /// hotkey.bind_shift_alt_tab(Button::A, Button::T);
    /// ```
    ///
    fn bind_shift_alt_tab(&self, alt: impl Into<ButtonArg>, tab: impl Into<ButtonArg>) {
        let alt = alt.into();
        let tab = tab.into();
        self.on_release(&alt, move |_| {
            IS_ALT_TAB_WORKING.store(false, Ordering::SeqCst);
            seq!(LAlt up).send();
        });
        let mod_alt = self.add_modifier_keys(alt);
        mod_alt.disable(&tab);
        mod_alt.on_press(tab, move |_| {
            if !IS_ALT_TAB_WORKING.swap(true, Ordering::SeqCst) {
                seq!(with(LShift), LAlt down).send();
            }
            seq!(Tab).send();
        });
    }
}

impl<T: RegisterHotkey> Utils for T {}
