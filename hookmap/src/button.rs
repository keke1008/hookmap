pub use hookmap_core::{Button, ButtonAction, ButtonEvent};

use crate::macros::{ButtonArg, ExpandButtonArg};
use hookmap_core::ButtonOperation;

#[derive(Debug, Clone)]
pub struct ConstantAny<const N: usize>([Button; N]);

/// Shift key that does not distinguish between right and left.
pub const SHIFT: ConstantAny<2> = ConstantAny([Button::LShift, Button::RShift]);

/// Control key that does not distinguish between right and left.
pub const CTRL: ConstantAny<2> = ConstantAny([Button::LCtrl, Button::RCtrl]);

/// Alt key that does not distinguish between right and left.
pub const ALT: ConstantAny<2> = ConstantAny([Button::LAlt, Button::RAlt]);

/// Meta key that does not distinguish between right and left.
pub const META: ConstantAny<2> = ConstantAny([Button::LMeta, Button::RMeta]);

impl<const N: usize> ExpandButtonArg for ConstantAny<N> {
    fn expand(self) -> Box<dyn Iterator<Item = ButtonArg>> {
        Box::new(IntoIterator::into_iter(self.0).map(ButtonArg::direct))
    }
}

/// Emulates button input.
pub trait ButtonInput {
    /// Emulates button press operation.
    fn press(&self);

    /// Emulates button release operation.
    fn release(&self);

    /// Presses button and releases it immediately.
    fn click(&self) {
        self.press();
        self.release();
    }

    /// Emulates button press operation.
    /// This differs from [`ButtonInput::press`] in that it can call hook handlers.
    fn press_recursive(&self);

    /// Emulates button release operation.
    /// This differs from [`ButtonInput::release`] in that it can call hook handlers.
    fn release_recursive(&self);

    /// Calls [`ButtonInput::press_recursive`] and [`ButtonInput::release_recursive`].
    fn click_recursive(&self) {
        self.press_recursive();
        self.release_recursive();
    }
}

impl<T: ButtonInput> ButtonInput for &T {
    fn press(&self) {
        (**self).press()
    }

    fn release(&self) {
        (**self).release()
    }

    fn press_recursive(&self) {
        (**self).press_recursive()
    }

    fn release_recursive(&self) {
        (**self).release_recursive()
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

impl<const N: usize> ButtonInput for ConstantAny<N> {
    fn press(&self) {
        if let Some(button) = self.0.get(0) {
            button.press();
        }
    }

    fn release(&self) {
        if let Some(button) = self.0.get(0) {
            button.release();
        }
    }

    fn press_recursive(&self) {
        if let Some(button) = self.0.get(0) {
            button.press_recursive();
        }
    }

    fn release_recursive(&self) {
        if let Some(button) = self.0.get(0) {
            button.release_recursive();
        }
    }
}

// Get the status of a button.
pub trait ButtonState {
    /// Returns `true` if the button is pressed.
    fn is_pressed(&self) -> bool;

    /// Returns `true` if the button is released.
    fn is_released(&self) -> bool {
        !self.is_pressed()
    }
}

impl<T: ButtonState> ButtonState for &T {
    fn is_pressed(&self) -> bool {
        (**self).is_pressed()
    }

    fn is_released(&self) -> bool {
        (**self).is_released()
    }
}

impl ButtonState for Button {
    fn is_pressed(&self) -> bool {
        self.read_is_pressed()
    }
}

impl<const N: usize> ButtonState for ConstantAny<N> {
    fn is_pressed(&self) -> bool {
        self.0.iter().any(Button::is_pressed)
    }

    fn is_released(&self) -> bool {
        self.0.iter().any(Button::is_released)
    }
}
