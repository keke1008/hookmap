use crate::button::{ButtonWithState, DownCastableButtonState};
use hookmap_core::ButtonState;
use std::{fmt::Debug, sync::Arc};

#[derive(Clone)]
pub enum Cond {
    Pressed(ButtonWithState),
    Released(ButtonWithState),
    Callback(Arc<dyn Fn() -> bool + Send + Sync>),
}

impl Cond {
    pub(crate) fn is_satisfied(&self) -> bool {
        match self {
            Cond::Pressed(button) => button.is_pressed(),
            Cond::Released(button) => !button.is_pressed(),
            Cond::Callback(callback) => callback(),
        }
    }

    pub fn pressed(button: impl DownCastableButtonState) -> Self {
        let button = Box::new(button).into_button_with_state();
        Self::Pressed(button)
    }

    pub fn released(button: impl DownCastableButtonState) -> Self {
        let button = Box::new(button).into_button_with_state();
        Self::Released(button)
    }

    pub fn cond<F: 'static + Fn() -> bool + Send + Sync>(callback: F) -> Self {
        Self::Callback(Arc::new(callback))
    }
}

impl Debug for Cond {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}::", std::any::type_name::<Self>()))?;
        match self {
            Cond::Pressed(button) => f.write_fmt(format_args!("Press({:?})", button)),
            Cond::Released(button) => f.write_fmt(format_args!("Release({:?})", button)),
            Cond::Callback(_) => f.write_str("callback"),
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
