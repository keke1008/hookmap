use super::{vkcode, INJECTED_FLAG, SHOULD_BE_IGNORED_FLAG};
use crate::button::{Button, ButtonAction, ButtonKind};

use std::{mem::MaybeUninit, sync::Mutex};

use windows::Win32::UI::Input::KeyboardAndMouse;
use windows::Win32::UI::WindowsAndMessaging;
// For many constants.
use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::Win32::UI::WindowsAndMessaging::*;

const INPUT_MEM_SIZE: i32 = std::mem::size_of::<INPUT>() as i32;

#[inline]
fn create_dw_extra_info(recursive: bool) -> usize {
    INJECTED_FLAG | if recursive { 0 } else { SHOULD_BE_IGNORED_FLAG }
}

fn create_mouse_input(mouse_data: i32, dw_flags: MOUSE_EVENT_FLAGS, recursive: bool) -> INPUT {
    let input = MOUSEINPUT {
        dx: 0,
        dy: 0,
        mouseData: mouse_data,
        dwFlags: dw_flags,
        time: 0,
        dwExtraInfo: create_dw_extra_info(recursive),
    };
    INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 { mi: input },
    }
}

fn create_input_struct(button: Button, action: ButtonAction, recursive: bool) -> INPUT {
    match button.kind() {
        ButtonKind::Key => {
            let flags = match action {
                ButtonAction::Press => KEYBD_EVENT_FLAGS(0),
                ButtonAction::Release => KEYEVENTF_KEYUP,
            };
            let keybd_input = KEYBDINPUT {
                wVk: vkcode::from_button(button),
                wScan: 0,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: create_dw_extra_info(recursive),
            };
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 { ki: keybd_input },
            }
        }
        ButtonKind::Mouse => {
            let (mouse_data, dw_flags) = match action {
                ButtonAction::Press => match button {
                    Button::LeftButton => (MOUSEHOOKSTRUCTEX_MOUSE_DATA(0), MOUSEEVENTF_LEFTDOWN),
                    Button::RightButton => (MOUSEHOOKSTRUCTEX_MOUSE_DATA(0), MOUSEEVENTF_RIGHTDOWN),
                    Button::MiddleButton => {
                        (MOUSEHOOKSTRUCTEX_MOUSE_DATA(0), MOUSEEVENTF_MIDDLEDOWN)
                    }
                    Button::SideButton1 => (XBUTTON1, MOUSEEVENTF_XDOWN),
                    Button::SideButton2 => (XBUTTON2, MOUSEEVENTF_XDOWN),
                    _ => unreachable!(),
                },
                ButtonAction::Release => match button {
                    Button::LeftButton => (MOUSEHOOKSTRUCTEX_MOUSE_DATA(0), MOUSEEVENTF_LEFTUP),
                    Button::RightButton => (MOUSEHOOKSTRUCTEX_MOUSE_DATA(0), MOUSEEVENTF_RIGHTUP),
                    Button::MiddleButton => (MOUSEHOOKSTRUCTEX_MOUSE_DATA(0), MOUSEEVENTF_MIDDLEUP),
                    Button::SideButton1 => (XBUTTON1, MOUSEEVENTF_XUP),
                    Button::SideButton2 => (XBUTTON2, MOUSEEVENTF_XUP),
                    _ => unreachable!(),
                },
            };
            create_mouse_input(mouse_data.0 as i32, dw_flags, recursive)
        }
    }
}

#[inline]
fn get_cursor_position() -> (i32, i32) {
    unsafe {
        let mut pos = MaybeUninit::zeroed().assume_init();
        WindowsAndMessaging::GetCursorPos(&mut pos);
        (pos.x, pos.y)
    }
}

#[derive(Debug)]
pub(super) struct Input {
    cursor_position: Mutex<(i32, i32)>,
}

impl Input {
    pub(super) fn new() -> Self {
        Self {
            cursor_position: Mutex::new(get_cursor_position()),
        }
    }

    pub(super) fn button_input(&self, button: Button, action: ButtonAction, recursive: bool) {
        unsafe {
            KeyboardAndMouse::SendInput(
                &[create_input_struct(button, action, recursive)],
                INPUT_MEM_SIZE,
            );
        }
    }

    pub(super) fn rotate_wheel(&self, speed: i32, recursive: bool) {
        let speed = speed * WHEEL_DELTA as i32;
        let input = create_mouse_input(speed, MOUSEEVENTF_WHEEL, recursive);
        unsafe {
            KeyboardAndMouse::SendInput(&[input], INPUT_MEM_SIZE);
        }
    }

    pub(super) fn cursor_position(&self) -> (i32, i32) {
        get_cursor_position()
    }

    pub(super) fn update_cursor_position(&self) {
        *self.cursor_position.lock().unwrap() = get_cursor_position();
    }

    // FIXME: Since the cursor is moved with SetCursorPos, it cannot be hooked
    // with SetWindowHookEx. Therefore, recursive mouse movement is not possible.
    pub(super) fn move_absolute(&self, x: i32, y: i32, recursive: bool) {
        unsafe {
            // The SendInput function moves the cursor to an incorrect position, so
            // SetCursorPos is used to move it.
            WindowsAndMessaging::SetCursorPos(x, y);

            self.update_cursor_position();

            // In some applications, the SetCursorPos function alone is not enough
            // to notice the cursor move, so the SendInput function is used to move
            // the cursor by 0.
            let input = create_mouse_input(0, MOUSEEVENTF_MOVE, recursive);
            KeyboardAndMouse::SendInput(&[input], INPUT_MEM_SIZE);
        }
    }

    pub(super) fn move_relative(&self, dx: i32, dy: i32, recursive: bool) {
        let current_pos = get_cursor_position();
        let (x, y) = (current_pos.0 + dx, current_pos.1 + dy);
        self.move_absolute(x, y, recursive);
    }
}
