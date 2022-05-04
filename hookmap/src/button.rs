pub use hookmap_core::button::{Button, ButtonAction};
pub use hookmap_core::event::ButtonEvent;

/// Emulates button input.
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
        Button::LSuper,
        Button::RSuper,
    ];

    pub fn new(with: Vec<Button>, seq: Vec<SequenceOperation>) -> Self {
        Self { with, seq }
    }

    fn operate_with_keys(&self, operation: fn(Button)) {
        self.with.iter().copied().for_each(operation);
    }

    fn send_inner(
        &self,
        press: fn(Button),
        release: fn(Button),
        operation: fn(&SequenceOperation),
    ) {
        self.operate_with_keys(press);
        self.seq.iter().for_each(operation);
        self.operate_with_keys(release);
    }

    pub fn send(&self) {
        self.send_inner(Button::press, Button::release, SequenceOperation::operate);
    }

    pub fn send_recursive(&self) {
        self.send_inner(
            Button::press_recursive,
            Button::release_recursive,
            SequenceOperation::operate_recursive,
        );
    }

    fn send_ignore_modifiers_inner(
        &self,
        press: fn(Button),
        release: fn(Button),
        operation: fn(&SequenceOperation),
    ) {
        let pressed_modifiers: Vec<_> = Self::MODIFIER_LIST
            .iter()
            .copied()
            .filter(|button| button.is_pressed())
            .collect();

        pressed_modifiers.iter().copied().for_each(release);
        self.send_inner(press, release, operation);
        pressed_modifiers.iter().copied().for_each(press);
    }

    pub fn send_ignore_modifiers(&self) {
        self.send_ignore_modifiers_inner(
            Button::press,
            Button::release,
            SequenceOperation::operate,
        );
    }

    pub fn send_ignore_modifiers_recursive(&self) {
        self.send_ignore_modifiers_inner(
            Button::press_recursive,
            Button::release_recursive,
            SequenceOperation::operate_recursive,
        );
    }
}
