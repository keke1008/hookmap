use hookmap_core::{Button, ButtonOperation};
use std::iter::{self, FromIterator};

#[derive(Debug, Clone)]
pub struct ConstantAny<const N: usize>([Button; N]);

/// Shift key that does not distinguish between right and left.
pub static SHIFT: ConstantAny<2> = ConstantAny([Button::LShift, Button::RShift]);

/// Control key that does not distinguish between right and left.
pub static CTRL: ConstantAny<2> = ConstantAny([Button::LCtrl, Button::RCtrl]);

/// Alt key that does not distinguish between right and left.
pub static ALT: ConstantAny<2> = ConstantAny([Button::LAlt, Button::RAlt]);

/// Meta key that does not distinguish between right and left.
pub static META: ConstantAny<2> = ConstantAny([Button::LMeta, Button::RMeta]);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ButtonSet(Vec<Button>);

impl ButtonSet {
    pub(crate) fn iter<'a>(&'a mut self) -> impl Iterator<Item = Button> + 'a {
        self.0.iter().copied()
    }
}

impl<I: Iterator<Item = Button>> FromIterator<I> for ButtonSet {
    fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
        ButtonSet(Vec::from_iter(iter.into_iter().flatten()))
    }
}

pub(crate) trait ExpandButton {
    fn expand<'a>(&'a self) -> Box<dyn Iterator<Item = Button> + 'a>;
}

impl ExpandButton for Button {
    fn expand(&self) -> Box<dyn Iterator<Item = Button>> {
        Box::new(iter::once(*self)) as Box<dyn Iterator<Item = Button>>
    }
}

impl ExpandButton for ButtonSet {
    fn expand<'a>(&'a self) -> Box<dyn Iterator<Item = Button> + 'a> {
        Box::new(self.0.iter().copied())
    }
}

impl<const N: usize> ExpandButton for ConstantAny<N> {
    fn expand<'a>(&'a self) -> Box<dyn Iterator<Item = Button> + 'a> {
        Box::new(self.0.iter().copied())
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
