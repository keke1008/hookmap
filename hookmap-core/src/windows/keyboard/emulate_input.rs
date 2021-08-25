use super::DW_EXTRA_INFO;
use crate::common::button::Button;
use std::mem;
use winapi::{
    ctypes::c_int,
    um::winuser::{self, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP},
};

pub(crate) fn press(key: &Button, recursive: bool) {
    send_key_input(key, 0, recursive);
}

pub(crate) fn release(key: &Button, recursive: bool) {
    send_key_input(key, KEYEVENTF_KEYUP, recursive);
}

fn send_key_input(key: &Button, flags: u32, recursive: bool) {
    let vk_code = key.to_vkcode();
    let keybd_input = KEYBDINPUT {
        wVk: vk_code as u16,
        wScan: 0,
        dwFlags: flags,
        time: 0,
        dwExtraInfo: if recursive { 0 } else { DW_EXTRA_INFO },
    };
    let mut input = INPUT {
        type_: INPUT_KEYBOARD,
        u: unsafe { mem::transmute_copy(&keybd_input) },
    };

    unsafe {
        winuser::SendInput(1, &mut input, mem::size_of::<INPUT>() as c_int);
    }
}
