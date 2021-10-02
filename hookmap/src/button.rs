use hookmap_core::{Button, ButtonOperation};
use std::iter;

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
pub enum ButtonSet {
    Single(Button),
    Any(Vec<Button>),
    All(Vec<Button>),
}

impl ButtonSet {
    pub(crate) fn iter(&self) -> Box<dyn Iterator<Item = &Button> + '_> {
        match self {
            ButtonSet::Single(ref button) => Box::new(iter::once(button)),
            ButtonSet::Any(ref buttons) => Box::new(buttons.iter()),
            ButtonSet::All(ref buttons) => Box::new(buttons.iter()),
        }
    }
}

impl From<Button> for ButtonSet {
    fn from(button: Button) -> Self {
        Self::Single(button)
    }
}

impl<T: Into<ButtonSet> + Clone> From<&T> for ButtonSet {
    fn from(button: &T) -> Self {
        <T as Into<ButtonSet>>::into(button.clone())
    }
}

impl<const N: usize> From<ConstantAny<N>> for ButtonSet {
    fn from(any: ConstantAny<N>) -> Self {
        ButtonSet::Any(any.0.into())
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

impl ButtonInput for ButtonSet {
    fn press(&self) {
        match self {
            ButtonSet::All(buttons) => buttons.iter().for_each(Button::press),
            ButtonSet::Any(buttons) => {
                if let Some(button) = buttons.iter().next() {
                    button.press();
                }
            }
            ButtonSet::Single(button) => button.press(),
        }
    }

    fn release(&self) {
        match self {
            ButtonSet::All(buttons) => buttons.iter().for_each(Button::release),
            ButtonSet::Any(buttons) => {
                if let Some(button) = buttons.iter().next() {
                    button.release();
                }
            }
            ButtonSet::Single(button) => button.release(),
        }
    }

    fn press_recursive(&self) {
        match self {
            ButtonSet::All(buttons) => buttons.iter().for_each(Button::press_recursive),
            ButtonSet::Any(buttons) => {
                if let Some(button) = buttons.iter().next() {
                    button.press_recursive();
                }
            }
            ButtonSet::Single(button) => button.press_recursive(),
        }
    }

    fn release_recursive(&self) {
        match self {
            ButtonSet::All(buttons) => buttons.iter().for_each(Button::release_recursive),
            ButtonSet::Any(buttons) => {
                if let Some(button) = buttons.iter().next() {
                    button.release_recursive();
                }
            }
            ButtonSet::Single(button) => button.release_recursive(),
        }
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

impl ButtonState for ButtonSet {
    fn is_pressed(&self) -> bool {
        match self {
            ButtonSet::All(buttons) => buttons.iter().all(Button::is_pressed),
            ButtonSet::Any(buttons) => buttons.iter().any(Button::is_pressed),
            ButtonSet::Single(button) => button.is_pressed(),
        }
    }

    fn is_released(&self) -> bool {
        match self {
            ButtonSet::All(buttons) => buttons.iter().all(Button::is_released),
            ButtonSet::Any(buttons) => buttons.iter().any(Button::is_released),
            ButtonSet::Single(button) => button.is_released(),
        }
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
