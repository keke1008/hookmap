use crate::keyboard::{EmulateKeyboardInput, Key};
use std::mem;
use winapi::{
    ctypes::c_int,
    shared::minwindef::UINT,
    um::{
        winuser,
        winuser::{INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, LPINPUT},
    },
};

impl EmulateKeyboardInput for Key {
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
        wVk: <Key as Into<u32>>::into(*key) as u16,
        wScan: 0,
        dwFlags: flags,
        time: 0,
        dwExtraInfo: 0,
    };
    let input = &mut INPUT {
        type_: INPUT_KEYBOARD,
        u: unsafe { mem::transmute_copy(&keybd_input) },
    } as LPINPUT;
    unsafe { winuser::SendInput(1 as UINT, input, mem::size_of::<INPUT>() as c_int) };
}

fn get_key_state(key: &Key) -> i16 {
    unsafe { winuser::GetKeyState(<Key as Into<u32>>::into(*key) as i32) }
}
