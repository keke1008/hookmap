use crate::{button::ButtonSet, *};
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
                seq!(LAlt up);
            };

            modifier([alt]) {
                remap [tab] => Tab;
                on_press Tab => move |_| {
                    if !IS_ALT_TAB_WORKING.swap(true, Ordering::SeqCst) {
                        seq!(LAlt down, Tab);
                    }
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
                seq!(LAlt up, LShift up);
            };

            modifier([alt]) {
                remap [tab] => Tab;
                on_press Tab => move |_| {
                    if !IS_ALT_TAB_WORKING.swap(true, Ordering::SeqCst) {
                        seq!(LAlt down, LShift down, Tab);
                    }
                };
            }
        });
    }
}

impl<T: SelectHandleTarget> Utils for T {}
