use crate::{button::EmulateButtonInput, *};
use hookmap_core::Button;
use interface::ToButtonWithState;
use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicBool, Ordering};

static IS_ALT_TAB_WORKING: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));

fn alt_tab<T, U, V>(hook: &T, alt: U, tab: U, tab_like: V)
where
    T: SelectHandleTarget,
    U: ToButtonWithState + Clone,
    V: EmulateButtonInput,
{
    hook.bind(&alt).disable();
    hook.bind(&alt).block().on_release(move |_| {
        IS_ALT_TAB_WORKING.store(false, Ordering::SeqCst);
        Button::Alt.release();
    });

    let modifier_alt = hook.cond(Cond::pressed(alt));
    modifier_alt.bind(&tab).block().on_press(move |_| {
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
    // fn bind_alt_tab<B: EmulateButtonState>(&self, alt: &B, tab: &B) {
    //     alt_tab(self, alt, tab, &Button::Tab);
    // }
    fn bind_alt_tab<T>(&self, alt: T, tab: T)
    where
        T: ToButtonWithState + Clone,
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
        T: ToButtonWithState + Clone,
    {
        let shift_tab = ButtonSet::new([Button::Tab, Button::Shift]);
        alt_tab(self, alt, tab, shift_tab.all());
    }
}

impl<T: SelectHandleTarget> Utils for T {}
