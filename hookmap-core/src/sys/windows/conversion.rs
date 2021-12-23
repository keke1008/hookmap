use winapi::um::winuser::KBDLLHOOKSTRUCT;

use crate::common::button::Button;

impl Button {
    pub(super) fn from_hook_struct(hook: &KBDLLHOOKSTRUCT) -> Option<Self> {
        let button = *CODE_TO_BUTTON.get(hook.scanCode as usize)?;
        match button {
            Button::Void => None,
            Button::LCtrl if hook.flags & 1 != 0 => Some(Button::RCtrl),
            Button::RAlt if hook.flags & 1 != 0 => Some(Button::RAlt),
            Button::Insert if hook.flags & 1 == 0 => Some(Button::Numpad0),
            Button::Delete if hook.flags & 1 == 0 => Some(Button::NumpadDot),
            Button::End if hook.flags & 1 == 0 => Some(Button::Numpad0),
            Button::DownArrow if hook.flags & 1 == 0 => Some(Button::Numpad2),
            Button::PageDown if hook.flags & 1 == 0 => Some(Button::Numpad3),
            Button::LeftArrow if hook.flags & 1 == 0 => Some(Button::Numpad4),
            Button::RightArrow if hook.flags & 1 == 0 => Some(Button::Numpad6),
            Button::Home if hook.flags & 1 == 0 => Some(Button::Numpad7),
            Button::UpArrow if hook.flags & 1 == 0 => Some(Button::Numpad8),
            Button::PageUp if hook.flags & 1 == 0 => Some(Button::Numpad9),
            Button::RMeta if hook.flags & 1 == 0 => None,

            _ => Some(button),
        }
    }

    pub(super) fn to_scancode_and_flag(self) -> Option<(u32, u32)> {
        match self {
            Button::RCtrl => Some((*BUTTON_TO_CODE.get(Button::LCtrl as usize)?, 0x1)),
            Button::RAlt => Some((*BUTTON_TO_CODE.get(Button::LAlt as usize)?, 0x1)),
            Button::End => Some((*BUTTON_TO_CODE.get(Button::End as usize)?, 0x1)),
            Button::DownArrow => Some((*BUTTON_TO_CODE.get(Button::DownArrow as usize)?, 0x1)),
            Button::PageDown => Some((*BUTTON_TO_CODE.get(Button::PageDown as usize)?, 0x1)),
            Button::LeftArrow => Some((*BUTTON_TO_CODE.get(Button::LeftArrow as usize)?, 0x1)),
            Button::RightArrow => Some((*BUTTON_TO_CODE.get(Button::RightArrow as usize)?, 0x1)),
            Button::Home => Some((*BUTTON_TO_CODE.get(Button::Home as usize)?, 0x1)),
            Button::UpArrow => Some((*BUTTON_TO_CODE.get(Button::UpArrow as usize)?, 0x1)),
            Button::PageUp => Some((*BUTTON_TO_CODE.get(Button::PageUp as usize)?, 0x1)),
            Button::Insert => Some((*BUTTON_TO_CODE.get(Button::Insert as usize)?, 0x1)),
            Button::Delete => Some((*BUTTON_TO_CODE.get(Button::Delete as usize)?, 0x1)),
            Button::RMeta => Some((*BUTTON_TO_CODE.get(Button::RMeta as usize)?, 0x1)),
            _ => {
                let code = BUTTON_TO_CODE
                    .get(self as usize)
                    .and_then(|&code| (code != 0).then(|| code))?;
                Some((code, 0x0))
            }
        }
    }
}

macro_rules! button_code_map {
    (@single $e:tt) => (());
    (@max $($value:expr),+) => {
        {
            let list = [$($value),+];
            let mut max = list[0];
            let mut i = 1;
            while i < list.len() {
                max = if max < list[i] { list[i] } else { max };
                i += 1;
            }
            max
        }
    };

    (
        const ($button_to_code:ident, $code_to_button:ident) = {
            $(
                $(#[$att:meta])?
                $button:ident => $code:expr
             ),*
            $(,)?
        };
    ) => {
        const $button_to_code: [u32; Button::VARIANT_COUNT] = {
            let mut map = [0; Button::VARIANT_COUNT];
            $(
                $(#[$att])?
                { map[Button::$button as usize] = $code; }
             )*
            map
        };
        const _MAX_CODE_VALUE: usize = {
            button_code_map!(@max $($code),*) + 1
        };
        const $code_to_button: [Button; _MAX_CODE_VALUE] = {
            let mut map = [Button::Void; _MAX_CODE_VALUE];
            $(
                $(#[$att])?
                { map[$code] = Button::$button; }
             )*
            map
        };
    };
}

button_code_map! {
    const (BUTTON_TO_CODE, CODE_TO_BUTTON) = {

        #[cfg(feature="us-keyboard-layout")]
        Tilde => 0x29,
        #[cfg(feature="japanese-keyboard-layout")]
        HankakuZenkaku => 0x29,

        Key1 => 0x02,
        Key2 => 0x03,
        Key3 => 0x04,
        Key4 => 0x05,
        Key5 => 0x06,
        Key6 => 0x07,
        Key7 => 0x08,
        Key8 => 0x09,
        Key9 => 0x0A,
        Key0 => 0x0B,
        Minus => 0x0C,

        #[cfg(feature="us-keyboard-layout")]
        Equal => 0x0D,
        #[cfg(feature="japanese-keyboard-layout")]
        Hat => 0x0D,

        #[cfg(feature="japanese-keyboard-layout")]
        Yen => 0x7D,

        Backspace => 0x0E,
        Tab => 0x0F,

        Q => 0x10,
        W => 0x11,
        E => 0x12,
        R => 0x13,
        T => 0x14,
        Y => 0x15,
        U => 0x16,
        I => 0x17,
        O => 0x18,
        P => 0x19,

        #[cfg(feature = "us-keyboard-layout")]
        OpenSquareBracket => 0x1A,
        #[cfg(feature = "japanese-keyboard-layout")]
        At => 0x1A,

        #[cfg(feature = "us-keyboard-layout")]
        CloseSquareBracket => 0x1B,
        #[cfg(feature = "japanese-keyboard-layout")]
        OpenSquareBracket => 0x1B,

        #[cfg(feature = "us-keyboard-layout")]
        CapsLock => 0x3A,
        #[cfg(feature = "japanese-keyboard-layout")]
        Eisu => 0x3A,

        A => 0x1E,
        S => 0x1F,
        D => 0x20,
        F => 0x21,
        G => 0x22,
        H => 0x23,
        J => 0x24,
        K => 0x25,
        L => 0x26,
        SemiColon => 0x27,

        #[cfg(feature = "us-keyboard-layout")]
        SingleQuote => 0x28,
        #[cfg(feature = "japanese-keyboard-layout")]
        Colon => 0x28,

        #[cfg(feature = "japanese-keyboard-layout")]
        CloseSquareBracket => 0x2B,

        Enter => 0x1C,
        LShift =>0x2A,
        Z => 0x2C,
        X => 0x2D,
        C => 0x2E,
        V => 0x2F,
        B => 0x30,
        N => 0x31,
        M => 0x32,
        Comma =>0x33,
        Dot => 0x34,
        Slash => 0x35,

        #[cfg(feature = "japanese-keyboard-layout")]
        BackSlash => 0x73,

        RShift => 0x36,
        LCtrl => 0x1D,
        LMeta => 0x5B,
        LAlt => 0x38,

        #[cfg(feature = "japanese-keyboard-layout")]
        Muhenkan => 0x7B,

        Space => 0x39,

        #[cfg(feature = "japanese-keyboard-layout")]
        Henkan => 0x79,

        #[cfg(feature = "japanese-keyboard-layout")]
        KatakanaHiragana => 0x70,

        // LAlt and RAlt have the same scancode.
        // RAlt => 0x38,
        RMeta => 0x5C,
        Application => 0x5D,

        // LCtrl and RCtrl have the same scancode.
        // RCtrl =>0x1D,

        Insert => 0x52,
        Delete => 0x53,
        LeftArrow => 0x4B,
        Home => 0x47,
        End => 0x4F,
        UpArrow => 0x48,
        DownArrow => 0x50,
        PageUp => 0x49,
        PageDown => 0x51,
        RightArrow => 0x4D,

        // Numpad1 and End have the same scancode.
        // Numpad1 => 0x4F,
        // Numpad2 and DownArrow have the same scancode.
        // Numpad2 => 0x50,
        // Numpad3 and PageDown have the same scancode.
        // Numpad3 => 0x51,
        // Numpad4 and LeftArrow have the same scancode.
        // Numpad4 => 0x4B,
        Numpad5 => 0x4C,
        // Numpad6 and RightArrow have the same scancode.
        // Numpad6 => 0x4D,
        // Numpad7 and Home have the same scancode.
        // Numpad7 => 0x47,
        // Numpad8 and UpArrow have the same scancode.
        // Numpad8 => 0x48,
        // Numpad9 and PageUp have the same scancode.
        // Numpad9 => 0x49,

        // Numpad0 and Insert have the same sacncode.
        // Numpad0 => 0x52,

        // NumpadDot and Delete have the same scancode.
        // NumpadDot => 0x53,
        NumpadSlash => 0x35,
        NumpadAsterisk => 0x37,
        NumpadMinus => 0x4A,
        NumpadPlus => 0x4E,
        Esc => 0x01,
        F1=> 0x3B,
        F2=> 0x3C,
        F3=> 0x3D,
        F4=> 0x3E,
        F5=> 0x3F,
        F6=> 0x40,
        F7=> 0x41,
        F8=> 0x42,
        F9=> 0x43,
        F10 => 0x44,
        F11 => 0x57,
        F12 => 0x58,
        F13 => 0x64,
        F14 => 0x65,
        F15 => 0x66,
        F16 => 0x67,
        F17 => 0x68,
        F18 => 0x69,
        F19 => 0x6A,
        F20 => 0x6B,
        F21 => 0x6C,
        F22 => 0x6D,
        F23 => 0x6E,
        F24 => 0x76,
        PrintScreen => 0x2C,
    };
}