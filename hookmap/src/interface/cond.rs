use crate::button::{ButtonSet, ButtonState, ToButtonSet};
use std::borrow::Borrow;
use std::{fmt::Debug, sync::Arc};

/// An enum that represents the conditions under which hooks are enabled.
///
/// # Example
///
/// ```
/// use hookmap::*;
/// use std::sync::{Arc, atomic::{AtomicU32, Ordering}};
/// let hook = Hook::new();
/// let a_and_b = all!(A, B);
/// let times = Arc::new(AtomicU32::new(0));
/// let times_ = Arc::clone(&times);
///
/// let conditional_hook = hook
///     .cond(Cond::pressed(Button::C))
///     .cond(Cond::released(a_and_b))
///     .cond(Cond::callback(move || {
///          times.load(Ordering::SeqCst) < 10
///     }));
///
/// // This hook is available when
/// //      C key is pressed and
/// //      A key and B key is released and
/// //      `times` < 10.
/// conditional_hook.bind(Button::D).on_press(move |_| {
///     let times = times_.fetch_add(1, Ordering::SeqCst);
///     println!("Called {} times", times);
/// });
/// ```
///
#[derive(Clone)]
pub enum Cond {
    Pressed(ButtonSet),
    Released(ButtonSet),
    Callback(Arc<dyn Fn() -> bool + Send + Sync>),
}

impl Cond {
    /// Creates a new `Cond` that is conditional on the button being pressed.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::*;
    /// let hook = Hook::new();
    /// let cond = Cond::pressed(Button::A);
    /// hook.cond(cond)
    ///     .bind(Button::B)
    ///     .on_press(|_| assert!(Button::A.is_pressed()));
    /// ```
    ///
    pub fn pressed<B: Borrow<B> + ToButtonSet>(button: B) -> Self {
        Self::Pressed(button.to_button_set())
    }

    /// Creates a new `Cond` that is conditional on the button being released.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::*;
    /// let hook = Hook::new();
    /// let cond = Cond::released(Button::A);
    /// hook.cond(cond)
    ///     .bind(Button::B)
    ///     .on_press(|_| assert!(!Button::A.is_pressed()));
    /// ```
    pub fn released<B: Borrow<B> + ToButtonSet>(button: B) -> Self {
        Self::Released(button.to_button_set())
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
    /// hook.cond(cond)
    ///     .bind(Button::A)
    ///     .on_press(move |_| {
    ///         assert!(times.fetch_add(1, Ordering::SeqCst) < 10);
    ///     });
    /// ```
    ///
    pub fn callback<F: 'static + Fn() -> bool + Send + Sync>(callback: F) -> Self {
        Self::Callback(Arc::new(callback))
    }
}

impl Debug for Cond {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}::", std::any::type_name::<Self>()))?;
        match self {
            Cond::Pressed(button) => f.write_fmt(format_args!("Pressed({:?})", button)),
            Cond::Released(button) => f.write_fmt(format_args!("Released({:?})", button)),
            Cond::Callback(_) => f.write_str("callback"),
        }
    }
}

#[derive(Default, Clone)]
pub(crate) struct Conditions {
    pressed: Vec<ButtonSet>,
    released: Vec<ButtonSet>,
    callback: Vec<Arc<dyn Fn() -> bool + Send + Sync>>,
}

impl Conditions {
    pub(crate) fn is_satisfied(&self) -> bool {
        self.pressed.iter().all(ButtonSet::is_pressed)
            && self.released.iter().all(ButtonSet::is_released)
            && self.callback.iter().all(|callback| (callback)())
    }

    pub(crate) fn add(&mut self, cond: Cond) {
        match cond {
            Cond::Pressed(button) => self.pressed.push(button),
            Cond::Released(button) => self.released.push(button),
            Cond::Callback(callback) => self.callback.push(callback),
        }
    }
}

impl Debug for Conditions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Conditions")
            .field("pressed", &self.pressed)
            .field("released", &self.released)
            .field("callback", &format!("len: {}", self.callback.len()))
            .finish()
    }
}
