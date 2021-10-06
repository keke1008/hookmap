use winapi::um::winuser::KBDLLHOOKSTRUCT;

use crate::common::button::Button;

impl Button {
    pub(super) fn from_hook_struct(hook: &KBDLLHOOKSTRUCT) -> Option<Self> {
        let button = *CODE_TO_BUTTON.get(hook.scanCode as usize)?;
        match button {
            Some(Button::LCtrl) if hook.flags & 1 != 0 => Some(Button::RCtrl),
            Some(Button::RAlt) if hook.flags & 1 != 0 => Some(Button::RAlt),
            _ => button,
        }
    }

    pub(super) fn to_scancode_and_flag(self) -> Option<(u32, u32)> {
        const CTRL_KEY_SCANCODE: u32 = 0x1D;
        const ALT_KEY_SCANCODE: u32 = 0x38;
        match self {
            Button::RCtrl => Some((CTRL_KEY_SCANCODE, 0x1)),
            Button::RAlt => Some((ALT_KEY_SCANCODE, 0x1)),
            _ => Some((*BUTTON_TO_CODE.get(self as usize)?, 0x0)),
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
                {
                    map[Button::$button as usize] = $code;
                }
             )*
            map
        };
        const _MAX_CODE_VALUE: usize = {
            button_code_map!(@max $($code),*) + 1
        };
        const $code_to_button: [Option<Button>; _MAX_CODE_VALUE] = {
            let mut map = [None; _MAX_CODE_VALUE];
            $(
                $(#[$att])?
                {
                    map[$code] = Some(Button::$button);
                }
             )*
            map
        };
    };
}

button_code_map! {
    const (BUTTON_TO_CODE, CODE_TO_BUTTON) = {

        #[cfg(feature="us_keyboard_layout")]
        Tilde => 0x29,
        #[cfg(feature="japanese_keyboard_layout")]
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

        #[cfg(feature="us_keyboard_layout")]
        Equal => 0x0D,
        #[cfg(feature="japanese_keyboard_layout")]
        Hat => 0x0D,

        #[cfg(feature="japanese_keyboard_layout")]
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

        #[cfg(feature = "us_keyboard_layout")]
        OpenSquareBracket => 0x1A,
        #[cfg(feature = "japanese_keyboard_layout")]
        At => 0x1A,

        #[cfg(feature = "us_keyboard_layout")]
        CloseSquareBracket => 0x1B,
        #[cfg(feature = "japanese_keyboard_layout")]
        OpenSquareBracket => 0x1B,

        #[cfg(feature = "us_keyboard_layout")]
        CapsLock => 0x3A,
        #[cfg(feature = "japanese_keyboard_layout")]
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

        #[cfg(feature = "us_keyboard_layout")]
        SingleQuote => 0x28,
        #[cfg(feature = "japanese_keyboard_layout")]
        Colon => 0x28,

        #[cfg(feature = "japanese_keyboard_layout")]
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

        #[cfg(feature = "japanese_keyboard_layout")]
        BackSlash => 0x73,

        RShift => 0x36,
        LCtrl => 0x1D,
        LMeta => 0x5B,
        LAlt => 0x38,

        #[cfg(feature = "japanese_keyboard_layout")]
        Muhenkan => 0x7B,

        Space => 0x39,

        #[cfg(feature = "japanese_keyboard_layout")]
        Henkan => 0x79,

        #[cfg(feature = "japanese_keyboard_layout")]
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
        Numpad1 => 0x4F,
        Numpad2 => 0x62,
        Numpad3 => 0x63,
        Numpad4 => 0x4B,
        Numpad5 => 0x65,
        Numpad6 => 0x66,
        Numpad7 => 0x47,
        Numpad8 => 0x48,
        Numpad9 => 0x49,
        Numpad0 => 0x52,
        NumpadDot => 0x53,
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
