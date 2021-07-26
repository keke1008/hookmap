pub mod event;
pub mod handler;
pub mod keyboard;
pub mod mouse;

/// Emulates button input.
/// This needs to be implemented for `Key` and `Mouse`.
pub trait EmulateButtonInput {
    /// Emulates a button press operation.
    fn press(&self);

    /// Emulates a button release operation.
    fn release(&self);

    /// Presses a button and releases it immediately.
    fn click(&self) {
        self.press();
        self.release();
    }

    /// Returns `true` if a button is pressed.
    fn is_pressed(&self) -> bool;

    /// Return `true` if a button is toggled on.
    fn is_toggled(&self) -> bool;
}

/// A button input action.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum ButtonAction {
    Press,
    Release,
}
