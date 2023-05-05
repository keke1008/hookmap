use windows::Win32::{
    Foundation::WPARAM,
    UI::{Input::KeyboardAndMouse::*, WindowsAndMessaging::*},
};

use crate::{
    button::{Button, ButtonAction, ButtonKind},
    event::{ButtonEvent, WheelEvent},
};

use super::{input::get_cursor_position, vkcode};

const IGNORE: usize = 0b01;

fn create_dw_extra_info(recursive: bool) -> usize {
    if recursive {
        0
    } else {
        IGNORE
    }
}

pub(super) fn to_button_event(input: &KBDLLHOOKSTRUCT) -> Option<ButtonEvent> {
    if input.dwExtraInfo & IGNORE == IGNORE {
        return None;
    }

    let vkcode = VIRTUAL_KEY(input.vkCode as u16);
    let target = vkcode::into_button(vkcode)?;

    let action = if input.flags & LLKHF_UP == LLKHF_UP {
        ButtonAction::Release
    } else {
        ButtonAction::Press
    };

    let injected = input.flags & LLKHF_INJECTED == LLKHF_INJECTED;

    Some(ButtonEvent {
        target,
        action,
        injected,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct WindowsCursorEvent {
    pub(super) position: (i32, i32),
    pub(super) injected: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum MouseEvent {
    Button(ButtonEvent),
    Cursor(WindowsCursorEvent),
    Wheel(WheelEvent),
}

pub(super) fn to_mouse_event(wparam: WPARAM, input: &MSLLHOOKSTRUCT) -> Option<MouseEvent> {
    if input.dwExtraInfo & IGNORE == IGNORE {
        return None;
    }

    let injected = input.flags & LLMHF_INJECTED == LLMHF_INJECTED;

    let wparam = wparam.0 as u32;

    if wparam == WM_MOUSEWHEEL {
        let delta = (input.mouseData.0 as i32 >> 16) / WHEEL_DELTA as i32;
        return Some(MouseEvent::Wheel(WheelEvent { delta, injected }));
    }

    if wparam == WM_MOUSEMOVE {
        return Some(MouseEvent::Cursor(WindowsCursorEvent {
            position: (input.pt.x, input.pt.y),
            injected,
        }));
    }

    use Button::{LeftButton, MiddleButton, RightButton, SideButton1, SideButton2};
    use ButtonAction::*;
    let (target, action) = match wparam {
        WM_LBUTTONDOWN => (LeftButton, Press),
        WM_LBUTTONUP => (LeftButton, Release),
        WM_RBUTTONDOWN => (RightButton, Press),
        WM_RBUTTONUP => (RightButton, Release),
        WM_MBUTTONDOWN => (MiddleButton, Press),
        WM_MBUTTONUP => (MiddleButton, Release),
        WM_XBUTTONDOWN if input.mouseData.0 >> 16 == XBUTTON1.0 => (SideButton1, Press),
        WM_XBUTTONUP if input.mouseData.0 >> 16 == XBUTTON1.0 => (SideButton1, Release),
        WM_XBUTTONDOWN if input.mouseData.0 >> 16 == XBUTTON2.0 => (SideButton2, Press),
        WM_XBUTTONUP if input.mouseData.0 >> 16 == XBUTTON2.0 => (SideButton2, Release),
        _ => return None,
    };
    Some(MouseEvent::Button(ButtonEvent {
        target,
        action,
        injected,
    }))
}

fn to_key_input(key: Button, action: ButtonAction, recursive: bool) -> INPUT {
    let dw_flags = match action {
        ButtonAction::Press => KEYBD_EVENT_FLAGS(0),
        ButtonAction::Release => KEYEVENTF_KEYUP,
    };

    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: vkcode::from_button(key),
                dwFlags: dw_flags,
                dwExtraInfo: create_dw_extra_info(recursive),
                ..Default::default()
            },
        },
    }
}

fn to_mouse_input(
    mut dx: i32,
    mut dy: i32,
    mouse_data: i32,
    dw_flags: MOUSE_EVENT_FLAGS,
    recursive: bool,
) -> INPUT {
    if (dx, dy) != (0, 0) {
        let (sx, sy) = unsafe { (GetSystemMetrics(SM_CXSCREEN), GetSystemMetrics(SM_CYSCREEN)) };
        dx = ((dx as i64 * 65536) / (sx as i64)) as i32;
        dy = ((dy as i64 * 65536) / (sy as i64)) as i32;
    }

    INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 {
            mi: MOUSEINPUT {
                dx,
                dy,
                mouseData: mouse_data,
                dwFlags: dw_flags,
                time: 0,
                dwExtraInfo: create_dw_extra_info(recursive),
            },
        },
    }
}

fn to_mouse_button_input(button: Button, action: ButtonAction, recursive: bool) -> INPUT {
    let (mouse_data, dw_flags) = match action {
        ButtonAction::Press => match button {
            Button::LeftButton => (MOUSEHOOKSTRUCTEX_MOUSE_DATA(0), MOUSEEVENTF_LEFTDOWN),
            Button::RightButton => (MOUSEHOOKSTRUCTEX_MOUSE_DATA(0), MOUSEEVENTF_RIGHTDOWN),
            Button::MiddleButton => (MOUSEHOOKSTRUCTEX_MOUSE_DATA(0), MOUSEEVENTF_MIDDLEDOWN),
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

    to_mouse_input(0, 0, mouse_data.0 as i32, dw_flags, recursive)
}

pub(super) fn to_button_input(button: Button, action: ButtonAction, recursive: bool) -> INPUT {
    match button.kind() {
        ButtonKind::Key => to_key_input(button, action, recursive),
        ButtonKind::Mouse => to_mouse_button_input(button, action, recursive),
    }
}

pub(super) fn to_mouse_cursor_input(
    mut x: i32,
    mut y: i32,
    absolute: bool,
    recursive: bool,
) -> INPUT {
    if !absolute {
        let current = get_cursor_position();
        x += current.0;
        y += current.1;
    }

    let dw_flags = MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE;
    to_mouse_input(x, y, 0, dw_flags, recursive)
}

pub(super) fn to_mouse_wheel_input(delta: i32, recursive: bool) -> INPUT {
    let speed = delta * WHEEL_DELTA as i32;
    to_mouse_input(0, 0, speed, MOUSEEVENTF_WHEEL, recursive)
}
