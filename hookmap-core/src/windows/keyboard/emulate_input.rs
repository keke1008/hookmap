use super::DW_EXTRA_INFO;
use crate::common::{keyboard::Key, EmulateButtonInput};
use crate::windows::keyboard::VkCode;
use std::{mem, thread};
use winapi::{
    ctypes::c_int,
    um::winuser::{self, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP},
};

impl EmulateButtonInput for Key {
    fn press(&self) {
        send_key_input(self, 0);
    }
    fn release(&self) {
        send_key_input(self, KEYEVENTF_KEYUP);
    }

    fn is_pressed(&self) -> bool {
        get_key_state(self) & (1 << 15) != 0
    }
    fn is_toggled(&self) -> bool {
        get_key_state(self) & 1 != 0
    }
}

fn send_key_input(key: &Key, flags: u32) {
    let keybd_input = KEYBDINPUT {
        wVk: <Key as Into<VkCode>>::into(*key).0 as u16,
        wScan: 0,
        dwFlags: flags,
        time: 0,
        dwExtraInfo: DW_EXTRA_INFO,
    };
    let mut input = INPUT {
        type_: INPUT_KEYBOARD,
        u: unsafe { mem::transmute_copy(&keybd_input) },
    };

    thread::spawn(move || unsafe {
        winuser::SendInput(1, &mut input, mem::size_of::<INPUT>() as c_int);
    });
}

fn get_key_state(key: &Key) -> i16 {
    let key_code: VkCode = (*key).into();
    unsafe { winuser::GetKeyState(key_code.0 as i32) as i16 }
}
