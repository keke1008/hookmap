use super::button::{ButtonWithState, DownCastableButtonState};
use hookmap_core::ButtonState;
use std::{fmt::Debug, sync::Arc};

#[derive(Clone)]
enum _Cond {
    Pressed(ButtonWithState),
    Released(ButtonWithState),
    Callback(Arc<dyn Fn() -> bool + Send + Sync>),
}

#[derive(Clone)]
pub struct Cond(_Cond);

impl Cond {
    pub(crate) fn is_satisfied(&self) -> bool {
        match &self.0 {
            _Cond::Pressed(button) => button.is_pressed(),
            _Cond::Released(button) => !button.is_pressed(),
            _Cond::Callback(callback) => callback(),
        }
    }

    pub fn pressed(button: impl DownCastableButtonState) -> Self {
        let button = Box::new(button).into_button_with_state();
        Self(_Cond::Pressed(button))
    }

    pub fn released(button: impl DownCastableButtonState) -> Self {
        let button = Box::new(button).into_button_with_state();
        Self(_Cond::Released(button))
    }

    pub fn callback<F: 'static + Fn() -> bool + Send + Sync>(callback: F) -> Self {
        Self(_Cond::Callback(Arc::new(callback)))
    }
}

impl Debug for Cond {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}::", std::any::type_name::<Self>()))?;
        match &self.0 {
            _Cond::Pressed(button) => f.write_fmt(format_args!("Press({:?})", button)),
            _Cond::Released(button) => f.write_fmt(format_args!("Release({:?})", button)),
            _Cond::Callback(_) => f.write_str("callback"),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub(crate) struct Conditions(Vec<Cond>);

impl Conditions {
    pub(crate) fn is_satisfied(&self) -> bool {
        self.0.iter().all(Cond::is_satisfied)
    }

    pub(crate) fn add(&mut self, cond: Cond) {
        self.0.push(cond);
    }
}
