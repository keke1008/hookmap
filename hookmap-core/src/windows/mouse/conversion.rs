use crate::{
    common::{mouse::Mouse, ButtonAction},
    EmulateMouseCursor, MouseCursor,
};
use winapi::{
    shared::minwindef::{HIWORD, WPARAM},
    um::winuser::{
        self, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_MIDDLEDOWN,
        MOUSEEVENTF_MIDDLEUP, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP, MOUSEEVENTF_XDOWN,
        MOUSEEVENTF_XUP, MSLLHOOKSTRUCT, VK_LBUTTON, VK_MBUTTON, VK_RBUTTON, VK_XBUTTON1,
        VK_XBUTTON2, WHEEL_DELTA, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MBUTTONDOWN, WM_MBUTTONUP,
        WM_MOUSEMOVE, WM_MOUSEWHEEL, WM_RBUTTONDOWN, WM_RBUTTONUP, WM_XBUTTONDOWN, WM_XBUTTONUP,
        XBUTTON1, XBUTTON2,
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

impl Mouse {
    pub(super) fn into_press(self) -> MouseParameter {
        match self {
            Mouse::LButton => MouseParameter::new(0, MOUSEEVENTF_LEFTDOWN),
            Mouse::RButton => MouseParameter::new(0, MOUSEEVENTF_RIGHTDOWN),
            Mouse::MButton => MouseParameter::new(0, MOUSEEVENTF_MIDDLEDOWN),
            Mouse::SideButton1 => MouseParameter::new(XBUTTON1, MOUSEEVENTF_XDOWN),
            Mouse::SideButton2 => MouseParameter::new(XBUTTON2, MOUSEEVENTF_XDOWN),
        }
    }

    pub(super) fn into_release(self) -> MouseParameter {
        let (mouse_data, dw_flags) = match self {
            Mouse::LButton => (0, MOUSEEVENTF_LEFTUP),
            Mouse::RButton => (0, MOUSEEVENTF_RIGHTUP),
            Mouse::MButton => (0, MOUSEEVENTF_MIDDLEUP),
            Mouse::SideButton1 => (XBUTTON1, MOUSEEVENTF_XUP),
            Mouse::SideButton2 => (XBUTTON2, MOUSEEVENTF_XUP),
        };
        MouseParameter::new(mouse_data, dw_flags)
    }

    pub(super) fn into_vk_code(self) -> u32 {
        match self {
            Mouse::LButton => VK_LBUTTON as u32,
            Mouse::RButton => VK_RBUTTON as u32,
            Mouse::MButton => VK_MBUTTON as u32,
            Mouse::SideButton1 => VK_XBUTTON1 as u32,
            Mouse::SideButton2 => VK_XBUTTON2 as u32,
        }
    }
}

enum MouseTarget {
    Button(Mouse),
    Wheel,
    Cursor,
}

#[derive(Clone, Copy)]
pub(super) enum MouseEventInfo {
    Button(Mouse, ButtonAction),
    Wheel(i32),
    Cursor((i32, i32)),
}

impl MouseEventInfo {
    pub(super) fn new(w_param: WPARAM, event_info: MSLLHOOKSTRUCT) -> Option<Self> {
        let event_info = match Self::get_target(w_param, event_info)? {
            MouseTarget::Button(button) => Self::Button(button, Self::get_action(w_param as u32)?),
            MouseTarget::Cursor => Self::Cursor(MouseCursor::get_pos()),
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
            WM_LBUTTONDOWN | WM_LBUTTONUP => Mouse::LButton,
            WM_RBUTTONDOWN | WM_RBUTTONUP => Mouse::RButton,
            WM_MBUTTONDOWN | WM_MBUTTONUP => Mouse::MButton,
            WM_XBUTTONDOWN | WM_XBUTTONUP if HIWORD(event_info.mouseData) == XBUTTON1 => {
                Mouse::SideButton1
            }
            WM_XBUTTONDOWN | WM_XBUTTONUP if HIWORD(event_info.mouseData) == XBUTTON2 => {
                Mouse::SideButton2
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
