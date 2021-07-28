use super::event::Event;

/// Installs a mouse hook in the way of each platform.
pub trait InstallMouseHook {
    /// Installs a mouse hook.
    fn install();
}

pub type MouseEvent = Event<Mouse>;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Mouse {
    LButton,
    RButton,
    MButton,
    SideButton1,
    SideButton2,
}

pub struct MouseWheel;
pub struct MouseCursor;

/// Emulates the mouse wheel input.
pub trait EmulateMouseWheel {
    /// Rotates a mouse wheel.
    fn rotate(speed: u32);
}

/// Emulates the mouse cursor input.
pub trait EmulateMouseCursor {
    /// Moves a mouse cursor absolutely.
    fn move_abs(x: i32, y: i32);

    /// Moves a mouse cursor relatively.
    fn move_rel(dx: i32, dy: i32);

    /// Returns a position of a mouse cursor.
    fn get_pos() -> (i32, i32);
}
