use super::event::Event;

pub trait InstallMouseHook {
    fn install();
}

pub trait EmulateMouseInput {
    fn press(&self);
    fn release(&self);
    fn click(&self) {
        self.press();
        self.release();
    }
    fn is_pressed(&self) -> bool;
    fn get_cursor_pos() -> (i32, i32);
    fn move_abs(x: i32, y: i32);
    fn move_rel(dx: i32, dy: i32);
    fn rotate_wheel(speed: u32);
}

pub type MouseEvent = Event<MouseInput, MouseAction>;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum MouseAction {
    Press,
    Release,
    Move(i32, i32),
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
