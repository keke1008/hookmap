use super::event::Event;

/// Installs a mouse hook in the way of each platform.
/// This needs to implement for `InputHandler`.
pub trait InstallMouseHook {
    /// Installs a mouse hook.
    fn install();
}

/// Emulates mouse input.
/// This needs to implement for `MouseInput`.
pub trait EmulateMouseInput {
    /// Emulates a action of pressing a mouse button.
    fn press(&self);

    /// Emulates a action of releasing a mouse button.
    fn release(&self);

    /// Presses a mouse button and releases it immediately.
    fn click(&self) {
        self.press();
        self.release();
    }

    /// Returns `true` if a mouse button is pressed.
    fn is_pressed(&self) -> bool;

    /// Returns a position of a mouse cursor.
    fn get_cursor_pos() -> (i32, i32);

    /// Moves a mouse cursor absolutely.
    fn move_abs(x: i32, y: i32);

    /// Moves a mouse cursor relatively.
    fn move_rel(dx: i32, dy: i32);

    /// Rotates a mouse wheel.
    fn rotate_wheel(speed: u32);
}

pub type MouseEvent = Event<MouseInput, MouseAction>;

/// A Mouse input action.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum MouseAction {
    Press,
    Release,
    Move((i32, i32)),
    Wheel(i32),
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum MouseInput {
    LButton,
    RButton,
    MButton,
    SideButton1,
    SideButton2,
    Wheel,
    Move,
}
