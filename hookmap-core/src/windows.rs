mod keyboard;
mod mouse;

use crate::common::{
    button::{Button, ButtonInput, ButtonKind, ButtonState},
    handler::{HookInstaller, InputHandler},
};
use once_cell::sync::Lazy;
use std::{collections::HashMap, mem::MaybeUninit, ptr, sync::Mutex};
use winapi::{
    ctypes::c_int,
    shared::minwindef::{LPARAM, LRESULT, WPARAM},
    um::winuser,
};

static DW_EXTRA_INFO: usize = 0x1;

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

static BUTTON_STATE: Lazy<Mutex<HashMap<Button, bool>>> = Lazy::new(Mutex::default);

impl ButtonState for Button {
    fn is_pressed(&self) -> bool {
        *BUTTON_STATE.lock().unwrap().get(self).unwrap_or(&false)
    }
}

impl ButtonInput for Button {
    fn press(&self) {
        self.assume_pressed();
        match self.kind() {
            ButtonKind::Key => keyboard::press(self),
            ButtonKind::Mouse => mouse::press(self),
        }
    }

    fn release(&self) {
        self.assume_pressed();
        match self.kind() {
            ButtonKind::Key => keyboard::release(self),
            ButtonKind::Mouse => mouse::release(self),
        }
    }
}

impl Button {
    pub(self) fn assume_pressed(&self) {
        BUTTON_STATE.lock().unwrap().insert(*self, true);
    }

    pub(self) fn assume_released(&self) {
        BUTTON_STATE.lock().unwrap().insert(*self, false);
    }
}

impl HookInstaller for InputHandler {
    fn install() {
        keyboard::install_hook();
        mouse::install_hook();
    }

    fn handle_input() {
        unsafe {
            winuser::GetMessageW(MaybeUninit::zeroed().assume_init(), ptr::null_mut(), 0, 0);
        }
    }
}
