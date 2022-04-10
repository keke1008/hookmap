mod conversion;
mod keyboard;
mod mouse;

use crate::common::{
    button::{Button, ButtonKind},
    event::EventProvider,
};
use std::{
    mem::MaybeUninit,
    ptr,
    sync::atomic::{AtomicBool, Ordering},
};
use winapi::{
    ctypes::c_int,
    shared::minwindef::{LPARAM, LRESULT, WPARAM},
    um::winuser,
};

static IGNORED_DW_EXTRA_INFO: usize = 0x1;

fn call_next_hook(n_code: c_int, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    unsafe {
        winuser::CallNextHookEx(
            // This parameter is ignored.
            MaybeUninit::zeroed().assume_init(),
            n_code,
            w_param,
            l_param,
        )
    }
}

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
        match self.kind() {
            ButtonKind::Key => keyboard::press(self, false),
            ButtonKind::Mouse => mouse::press(self, false),
        }
    }

    #[inline]
    pub fn press_recursive(self) {
        self.assume_pressed();
        match self.kind() {
            ButtonKind::Key => keyboard::press(self, true),
            ButtonKind::Mouse => mouse::press(self, true),
        }
    }

    #[inline]
    pub fn release(self) {
        self.assume_released();
        match self.kind() {
            ButtonKind::Key => keyboard::release(self, false),
            ButtonKind::Mouse => mouse::release(self, false),
        }
    }

    #[inline]
    pub fn release_recursive(self) {
        self.assume_released();
        match self.kind() {
            ButtonKind::Key => keyboard::release(self, true),
            ButtonKind::Mouse => mouse::release(self, true),
        }
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
        !BUTTON_STATE.is_released(self, Ordering::SeqCst)
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

pub(crate) fn install_hook(event_provider: EventProvider) {
    keyboard::install_hook(event_provider.clone());
    mouse::install_hook(event_provider);
    unsafe {
        winuser::GetMessageW(MaybeUninit::zeroed().assume_init(), ptr::null_mut(), 0, 0);
    }
}
