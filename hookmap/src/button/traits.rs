use super::ButtonSet;
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

impl ButtonInput for ButtonSet {
    fn press(&self) {
        match self {
            ButtonSet::All(buttons) => buttons.iter().for_each(Button::press),
            ButtonSet::Any(buttons) => {
                buttons.iter().next().map(Button::press);
            }
            ButtonSet::Single(button) => button.press(),
        }
    }

    fn release(&self) {
        match self {
            ButtonSet::All(buttons) => buttons.iter().for_each(Button::release),
            ButtonSet::Any(buttons) => {
                buttons.iter().next().map(Button::release);
            }
            ButtonSet::Single(button) => button.release(),
        }
    }

    fn press_recursive(&self) {
        match self {
            ButtonSet::All(buttons) => buttons.iter().for_each(Button::press_recursive),
            ButtonSet::Any(buttons) => {
                buttons.iter().next().map(Button::press_recursive);
            }
            ButtonSet::Single(button) => button.press_recursive(),
        }
    }

    fn release_recursive(&self) {
        match self {
            ButtonSet::All(buttons) => buttons.iter().for_each(Button::release_recursive),
            ButtonSet::Any(buttons) => {
                buttons.iter().next().map(Button::release_recursive);
            }
            ButtonSet::Single(button) => button.release_recursive(),
        }
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
