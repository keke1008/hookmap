mod handler;
mod keyboard;
mod mouse;

use std::mem::MaybeUninit;
use winapi::{
    ctypes::c_int,
    shared::minwindef::{LPARAM, LRESULT, WPARAM},
    um::winuser,
};

use crate::{KeyboardAction, MouseAction};

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

enum ButtonAction {
    Press,
    Release,
}

impl From<KeyboardAction> for ButtonAction {
    fn from(action: KeyboardAction) -> Self {
        match action {
            KeyboardAction::Press => ButtonAction::Press,
            KeyboardAction::Release => ButtonAction::Release,
        }
    }
}

impl From<MouseAction> for ButtonAction {
    fn from(action: MouseAction) -> Self {
        match action {
            MouseAction::Press => ButtonAction::Press,
            MouseAction::Release => ButtonAction::Release,
            _ => panic!("{:?} is not a button.", action),
        }
    }
}

fn set_button_state(vk_code: u32, action: impl Into<ButtonAction>) {
    let vk_code = vk_code as usize;
    let mut buffer = [0; 256];
    unsafe {
        winuser::GetKeyboardState(&buffer as *const _ as *mut _);
        match action.into() {
            ButtonAction::Press => buffer[vk_code] |= 1 << 7,
            ButtonAction::Release => buffer[vk_code] &= !0u8 >> 1,
        }
        winuser::SetKeyboardState(&buffer as *const _ as *mut _);
        winuser::GetKeyboardState(&buffer as *const _ as *mut _);
    };
}
