use crate::{
    button::{EmulateButtonInput, EmulateButtonState},
    *,
};
use hookmap_core::Button;
use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicBool, Ordering};

static IS_ALT_TAB_WORKING: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));

fn alt_tab<T, S, I>(hook: &T, alt: &S, tab: &S, tab_like: &I)
where
    T: SelectHandleTarget,
    S: EmulateButtonState,
    I: EmulateButtonInput,
{
    hook.bind(alt).disable();
    hook.bind(alt).block().on_release(move |_| {
        IS_ALT_TAB_WORKING.store(false, Ordering::SeqCst);
        Button::Alt.release();
    });

    let modifier_alt = hook.cond(Cond::pressed(alt));
    modifier_alt.bind(tab).block().on_press(move |_| {
        if !IS_ALT_TAB_WORKING.swap(true, Ordering::SeqCst) {
            Button::Alt.press();
        }
    });
    modifier_alt.bind(tab).like(tab_like);
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
    fn bind_alt_tab<B: EmulateButtonState>(&self, alt: &B, tab: &B) {
        alt_tab(self, alt, tab, &Button::Tab);
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
    fn bind_shift_alt_tab<B: EmulateButtonState>(&self, alt: &B, tab: &B) {
        let shift_tab = ButtonSet::new([Button::Tab, Button::Shift]);
        alt_tab(self, alt, tab, &shift_tab.all());
    }
}

impl<T: SelectHandleTarget> Utils for T {}