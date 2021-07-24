use crate::common::{
    keyboard::Key,
    mouse::{MouseAction, MouseInput},
};
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

mod keycode;
use keycode::SCANCODE_MAP;

impl From<Key> for u32 {
    fn from(key: Key) -> Self {
        match key {
            Key::Other(code) => code,
            _ => *SCANCODE_MAP.get_by_left(&key).unwrap(),
        }
    }
}

impl From<u32> for Key {
    fn from(code: u32) -> Self {
        if let Some(key) = SCANCODE_MAP.get_by_right(&code) {
            *key
        } else {
            Key::Other(code)
        }
    }
}

impl MouseInput {
    pub(super) fn into_press_parameter(self) -> (u16, u32) {
        match self {
            MouseInput::LButton => (0, MOUSEEVENTF_LEFTDOWN),
            MouseInput::RButton => (0, MOUSEEVENTF_RIGHTDOWN),
            MouseInput::MButton => (0, MOUSEEVENTF_MIDDLEDOWN),
            MouseInput::SideButton1 => (XBUTTON1, MOUSEEVENTF_XDOWN),
            MouseInput::SideButton2 => (XBUTTON2, MOUSEEVENTF_XDOWN),
            _ => panic!("{:?} cannnot be pressed. It's not a button.", self),
        }
    }

    pub(super) fn into_release_parameter(self) -> (u16, u32) {
        match self {
            MouseInput::LButton => (0, MOUSEEVENTF_LEFTUP),
            MouseInput::RButton => (0, MOUSEEVENTF_RIGHTUP),
            MouseInput::MButton => (0, MOUSEEVENTF_MIDDLEUP),
            MouseInput::SideButton1 => (XBUTTON1, MOUSEEVENTF_XUP),
            MouseInput::SideButton2 => (XBUTTON2, MOUSEEVENTF_XUP),
            _ => panic!("{:?} cannnot be released. It's not a button.", self),
        }
    }

    pub(super) fn into_vk_code(self) -> i32 {
        match self {
            MouseInput::LButton => VK_LBUTTON,
            MouseInput::RButton => VK_RBUTTON,
            MouseInput::MButton => VK_MBUTTON,
            MouseInput::SideButton1 => VK_XBUTTON1,
            MouseInput::SideButton2 => VK_XBUTTON2,
            _ => panic!("Cannot check if {:?} is pressed. It's not a button.", self),
        }
    }
}

impl From<(WPARAM, MSLLHOOKSTRUCT)> for MouseInput {
    fn from((w_param, mouse_struct): (WPARAM, MSLLHOOKSTRUCT)) -> Self {
        match w_param as u32 {
            WM_MOUSEWHEEL => MouseInput::Wheel,
            WM_MOUSEMOVE => MouseInput::Move,
            WM_LBUTTONDOWN | WM_LBUTTONUP => MouseInput::LButton,
            WM_RBUTTONDOWN | WM_RBUTTONUP => MouseInput::RButton,
            WM_MBUTTONDOWN | WM_MBUTTONUP => MouseInput::MButton,
            WM_XBUTTONDOWN | WM_XBUTTONUP if HIWORD(mouse_struct.mouseData) == XBUTTON1 => {
                MouseInput::SideButton1
            }
            WM_XBUTTONDOWN | WM_XBUTTONUP if HIWORD(mouse_struct.mouseData) == XBUTTON2 => {
                MouseInput::SideButton2
            }
            _ => {
                println!("{}, {}", mouse_struct.mouseData, XBUTTON1);
                unreachable!();
            }
        }
    }
}

impl From<(WPARAM, MSLLHOOKSTRUCT)> for MouseAction {
    fn from((w_param, mouse_struct): (WPARAM, MSLLHOOKSTRUCT)) -> Self {
        match w_param as u32 {
            WM_MOUSEMOVE => MouseAction::Move((mouse_struct.pt.x, mouse_struct.pt.y)),
            WM_MOUSEWHEEL => {
                let speed = winuser::GET_WHEEL_DELTA_WPARAM(mouse_struct.mouseData as usize);
                MouseAction::Wheel(speed as i32)
            }
            WM_LBUTTONDOWN | WM_RBUTTONDOWN | WM_MBUTTONDOWN | WM_XBUTTONDOWN => MouseAction::Press,
            WM_LBUTTONUP | WM_RBUTTONUP | WM_MBUTTONUP | WM_XBUTTONUP => MouseAction::Release,
            _ => unreachable!(),
        }
    }
}
