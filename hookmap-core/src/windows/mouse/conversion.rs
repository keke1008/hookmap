use crate::{
    common::{button::ButtonAction, mouse::Mouse},
    Button, EmulateMouseCursor,
};
use winapi::{
    shared::minwindef::{HIWORD, WPARAM},
    um::winuser::{
        self, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_MIDDLEDOWN,
        MOUSEEVENTF_MIDDLEUP, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP, MOUSEEVENTF_XDOWN,
        MOUSEEVENTF_XUP, MSLLHOOKSTRUCT, WHEEL_DELTA, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MBUTTONDOWN,
        WM_MBUTTONUP, WM_MOUSEMOVE, WM_MOUSEWHEEL, WM_RBUTTONDOWN, WM_RBUTTONUP, WM_XBUTTONDOWN,
        WM_XBUTTONUP, XBUTTON1, XBUTTON2,
    },
};

pub(super) struct MouseParameter {
    pub(super) mouse_data: u16,
    pub(super) dw_flags: u32,
}

impl MouseParameter {
    fn new(mouse_data: u16, dw_flags: u32) -> Self {
        Self {
            mouse_data,
            dw_flags,
        }
    }
}

impl Button {
    pub(super) fn into_press(self) -> MouseParameter {
        match self {
            Button::LeftButton => MouseParameter::new(0, MOUSEEVENTF_LEFTDOWN),
            Button::RightButton => MouseParameter::new(0, MOUSEEVENTF_RIGHTDOWN),
            Button::MiddleButton => MouseParameter::new(0, MOUSEEVENTF_MIDDLEDOWN),
            Button::SideButton1 => MouseParameter::new(XBUTTON1, MOUSEEVENTF_XDOWN),
            Button::SideButton2 => MouseParameter::new(XBUTTON2, MOUSEEVENTF_XDOWN),
            _ => panic!("{:?} is not a mouse button.", self),
        }
    }

    pub(super) fn into_release(self) -> MouseParameter {
        let (mouse_data, dw_flags) = match self {
            Button::LeftButton => (0, MOUSEEVENTF_LEFTUP),
            Button::RightButton => (0, MOUSEEVENTF_RIGHTUP),
            Button::MiddleButton => (0, MOUSEEVENTF_MIDDLEUP),
            Button::SideButton1 => (XBUTTON1, MOUSEEVENTF_XUP),
            Button::SideButton2 => (XBUTTON2, MOUSEEVENTF_XUP),
            _ => panic!("{:?} is not a mouse button.", self),
        };
        MouseParameter::new(mouse_data, dw_flags)
    }
}

enum MouseTarget {
    Button(Button),
    Wheel,
    Cursor,
}

#[derive(Clone, Copy)]
pub(super) enum MouseEventInfo {
    Button(Button, ButtonAction),
    Wheel(i32),
    Cursor((i32, i32)),
}

impl MouseEventInfo {
    pub(super) fn new(w_param: WPARAM, event_info: MSLLHOOKSTRUCT) -> Option<Self> {
        let event_info = match Self::get_target(w_param, event_info)? {
            MouseTarget::Button(button) => Self::Button(button, Self::get_action(w_param as u32)?),
            MouseTarget::Cursor => Self::Cursor(Mouse::get_pos()),
            MouseTarget::Wheel => {
                let speed = winuser::GET_WHEEL_DELTA_WPARAM(event_info.mouseData as usize);
                Self::Wheel(speed as i32 / WHEEL_DELTA as i32)
            }
        };
        Some(event_info)
    }

    fn get_target(w_param: WPARAM, event_info: MSLLHOOKSTRUCT) -> Option<MouseTarget> {
        let input = match w_param as u32 {
            WM_MOUSEWHEEL => return Some(MouseTarget::Wheel),
            WM_MOUSEMOVE => return Some(MouseTarget::Cursor),
            WM_LBUTTONDOWN | WM_LBUTTONUP => Button::LeftButton,
            WM_RBUTTONDOWN | WM_RBUTTONUP => Button::RightButton,
            WM_MBUTTONDOWN | WM_MBUTTONUP => Button::MiddleButton,
            WM_XBUTTONDOWN | WM_XBUTTONUP if HIWORD(event_info.mouseData) == XBUTTON1 => {
                Button::SideButton1
            }
            WM_XBUTTONDOWN | WM_XBUTTONUP if HIWORD(event_info.mouseData) == XBUTTON2 => {
                Button::SideButton2
            }
            _ => None?,
        };
        Some(MouseTarget::Button(input))
    }

    fn get_action(w_param: u32) -> Option<ButtonAction> {
        match w_param as u32 {
            WM_LBUTTONDOWN | WM_RBUTTONDOWN | WM_MBUTTONDOWN | WM_XBUTTONDOWN => {
                Some(ButtonAction::Press)
            }
            WM_LBUTTONUP | WM_RBUTTONUP | WM_MBUTTONUP | WM_XBUTTONUP => {
                Some(ButtonAction::Release)
            }
            _ => None,
        }
    }
}
