//! Definition of utility hotkeys.

use crate::{button::Sequence, hotkey::button_arg::ButtonArg, prelude::*};

fn bind_alt_tab_inner<T: RegisterHotkey + ?Sized>(
    hotkey: &T,
    alt: impl Into<ButtonArg> + Clone,
    tab: impl Into<ButtonArg> + Clone,
    tab_seq: Sequence,
) {
    hotkey.on_release(&alt, move |_| seq!(LAlt up).send());

    let mod_alt = hotkey.add_modifiers(alt);
    mod_alt.disable(&tab).on_press(tab, move |_| {
        if Button::LAlt.is_released() {
            seq!(LAlt down).send();
        }
        tab_seq.send();
    });
}

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
    fn bind_alt_tab(&self, alt: impl Clone + Into<ButtonArg>, tab: impl Into<ButtonArg> + Clone) {
        bind_alt_tab_inner(self, alt, tab, seq!(Tab));
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
    fn bind_shift_alt_tab(
        &self,
        alt: impl Into<ButtonArg> + Clone,
        tab: impl Into<ButtonArg> + Clone,
    ) {
        bind_alt_tab_inner(self, alt, tab, seq!(with(LShift), Tab));
    }
}

impl<T: RegisterHotkey> Utils for T {}
