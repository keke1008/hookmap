use super::{All, Any};
use crate::interface::button::ButtonWithState;
use hookmap_core::{Button, ButtonOperation};

pub trait ButtonInput {
    /// Emulates a button press operation.
    fn press(&self);

    /// Emulates a button release operation.
    fn release(&self);

    /// Presses a button and releases it immediately.
    fn click(&self) {
        self.press();
        self.release();
    }

    /// Emulates a button press operation.
    /// This differs from [`ButtonInput::press`] in that it can call hook handlers.
    fn press_recursive(&self);

    /// Emulates a button release operation.
    /// This differs from [`ButtonInput::release`] in that it can call hook handlers.
    fn release_recursive(&self);

    /// Calls [`ButtonInput::press_recursive`] and [`ButtonInput::release_recursive`].
    fn click_recursive(&self) {
        self.press_recursive();
        self.release_recursive();
    }
}

impl ButtonInput for Button {
    fn press(&self) {
        self.generate_press_event(false);
    }

    fn release(&self) {
        self.generate_release_event(false);
    }

    fn press_recursive(&self) {
        self.generate_press_event(true);
    }

    fn release_recursive(&self) {
        self.generate_release_event(true);
    }
}

impl ButtonInput for All {
    fn press(&self) {
        self.0.iter().for_each(Button::press);
    }

    fn release(&self) {
        self.0.iter().for_each(Button::release);
    }

    fn press_recursive(&self) {
        self.0.iter().for_each(Button::press_recursive);
    }

    fn release_recursive(&self) {
        self.0.iter().for_each(Button::release_recursive);
    }
}

pub trait ButtonState {
    /// Returns `true` if the button is pressed.
    fn is_pressed(&self) -> bool;

    /// Returns `true` if the button is released.
    fn is_released(&self) -> bool {
        !self.is_pressed()
    }
}

impl ButtonState for Button {
    fn is_pressed(&self) -> bool {
        self.read_is_pressed()
    }
}

impl ButtonState for Any {
    fn is_pressed(&self) -> bool {
        self.0.iter().any(Button::is_pressed)
    }

    fn is_released(&self) -> bool {
        self.0.iter().any(Button::is_released)
    }
}

impl ButtonState for All {
    fn is_pressed(&self) -> bool {
        self.0.iter().all(Button::is_pressed)
    }

    fn is_released(&self) -> bool {
        self.0.iter().all(Button::is_released)
    }
}

/// Convert to [`ButtonWithState`]
pub trait ToButtonWithState: Send + Sync {
    fn to_button_with_state(&self) -> ButtonWithState;
}

impl<T: ToButtonWithState> ToButtonWithState for &T {
    fn to_button_with_state(&self) -> ButtonWithState {
        (*self).to_button_with_state()
    }
}

impl ToButtonWithState for Button {
    fn to_button_with_state(&self) -> ButtonWithState {
        ButtonWithState::Button(*self)
    }
}

impl ToButtonWithState for Any {
    fn to_button_with_state(&self) -> ButtonWithState {
        ButtonWithState::Any(self.clone())
    }
}

impl ToButtonWithState for All {
    fn to_button_with_state(&self) -> ButtonWithState {
        ButtonWithState::All(self.clone())
    }
}

pub trait EmulateButtonInput: ButtonInput + Send + Sync + Clone + 'static {}
pub trait EmulateButtonState: ToButtonWithState + Clone + 'static {}

impl<T: ButtonInput + Send + Sync + Clone + 'static> EmulateButtonInput for T {}
impl<T: ToButtonWithState + Clone + 'static> EmulateButtonState for T {}
