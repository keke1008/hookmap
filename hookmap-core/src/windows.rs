mod handler;
mod keyboard;
mod mouse;

use std::mem::MaybeUninit;
use winapi::{
    ctypes::c_int,
    shared::minwindef::{LPARAM, LRESULT, WPARAM},
    um::winuser,
};

use crate::common::ButtonAction;

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

fn set_button_state(vk_code: u32, action: ButtonAction) {
    let vk_code = vk_code as usize;
    let mut buffer = [0; 256];
    unsafe {
        winuser::GetKeyboardState(buffer.as_mut_ptr());
        match action {
            ButtonAction::Press => buffer[vk_code] |= 1 << 7,
            ButtonAction::Release => buffer[vk_code] &= !0u8 >> 1,
        }
        winuser::SetKeyboardState(buffer.as_mut_ptr());
        winuser::GetKeyboardState(buffer.as_mut_ptr());
    };
}
