use crate::{button::EmulateButtonInput, *};
use button::ToButtonSet;
use hookmap_core::Button;
use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicBool, Ordering};

static IS_ALT_TAB_WORKING: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));

fn alt_tab<T, U, V>(hook: &T, alt: U, tab: U, tab_like: V)
where
    T: SelectHandleTarget,
    U: ToButtonSet + Clone,
    V: EmulateButtonInput,
{
    hotkey!(hook => {
        disable [&alt];
        on_release [&alt] => move |_| {
            IS_ALT_TAB_WORKING.store(false, Ordering::SeqCst);
            Button::LAlt.release();
        };

        if (pressed [alt]) {
            on_press [&tab] => move |_| {
                if !IS_ALT_TAB_WORKING.swap(true, Ordering::SeqCst) {
                    Button::LAlt.press();
                }
            };
            bind [&tab] => [tab_like];
        }
    });
}

/// Utility function.
pub trait Utils: SelectHandleTarget + Sized {
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
    /// use hookmap::*;
    /// let hook = Hook::new();
    /// hook.bind_alt_tab(&Button::A, &Button::T);
    /// ```
    // fn bind_alt_tab<B: EmulateButtonState>(&self, alt: &B, tab: &B) {
    //     alt_tab(self, alt, tab, &Button::Tab);
    // }
    fn bind_alt_tab<T>(&self, alt: T, tab: T)
    where
        T: ToButtonSet + Clone,
    {
        alt_tab(self, alt, tab, Button::Tab);
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
    /// use hookmap::*;
    /// let hook = Hook::new();
    /// hook.bind_shift_alt_tab(&Button::A, &Button::R);
    /// ```
    fn bind_shift_alt_tab<T>(&self, alt: T, tab: T)
    where
        T: ToButtonSet + Clone,
    {
        let shift_tab = all!(Tab, LShift);
        alt_tab(self, alt, tab, shift_tab);
    }
}

impl<T: SelectHandleTarget> Utils for T {}
