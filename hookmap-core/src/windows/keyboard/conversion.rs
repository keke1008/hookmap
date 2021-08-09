use crate::{bihashmap, common::button::Button, ButtonAction};
use bimap::BiHashMap;
use once_cell::sync::Lazy;
use winapi::um::winuser::KBDLLHOOKSTRUCT;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkCode(pub(super) u32);

impl From<Button> for VkCode {
    fn from(button: Button) -> Self {
        let code = match button {
            Button::OtherKey(code) => code,
            _ => *VK_CODE_MAP.get_by_left(&button).unwrap(),
        };
        VkCode(code)
    }
}

impl From<VkCode> for Button {
    fn from(code: VkCode) -> Self {
        match VK_CODE_MAP.get_by_right(&code.0) {
            Some(&button) => button,
            None => Button::OtherKey(code.0),
        }
    }
}

pub(super) fn into_action(event_info: KBDLLHOOKSTRUCT) -> ButtonAction {
    match event_info.flags >> 7 {
        0 => ButtonAction::Press,
        _ => ButtonAction::Release,
    }
}

pub(super) static VK_CODE_MAP: Lazy<BiHashMap<Button, u32>> = Lazy::new(|| {
    bihashmap! {
        Button::Backspace => 0x08,
        Button::Tab => 0x09,
        Button::Enter => 0x0D,
        Button::Shift => 0x10,
        Button::Ctrl => 0x11,
        Button::Alt => 0x12,
        Button::CapsLock => 0x14,
        Button::Esc => 0x1B,
        Button::Henkan => 0x1C,
        Button::Muhenkan => 0x1D,
        Button::Space => 0x20,
        Button::PageUp => 0x21,
        Button::PageDown => 0x22,
        Button::End => 0x23,
        Button::Home => 0x24,
        Button::LeftArrow => 0x25,
        Button::UpArrow => 0x26,
        Button::RightArrow => 0x27,
        Button::DownArrow => 0x28,
        Button::PrintScreen => 0x2C,
        Button::Insert => 0x2D,
        Button::Delete => 0x2E,
        Button::Key0 => 0x30,
        Button::Key1 => 0x31,
        Button::Key2 => 0x32,
        Button::Key3 => 0x33,
        Button::Key4 => 0x34,
        Button::Key5 => 0x35,
        Button::Key6 => 0x36,
        Button::Key7 => 0x37,
        Button::Key8 => 0x38,
        Button::Key9 => 0x39,
        Button::A => 0x41,
        Button::B => 0x42,
        Button::C => 0x43,
        Button::D => 0x44,
        Button::E => 0x45,
        Button::F => 0x46,
        Button::G => 0x47,
        Button::H => 0x48,
        Button::I => 0x49,
        Button::J => 0x4A,
        Button::K => 0x4B,
        Button::L => 0x4C,
        Button::M => 0x4D,
        Button::N => 0x4E,
        Button::O => 0x4F,
        Button::P => 0x50,
        Button::Q => 0x51,
        Button::R => 0x52,
        Button::S => 0x53,
        Button::T => 0x54,
        Button::U => 0x55,
        Button::V => 0x56,
        Button::W => 0x57,
        Button::X => 0x58,
        Button::Y => 0x59,
        Button::Z => 0x5A,
        Button::LMeta => 0x5B,
        Button::RMeta => 0x5C,
        Button::Application => 0x5D,
        Button::Numpad0 => 0x60,
        Button::Numpad1 => 0x61,
        Button::Numpad2 => 0x62,
        Button::Numpad3 => 0x63,
        Button::Numpad4 => 0x64,
        Button::Numpad5 => 0x65,
        Button::Numpad6 => 0x66,
        Button::Numpad7 => 0x67,
        Button::Numpad8 => 0x68,
        Button::Numpad9 => 0x69,
        Button::NumpadAsterisk => 0x6A,
        Button::NumpadPlus => 0x6B,
        Button::NumpadMinus => 0x6D,
        Button::NumpadDot => 0x6E,
        Button::NumpadSlash => 0x6F,
        Button::F1 => 0x70,
        Button::F2 => 0x71,
        Button::F3 => 0x72,
        Button::F4 => 0x73,
        Button::F5 => 0x74,
        Button::F6 => 0x75,
        Button::F7 => 0x76,
        Button::F8 => 0x77,
        Button::F9 => 0x78,
        Button::F10 => 0x79,
        Button::F11 => 0x7A,
        Button::F12 => 0x7B,
        Button::F13 => 0x7C,
        Button::F14 => 0x7D,
        Button::F15 => 0x7E,
        Button::F16 => 0x7F,
        Button::F17 => 0x80,
        Button::F18 => 0x81,
        Button::F19 => 0x82,
        Button::F20 => 0x83,
        Button::F21 => 0x84,
        Button::F22 => 0x85,
        Button::F23 => 0x86,
        Button::F24 => 0x87,
        Button::Numlock => 0x90,
        Button::ScrollLock => 0x91,
        Button::LShift => 0xA0,
        Button::RShift => 0xA1,
        Button::LCtrl => 0xA2,
        Button::RCtrl => 0xA3,
        Button::LAlt => 0xA4,
        Button::RAlt => 0xA5,
        Button::Colon => 0xBA,
        Button::SemiColon => 0xBB,
        Button::Comma => 0xBC,
        Button::Minus => 0xBD,
        Button::Dot => 0xBE,
        Button::Slash => 0xBF,
        Button::At => 0xC0,
        Button::LeftSquareBracket => 0xDB,
        Button::BackSlashWithVerticalBar => 0xDC,
        Button::RightSquareBracket => 0xDD,
        Button::Hat => 0xDE,
        Button::BackSlashWithUnderLine => 0xE2,
        Button::Eisuu => 0xF0,
        Button::KatakanaHiragana => 0xF2,
        Button::HannkakuZenkaku => 0xF3,
    }
});
