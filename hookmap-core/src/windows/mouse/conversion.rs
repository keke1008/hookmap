use crate::common::mouse::{MouseAction, MouseInput};
use winapi::{
    shared::minwindef::{HIWORD, WPARAM},
    um::winuser::{
        self, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_MIDDLEDOWN,
        MOUSEEVENTF_MIDDLEUP, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP, MOUSEEVENTF_XDOWN,
        MOUSEEVENTF_XUP, MSLLHOOKSTRUCT, VK_LBUTTON, VK_MBUTTON, VK_RBUTTON, VK_XBUTTON1,
        VK_XBUTTON2, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MBUTTONDOWN, WM_MBUTTONUP, WM_MOUSEMOVE,
        WM_MOUSEWHEEL, WM_RBUTTONDOWN, WM_RBUTTONUP, WM_XBUTTONDOWN, WM_XBUTTONUP, XBUTTON1,
        XBUTTON2,
    },
};

pub(super) struct MouseParameter {
    pub(super) mouse_data: u16,
    pub(super) dw_flags: u32,
}

impl MouseParameter {
    pub(super) fn new(mouse_data: u16, dw_flags: u32) -> Self {
        Self {
            mouse_data,
            dw_flags,
        }
    }
}

impl MouseInput {
    pub(super) fn into_press(self) -> Option<MouseParameter> {
        match self {
            MouseInput::LButton => Some(MouseParameter::new(0, MOUSEEVENTF_LEFTDOWN)),
            MouseInput::RButton => Some(MouseParameter::new(0, MOUSEEVENTF_RIGHTDOWN)),
            MouseInput::MButton => Some(MouseParameter::new(0, MOUSEEVENTF_MIDDLEDOWN)),
            MouseInput::SideButton1 => Some(MouseParameter::new(XBUTTON1, MOUSEEVENTF_XDOWN)),
            MouseInput::SideButton2 => Some(MouseParameter::new(XBUTTON2, MOUSEEVENTF_XDOWN)),
            _ => None,
        }
    }

    pub(super) fn into_release(self) -> Option<MouseParameter> {
        match self {
            MouseInput::LButton => Some(MouseParameter::new(0, MOUSEEVENTF_LEFTUP)),
            MouseInput::RButton => Some(MouseParameter::new(0, MOUSEEVENTF_RIGHTUP)),
            MouseInput::MButton => Some(MouseParameter::new(0, MOUSEEVENTF_MIDDLEUP)),
            MouseInput::SideButton1 => Some(MouseParameter::new(XBUTTON1, MOUSEEVENTF_XUP)),
            MouseInput::SideButton2 => Some(MouseParameter::new(XBUTTON2, MOUSEEVENTF_XUP)),
            _ => None,
        }
    }

    pub(super) fn into_vk_code(self) -> Option<i32> {
        match self {
            MouseInput::LButton => Some(VK_LBUTTON),
            MouseInput::RButton => Some(VK_RBUTTON),
            MouseInput::MButton => Some(VK_MBUTTON),
            MouseInput::SideButton1 => Some(VK_XBUTTON1),
            MouseInput::SideButton2 => Some(VK_XBUTTON2),
            _ => None,
        }
    }
}

pub struct MouseEventInfo {
    w_param: WPARAM,
    mouse_struct: MSLLHOOKSTRUCT,
}

impl MouseEventInfo {
    pub(super) fn new(w_param: WPARAM, mouse_struct: MSLLHOOKSTRUCT) -> Self {
        Self {
            w_param,
            mouse_struct,
        }
    }

    pub(super) fn into_input(self) -> Option<MouseInput> {
        let input = match self.w_param as u32 {
            WM_MOUSEWHEEL => MouseInput::Wheel,
            WM_MOUSEMOVE => MouseInput::Move,
            WM_LBUTTONDOWN | WM_LBUTTONUP => MouseInput::LButton,
            WM_RBUTTONDOWN | WM_RBUTTONUP => MouseInput::RButton,
            WM_MBUTTONDOWN | WM_MBUTTONUP => MouseInput::MButton,
            WM_XBUTTONDOWN | WM_XBUTTONUP if HIWORD(self.mouse_struct.mouseData) == XBUTTON1 => {
                MouseInput::SideButton1
            }
            WM_XBUTTONDOWN | WM_XBUTTONUP if HIWORD(self.mouse_struct.mouseData) == XBUTTON2 => {
                MouseInput::SideButton2
            }
            _ => None?,
        };
        Some(input)
    }

    pub(super) fn into_action(self) -> Option<MouseAction> {
        let action = match self.w_param as u32 {
            WM_MOUSEMOVE => MouseAction::Move((self.mouse_struct.pt.x, self.mouse_struct.pt.y)),
            WM_MOUSEWHEEL => {
                let speed = winuser::GET_WHEEL_DELTA_WPARAM(self.mouse_struct.mouseData as usize);
                MouseAction::Wheel(speed as i32)
            }
            WM_LBUTTONDOWN | WM_RBUTTONDOWN | WM_MBUTTONDOWN | WM_XBUTTONDOWN => MouseAction::Press,
            WM_LBUTTONUP | WM_RBUTTONUP | WM_MBUTTONUP | WM_XBUTTONUP => MouseAction::Release,
            _ => None?,
        };
        Some(action)
    }
}
