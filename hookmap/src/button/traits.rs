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
