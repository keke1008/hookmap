use super::{conversion::VkCode, DW_EXTRA_INFO};
use crate::common::button::Button;
use std::mem;
use winapi::{
    ctypes::c_int,
    um::winuser::{self, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP},
};

pub(crate) fn press(key: &Button) {
    send_key_input(key, 0);
}

pub(crate) fn release(key: &Button) {
    send_key_input(key, KEYEVENTF_KEYUP);
}

fn send_key_input(key: &Button, flags: u32) {
    let vk_code = VkCode::from(*key).0;
    let keybd_input = KEYBDINPUT {
        wVk: vk_code as u16,
        wScan: 0,
        dwFlags: flags,
        time: 0,
        dwExtraInfo: DW_EXTRA_INFO,
    };
    let mut input = INPUT {
        type_: INPUT_KEYBOARD,
        u: unsafe { mem::transmute_copy(&keybd_input) },
    };

    unsafe {
        winuser::SendInput(1, &mut input, mem::size_of::<INPUT>() as c_int);
    }
}
