use crate::*;
use std::sync::atomic::{AtomicBool, Ordering};

static IS_ALT_TAB_WORKING: AtomicBool = AtomicBool::new(false);

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
    /// let hotkey = Hotkey::new();
    /// hotkey.bind_alt_tab(Button::A, Button::T);
    /// ```
    // fn bind_alt_tab<B: EmulateButtonState>(&self, alt: &B, tab: &B) {
    //     alt_tab(self, alt, tab, &Button::Tab);
    // }
    fn bind_alt_tab<T>(&self, alt: T, tab: Button)
    where
        T: Clone,
        ButtonSet: From<T>,
    {
        hotkey!(self => {
            on_release [&alt] => move |_| {
                IS_ALT_TAB_WORKING.store(false, Ordering::SeqCst);
                Button::LAlt.release();
            };

            modifier([alt]) {
                on_press [tab] => move |_| {
                    if !IS_ALT_TAB_WORKING.swap(true, Ordering::SeqCst) {
                        Button::LAlt.press();
                    }
                };
                bind [tab] => Tab;
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
    /// use hookmap::*;
    /// let hotkey = Hotkey::new();
    /// hotkey.bind_shift_alt_tab(Button::A, Button::R);
    /// ```
    fn bind_shift_alt_tab<T>(&self, alt: T, tab: Button)
    where
        T: Clone,
        ButtonSet: From<T>,
    {
        hotkey!(self => {
            on_release [&alt] => move |_| {
                IS_ALT_TAB_WORKING.store(false, Ordering::SeqCst);
                Button::LAlt.release();
            };

            modifier([alt]) {
                on_press [tab] => move |_| {
                    if !IS_ALT_TAB_WORKING.swap(true, Ordering::SeqCst) {
                        Button::LAlt.press();
                    }
                };
                bind [tab] => [all!(LShift, Tab)];
            }
        });
    }
}

impl<T: SelectHandleTarget> Utils for T {}
