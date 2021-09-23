use crate::button::{ButtonSet, ButtonState, ToButtonSet};
use std::borrow::Borrow;
use std::sync::atomic::AtomicBool;
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
    Modifier(ButtonSet),
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

    /// Creates a new `Cond` that is conditional on the button of the modifier.
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
    pub fn modifier<B: Borrow<B> + ToButtonSet>(button: B) -> Self {
        Self::Modifier(button.to_button_set())
    }
}

impl Debug for Cond {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}::", std::any::type_name::<Self>()))?;
        match self {
            Cond::Pressed(button) => f.write_fmt(format_args!("Pressed({:?})", button)),
            Cond::Released(button) => f.write_fmt(format_args!("Released({:?})", button)),
            Cond::Callback(_) => f.write_str("callback"),
            Cond::Modifier(button) => f.write_fmt(format_args!("Modifier({:?})", button)),
        }
    }
}

#[derive(Default)]
pub(crate) struct Conditions {
    pub(crate) pressed: Vec<ButtonSet>,
    pub(crate) released: Vec<ButtonSet>,
    pub(crate) callback: Vec<Arc<dyn Fn() -> bool + Send + Sync>>,
    pub(crate) modifier: (Vec<ButtonSet>, Arc<AtomicBool>),
}

impl Conditions {
    pub(crate) fn is_satisfied(&self) -> bool {
        self.pressed.iter().all(ButtonSet::is_pressed)
            && self.released.iter().all(ButtonSet::is_released)
            && self.callback.iter().all(|callback| (callback)())
    }

    fn clone(&self) -> Self {
        Self {
            pressed: self.pressed.clone(),
            released: self.released.clone(),
            callback: self.callback.clone(),
            modifier: (self.modifier.0.clone(), Arc::clone(&self.modifier.1)),
        }
    }

    fn clone_expect_modifier(&self) -> Self {
        Self {
            modifier: (self.modifier.0.clone(), Arc::default()),
            ..self.clone()
        }
    }

    pub(crate) fn add(&self, cond: Cond) -> Self {
        let mut this = if let Cond::Modifier(_) = cond {
            self.clone_expect_modifier()
        } else {
            self.clone()
        };
        match cond {
            Cond::Pressed(button) => this.pressed.push(button),
            Cond::Released(button) => this.released.push(button),
            Cond::Callback(callback) => this.callback.push(callback),
            Cond::Modifier(button) => this.modifier.0.push(button),
        }
        this
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
