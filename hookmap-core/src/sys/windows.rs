mod hook;
mod input;
mod vkcode;

use hook::Hook;
use input::Input;

use crate::button::{Button, ButtonAction};
use crate::event::{self, EventReceiver};

use std::sync::atomic::{AtomicBool, Ordering};

use once_cell::sync::Lazy;
use windows::Win32::UI::HiDpi;

const SHOULD_BE_IGNORED_FLAG: usize = 0x1;
const INJECTED_FLAG: usize = 0x2;

#[derive(Debug)]
struct ButtonState([AtomicBool; Button::VARIANT_COUNT]);

impl ButtonState {
    const fn new() -> Self {
        let inner = unsafe {
            // AtomicBool has the same in-memory representation as a bool.
            // https://doc.rust-lang.org/std/sync/atomic/struct.AtomicBool.html
            std::mem::transmute([false; Button::VARIANT_COUNT])
        };
        ButtonState(inner)
    }

    #[inline]
    fn press(&self, button: Button, order: Ordering) {
        self.0[button as usize].store(true, order);
    }

    #[inline]
    fn release(&self, button: Button, order: Ordering) {
        self.0[button as usize].store(false, order)
    }

    #[inline]
    fn is_pressed(&self, button: Button, order: Ordering) -> bool {
        self.0[button as usize].load(order)
    }

    #[inline]
    fn is_released(&self, button: Button, order: Ordering) -> bool {
        !self.0[button as usize].load(order)
    }
}

static BUTTON_STATE: ButtonState = ButtonState::new();

static INPUT: Lazy<Input> = Lazy::new(Input::new);

#[inline]
fn send_input(button: Button, action: ButtonAction, recursive: bool, assume: fn(Button)) {
    let left_and_right_modifier = match button {
        Button::Shift => Some((Button::LShift, Button::RShift)),
        Button::Ctrl => Some((Button::LCtrl, Button::RCtrl)),
        Button::Alt => Some((Button::LAlt, Button::RAlt)),
        Button::Super => Some((Button::LSuper, Button::RSuper)),
        _ => None,
    };
    if let Some((left, right)) = left_and_right_modifier {
        assume(left);
        assume(right);
        assume(button);
        INPUT.button_input(left, action, recursive);
        INPUT.button_input(right, action, recursive);
    } else {
        assume(button);
        INPUT.button_input(button, action, recursive);
    }
}

impl Button {
    #[inline]
    pub fn press(self) {
        send_input(self, ButtonAction::Press, false, Button::assume_pressed);
    }

    #[inline]
    pub fn press_recursive(self) {
        send_input(self, ButtonAction::Press, true, Button::assume_pressed);
    }

    #[inline]
    pub fn release(self) {
        send_input(self, ButtonAction::Release, false, Button::assume_released);
    }

    #[inline]
    pub fn release_recursive(self) {
        send_input(self, ButtonAction::Release, true, Button::assume_released);
    }

    #[inline]
    pub fn click(self) {
        self.press();
        self.release();
    }

    #[inline]
    pub fn click_recursive(self) {
        self.press_recursive();
        self.release_recursive();
    }

    #[inline]
    pub fn is_pressed(self) -> bool {
        BUTTON_STATE.is_pressed(self, Ordering::SeqCst)
    }

    #[inline]
    pub fn is_released(self) -> bool {
        BUTTON_STATE.is_released(self, Ordering::SeqCst)
    }

    #[inline]
    fn assume_pressed(self) {
        BUTTON_STATE.press(self, Ordering::SeqCst);
    }

    #[inline]
    fn assume_released(self) {
        BUTTON_STATE.release(self, Ordering::SeqCst);
    }
}

pub mod mouse {
    use super::INPUT;

    #[inline]
    pub fn get_position() -> (i32, i32) {
        INPUT.cursor_position()
    }

    #[inline]
    pub fn move_absolute(x: i32, y: i32) {
        INPUT.move_absolute(x, y, false);
    }

    #[inline]
    pub fn move_absolute_recursive(x: i32, y: i32) {
        INPUT.move_absolute(x, y, true);
    }

    #[inline]
    pub fn move_relative(dx: i32, dy: i32) {
        INPUT.move_relative(dx, dy, false);
    }

    #[inline]
    pub fn move_relative_recursive(dx: i32, dy: i32) {
        INPUT.move_relative(dx, dy, true);
    }

    #[inline]
    pub fn rotate(speed: i32) {
        INPUT.rotate_wheel(speed, false);
    }

    #[inline]
    pub fn rotate_recursive(speed: i32) {
        INPUT.rotate_wheel(speed, true);
    }
}

static HOOK: Lazy<Hook> = Lazy::new(Hook::new);

pub fn install_hook() -> EventReceiver {
    unsafe {
        // If this is not executed, the GetCursorPos function returns an invalid cursor position.
        HiDpi::SetProcessDpiAwarenessContext(HiDpi::DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE);
    }

    INPUT.update_cursor_position();

    let (tx, rx) = event::channel();
    HOOK.install(tx);

    rx
}
