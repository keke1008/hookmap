pub struct Mouse;

/// Emulates the mouse wheel input.
pub trait EmulateMouseWheel {
    /// Rotates a mouse wheel.
    fn rotate(speed: i32);
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
