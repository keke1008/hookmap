use super::IGNORED_DW_EXTRA_INFO;
use crate::common::button::Button;
use std::mem;
use winapi::{
    ctypes::c_int,
    um::winuser::{self, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, KEYEVENTF_SCANCODE},
};

pub(crate) fn press(key: &Button, recursive: bool) {
    send_key_input(key, 0, recursive);
}

pub(crate) fn release(key: &Button, recursive: bool) {
    send_key_input(key, KEYEVENTF_KEYUP, recursive);
}

fn send_key_input(key: &Button, flags: u32, recursive: bool) {
    let (scancode, flags) = if let Some((scancode, flag)) = key.to_scancode_and_flag() {
        (scancode, flags | flag)
    } else {
        return;
    };
    let keybd_input = KEYBDINPUT {
        wVk: 0,
        wScan: scancode as u16,
        dwFlags: flags | KEYEVENTF_SCANCODE,
        time: 0,
        dwExtraInfo: if recursive { 0 } else { IGNORED_DW_EXTRA_INFO },
    };
    let mut input = INPUT {
        type_: INPUT_KEYBOARD,
        u: unsafe { mem::transmute_copy(&keybd_input) },
    };

    unsafe {
        winuser::SendInput(1, &mut input, mem::size_of::<INPUT>() as c_int);
    }
}
