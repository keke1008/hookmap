use super::IGNORED_DW_EXTRA_INFO;
use crate::common::{
    button::Button,
    mouse::{EmulateMouseCursor, EmulateMouseWheel, Mouse},
};
use std::mem::{self, MaybeUninit};
use winapi::{ctypes::c_int, um::winuser};
// For many constants.
use winapi::um::winuser::*;

fn send_mouse_input(dx: i32, dy: i32, mouse_data: u32, dw_flags: u32, recursive: bool) {
    let mouse_input = MOUSEINPUT {
        dx,
        dy,
        mouseData: mouse_data,
        dwFlags: dw_flags,
        dwExtraInfo: if recursive { 0 } else { IGNORED_DW_EXTRA_INFO },
        time: 0,
    };
    let mut input = INPUT {
        type_: 0,
        u: unsafe { mem::transmute(mouse_input) },
    };
    unsafe {
        winuser::SendInput(1, &mut input, mem::size_of::<INPUT>() as c_int);
    }
}

pub(in crate::sys::windows) fn press(button: &Button, recursive: bool) {
    let (mouse_data, dw_flags) = match button {
        Button::LeftButton => (0, MOUSEEVENTF_LEFTDOWN),
        Button::RightButton => (0, MOUSEEVENTF_RIGHTDOWN),
        Button::MiddleButton => (0, MOUSEEVENTF_MIDDLEDOWN),
        Button::SideButton1 => (XBUTTON1, MOUSEEVENTF_XDOWN),
        Button::SideButton2 => (XBUTTON2, MOUSEEVENTF_XDOWN),
        _ => panic!("{:?} is not a mouse button.", button),
    };
    send_mouse_input(0, 0, mouse_data as u32, dw_flags as u32, recursive);
}

pub(in crate::sys::windows) fn release(button: &Button, recursive: bool) {
    let (mouse_data, dw_flags) = match button {
        Button::LeftButton => (0, MOUSEEVENTF_LEFTUP),
        Button::RightButton => (0, MOUSEEVENTF_RIGHTUP),
        Button::MiddleButton => (0, MOUSEEVENTF_MIDDLEUP),
        Button::SideButton1 => (XBUTTON1, MOUSEEVENTF_XUP),
        Button::SideButton2 => (XBUTTON2, MOUSEEVENTF_XUP),
        _ => panic!("{:?} is not a mouse button.", button),
    };
    send_mouse_input(0, 0, mouse_data as u32, dw_flags as u32, recursive);
}

impl EmulateMouseWheel for Mouse {
    fn rotate(speed: i32) {
        let speed = speed * WHEEL_DELTA as i32;
        send_mouse_input(0, 0, speed as u32, MOUSEEVENTF_WHEEL, false);
    }
}

impl EmulateMouseCursor for Mouse {
    fn move_rel(dx: i32, dy: i32) {
        let (x, y) = Self::get_pos();
        unsafe { winuser::SetCursorPos(x + dx, y + dy) };
    }

    fn move_abs(x: i32, y: i32) {
        unsafe { winuser::SetCursorPos(x, y) };
    }

    fn get_pos() -> (i32, i32) {
        unsafe {
            let mut pos = MaybeUninit::zeroed().assume_init();
            winuser::GetPhysicalCursorPos(&mut pos);
            (pos.x, pos.y)
        }
    }
}
