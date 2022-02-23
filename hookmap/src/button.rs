pub use hookmap_core::{Button, ButtonAction, ButtonEvent};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SequenceOperation {
    Click(Button),
    Press(Button),
    Release(Button),
}

impl SequenceOperation {
    fn operate(&self) {
        match self {
            SequenceOperation::Click(button) => button.click(),
            SequenceOperation::Press(button) => button.press(),
            SequenceOperation::Release(button) => button.release(),
        }
    }

    fn operate_recursive(&self) {
        match self {
            SequenceOperation::Click(button) => button.click_recursive(),
            SequenceOperation::Press(button) => button.press_recursive(),
            SequenceOperation::Release(button) => button.release_recursive(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sequence {
    with: Vec<Button>,
    seq: Vec<SequenceOperation>,
}

impl Sequence {
    const MODIFIER_LIST: [Button; 8] = [
        Button::LShift,
        Button::RShift,
        Button::LCtrl,
        Button::RCtrl,
        Button::LAlt,
        Button::RAlt,
        Button::LMeta,
        Button::RMeta,
    ];

    pub fn new(with: Vec<Button>, seq: Vec<SequenceOperation>) -> Self {
        Self { with, seq }
    }

    fn operate_with_keys(&self, operation: fn(&Button)) {
        self.with.iter().for_each(operation);
    }

    fn send_inner(
        &self,
        press: fn(&Button),
        release: fn(&Button),
        operation: fn(&SequenceOperation),
    ) {
        self.operate_with_keys(press);
        self.seq.iter().for_each(operation);
        self.operate_with_keys(release);
    }

    pub fn send(&self) {
        self.send_inner(
            ButtonInput::press,
            ButtonInput::release,
            SequenceOperation::operate,
        );
    }

    pub fn send_recursive(&self) {
        self.send_inner(
            ButtonInput::press_recursive,
            ButtonInput::release_recursive,
            SequenceOperation::operate_recursive,
        );
    }

    fn send_ignore_modifiers_inner(
        &self,
        press: fn(&Button),
        release: fn(&Button),
        operation: fn(&SequenceOperation),
    ) {
        let pressed_modifiers: Vec<_> = Self::MODIFIER_LIST
            .iter()
            .copied()
            .filter(Button::is_pressed)
            .collect();

        pressed_modifiers.iter().for_each(release);
        self.send_inner(press, release, operation);
        pressed_modifiers.iter().for_each(press);
    }

    pub fn send_ignore_modifiers(&self) {
        self.send_ignore_modifiers_inner(
            ButtonInput::press,
            ButtonInput::release,
            SequenceOperation::operate,
        );
    }

    pub fn send_ignore_modifiers_recursive(&self) {
        self.send_ignore_modifiers_inner(
            ButtonInput::press_recursive,
            ButtonInput::release_recursive,
            SequenceOperation::operate_recursive,
        );
    }
}
