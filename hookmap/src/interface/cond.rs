use super::button::{ButtonWithState, ToButtonWithState};
use hookmap_core::ButtonState;
use std::{fmt::Debug, sync::Arc};

#[derive(Clone)]
enum _Cond {
    Pressed(ButtonWithState),
    Released(ButtonWithState),
    Callback(Arc<dyn Fn() -> bool + Send + Sync>),
}

/// A struct that represents the conditions under which hooks are enabled.
///
/// # Example
///
/// ```
/// use hookmap::*;
/// use std::sync::{Arc, atomic::{AtomicU32, Ordering}};
/// let hook = Hook::new();
/// let set = ButtonSet::new(&[Button::A, Button::B]);
/// let times = Arc::new(AtomicU32::new(0));
/// let times_ = Arc::clone(&times);
///
/// let conditional_hook = hook
///     .cond(&Cond::pressed(&Button::C))
///     .cond(&Cond::released(&set.all()))
///     .cond(&Cond::callback(move || {
///          times.load(Ordering::SeqCst) < 10
///     }));
///
/// // This hook is available when
/// //      C key is pressed and
/// //      A key and B key is released and
/// //      `times` < 10.
/// conditional_hook.bind(&Button::D).on_press(move |_| {
///     let times = times_.fetch_add(1, Ordering::SeqCst);
///     println!("Called {} times", times);
/// });
/// ```
///
#[derive(Clone)]
pub struct Cond(_Cond);

impl Cond {
    pub(crate) fn is_satisfied(&self) -> bool {
        match &self.0 {
            _Cond::Pressed(button) => button.is_pressed(),
            _Cond::Released(button) => button.is_released(),
            _Cond::Callback(callback) => callback(),
        }
    }

    /// Creates a new `Cond` that is conditional on the button being pressed.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::*;
    /// let hook = Hook::new();
    /// let cond = Cond::pressed(&Button::A);
    /// hook.cond(&cond)
    ///     .bind(&Button::B)
    ///     .on_press(|_| assert!(Button::A.is_pressed()));
    /// ```
    ///
    pub fn pressed(button: &impl ToButtonWithState) -> Self {
        Self(_Cond::Pressed(button.to_button_with_state()))
    }

    /// Creates a new `Cond` that is conditional on the button being released.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::*;
    /// let hook = Hook::new();
    /// let cond = Cond::released(&Button::A);
    /// hook.cond(&cond)
    ///     .bind(&Button::B)
    ///     .on_press(|_| assert!(!Button::A.is_pressed()));
    /// ```
    pub fn released(button: &impl ToButtonWithState) -> Self {
        Self(_Cond::Released(button.to_button_with_state()))
    }

    /// Creates a new `Cond` that is conditioned on the callback function.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::*;
    /// use std::sync::{Arc, atomic::{AtomicU32, Ordering}};
    /// let hook = Hook::new();
    /// let times = Arc::new(AtomicU32::new(0));
    /// let cond = {
    ///     let times = Arc::clone(&times);
    ///     Cond::callback(move || times.load(Ordering::SeqCst) < 10)
    /// };
    /// hook.cond(&cond)
    ///     .bind(&Button::A)
    ///     .on_press(move |_| {
    ///         assert!(times.fetch_add(1, Ordering::SeqCst) < 10);
    ///     });
    /// ```
    ///
    pub fn callback<F: 'static + Fn() -> bool + Send + Sync>(callback: F) -> Self {
        Self(_Cond::Callback(Arc::new(callback)))
    }
}

impl Debug for Cond {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}::", std::any::type_name::<Self>()))?;
        match &self.0 {
            _Cond::Pressed(button) => f.write_fmt(format_args!("Pressed({:?})", button)),
            _Cond::Released(button) => f.write_fmt(format_args!("Released({:?})", button)),
            _Cond::Callback(_) => f.write_str("callback"),
        }
    }
}

#[derive(Default, Clone)]
pub(crate) struct Conditions(Vec<Cond>);

impl Conditions {
    pub(crate) fn is_satisfied(&self) -> bool {
        self.0.iter().all(Cond::is_satisfied)
    }

    pub(crate) fn add(&mut self, cond: Cond) {
        self.0.push(cond);
    }
}

impl Debug for Conditions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.0.iter()).finish()
    }
}
