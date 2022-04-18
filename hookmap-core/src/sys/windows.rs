mod hook;
mod input;
mod vkcode;

use crate::common::{
    button::{Button, ButtonAction},
    event::{self, EventConsumer},
};
use std::{
    mem::MaybeUninit,
    ptr,
    sync::atomic::{AtomicBool, Ordering},
    thread,
};
use winapi::{shared::windef::DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE, um::winuser};

static IGNORED_DW_EXTRA_INFO: usize = 0x1;

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

impl Button {
    #[inline]
    pub fn press(self) {
        self.assume_pressed();
        input::send_input(self, ButtonAction::Press, false);
    }

    #[inline]
    pub fn press_recursive(self) {
        self.assume_pressed();
        input::send_input(self, ButtonAction::Press, true);
    }

    #[inline]
    pub fn release(self) {
        self.assume_released();
        input::send_input(self, ButtonAction::Release, false);
    }

    #[inline]
    pub fn release_recursive(self) {
        self.assume_released();
        input::send_input(self, ButtonAction::Release, true);
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
    pub use super::input::{get_position, move_absolute, move_relative, rotate};
}

pub fn install_hook() -> EventConsumer {
    unsafe {
        // If this is not executed, the GetCursorPos function returns an invalid cursor position.
        winuser::SetThreadDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE);
    }

    let (event_tx, event_rx) = event::connection();
    thread::spawn(|| {
        hook::install(event_tx);
        unsafe {
            winuser::GetMessageW(MaybeUninit::zeroed().assume_init(), ptr::null_mut(), 0, 0);
        }
    });
    event_rx
}
