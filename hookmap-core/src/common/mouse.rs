use super::event::EventDetail;
use super::handler::HookManager;
use once_cell::sync::Lazy;

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

pub type MouseEvent = EventDetail<MouseInput, MouseAction>;
pub type MouseHook = HookManager<MouseInput, MouseAction>;

pub static MOUSE_HOOK: Lazy<MouseHook> = Lazy::new(MouseHook::default);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
