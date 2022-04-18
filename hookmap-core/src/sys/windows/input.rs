use super::{vkcode, IGNORED_DW_EXTRA_INFO};
use crate::common::button::{Button, ButtonAction, ButtonKind};

use std::mem::{self, MaybeUninit};

// For many constants.
use winapi::um::winuser::{self, *};

fn create_input_struct(button: Button, action: ButtonAction, recursive: bool) -> INPUT {
    let dw_extra_info = if recursive { 0 } else { IGNORED_DW_EXTRA_INFO };
    match button.kind() {
        ButtonKind::Key => {
            let flags = match action {
                ButtonAction::Press => 0,
                ButtonAction::Release => KEYEVENTF_KEYUP,
            };
            let keybd_input = KEYBDINPUT {
                wVk: vkcode::from_button(button),
                wScan: 0,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: dw_extra_info,
            };
            INPUT {
                type_: INPUT_KEYBOARD,
                u: unsafe { mem::transmute_copy(&keybd_input) },
            }
        }
        ButtonKind::Mouse => {
            let (mouse_data, dw_flags) = match action {
                ButtonAction::Press => match button {
                    Button::LeftButton => (0, MOUSEEVENTF_LEFTDOWN),
                    Button::RightButton => (0, MOUSEEVENTF_RIGHTDOWN),
                    Button::MiddleButton => (0, MOUSEEVENTF_MIDDLEDOWN),
                    Button::SideButton1 => (XBUTTON1, MOUSEEVENTF_XDOWN),
                    Button::SideButton2 => (XBUTTON2, MOUSEEVENTF_XDOWN),
                    _ => unreachable!(),
                },
                ButtonAction::Release => match button {
                    Button::LeftButton => (0, MOUSEEVENTF_LEFTUP),
                    Button::RightButton => (0, MOUSEEVENTF_RIGHTUP),
                    Button::MiddleButton => (0, MOUSEEVENTF_MIDDLEUP),
                    Button::SideButton1 => (XBUTTON1, MOUSEEVENTF_XUP),
                    Button::SideButton2 => (XBUTTON2, MOUSEEVENTF_XUP),
                    _ => unreachable!(),
                },
            };
            let mouse_input = MOUSEINPUT {
                dx: 0,
                dy: 0,
                time: 0,
                mouseData: mouse_data as u32,
                dwFlags: dw_flags,
                dwExtraInfo: dw_extra_info,
            };
            INPUT {
                type_: INPUT_MOUSE,
                u: unsafe { mem::transmute(mouse_input) },
            }
        }
    }
}

#[inline]
pub(super) fn send_input(button: Button, action: ButtonAction, recursive: bool) {
    unsafe {
        winuser::SendInput(
            1,
            &mut create_input_struct(button, action, recursive),
            mem::size_of::<INPUT>() as i32,
        );
    }
}

fn create_mouse_input(dx: i32, dy: i32, mouse_data: u32, dw_flags: u32) -> INPUT {
    let input = MOUSEINPUT {
        dx,
        dy,
        mouseData: mouse_data,
        dwFlags: dw_flags,
        time: 0,
        dwExtraInfo: IGNORED_DW_EXTRA_INFO,
    };
    INPUT {
        type_: INPUT_MOUSE,
        u: unsafe { mem::transmute(input) },
    }
}

#[inline]
pub fn rotate(speed: i32) {
    let speed = speed * WHEEL_DELTA as i32;
    let mut input = create_mouse_input(0, 0, speed as u32, MOUSEEVENTF_WHEEL);
    unsafe {
        winuser::SendInput(1, &mut input, mem::size_of::<INPUT>() as i32);
    }
}

#[inline]
pub fn get_position() -> (i32, i32) {
    unsafe {
        let mut pos = MaybeUninit::zeroed().assume_init();
        winuser::GetCursorPos(&mut pos);
        (pos.x, pos.y)
    }
}

#[inline]
pub fn move_relative(dx: i32, dy: i32) {
    unsafe {
        // The SendInput function moves the cursor to an incorrect position, so
        // SetCursorPos is used to move it.
        let pos = get_position();
        winuser::SetCursorPos(pos.0 + dx, pos.1 + dy);

        // In some applications, the SetCursorPos function alone is not enough
        // to notice the cursor move, so the SendInput function is used to move
        // the cursor by 0.
        let mut input = create_mouse_input(0, 0, 0, MOUSEEVENTF_MOVE);
        winuser::SendInput(1, &mut input, mem::size_of::<INPUT>() as i32);
    }
}

#[inline]
pub fn move_absolute(x: i32, y: i32) {
    unsafe {
        // The SendInput function moves the cursor to an incorrect position, so
        // SetCursorPos is used to move it.
        winuser::SetCursorPos(x, y);

        // In some applications, the SetCursorPos function alone is not enough
        // to notice the cursor move, so the SendInput function is used to move
        // the cursor by 0.
        let mut input = create_mouse_input(0, 0, 0, MOUSEEVENTF_MOVE);
        winuser::SendInput(1, &mut input, mem::size_of::<INPUT>() as i32);
    }
}
