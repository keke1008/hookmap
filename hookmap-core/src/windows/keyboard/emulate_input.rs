use super::DW_EXTRA_INFO;
use crate::common::{
    button::{ButtonInput, ButtonState},
    keyboard::Key,
};
use crate::windows::keyboard::VkCode;
use once_cell::sync::Lazy;
use std::{collections::HashMap, mem, sync::Mutex};
use winapi::{
    ctypes::c_int,
    um::winuser::{self, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP},
};

static KEYBOARD_STATE: Lazy<Mutex<HashMap<Key, bool>>> = Lazy::new(Mutex::default);

impl ButtonInput for Key {
    fn press(&self) {
        send_key_input(self, 0);
        KEYBOARD_STATE.lock().unwrap().insert(*self, true);
    }
    fn release(&self) {
        send_key_input(self, KEYEVENTF_KEYUP);
        KEYBOARD_STATE.lock().unwrap().insert(*self, false);
    }
}

impl ButtonState for Key {
    fn is_pressed(&self) -> bool {
        *KEYBOARD_STATE.lock().unwrap().entry(*self).or_default()
    }
}

impl Key {
    pub(super) fn assume_pressed(&self) {
        KEYBOARD_STATE.lock().unwrap().insert(*self, true);
    }

    pub(super) fn assume_released(&self) {
        KEYBOARD_STATE.lock().unwrap().insert(*self, true);
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

    unsafe {
        winuser::SendInput(1, &mut input, mem::size_of::<INPUT>() as c_int);
    }
}
