use crate::common::button::Button;

pub(super) const fn into_button(vkcode: u32) -> Option<Button> {
    use Button::*;
    Some(match vkcode {
        0x01 => LeftButton,
        0x02 => RightButton,
        0x04 => MiddleButton,
        0x05 => SideButton1,
        0x06 => SideButton2,

        #[cfg(feature = "us-keyboard-layout")]
        0xC0 => Tilde,
        #[cfg(feature = "japanese-keyboard-layout")]
        0xC0 => HankakuZenkaku,

        0x31 => Key1,
        0x32 => Key2,
        0x33 => Key3,
        0x34 => Key4,
        0x35 => Key5,
        0x36 => Key6,
        0x37 => Key7,
        0x38 => Key8,
        0x39 => Key9,
        0x30 => Key0,
        0xBD => Minus,

        #[cfg(feature = "us-keyboard-layout")]
        0xBB => Equal,
        #[cfg(feature = "japanese-keyboard-layout")]
        0xDE => Hat,

        #[cfg(feature = "japanese-keyboard-layout")]
        0xDC => Yen,

        0x08 => Backspace,
        0x09 => Tab,
        0x51 => Q,
        0x57 => W,
        0x45 => E,
        0x52 => R,
        0x54 => T,
        0x59 => Y,
        0x55 => U,
        0x49 => I,
        0x4F => O,
        0x50 => P,

        #[cfg(feature = "us-keyboard-layout")]
        0xDB => OpenSquareBracket,
        #[cfg(feature = "japanese-keyboard-layout")]
        0xC0 => At,

        #[cfg(feature = "us-keyboard-layout")]
        0xDD => CloseSquareBracket,
        #[cfg(feature = "japanese-keyboard-layout")]
        0xDB => OpenSquareBracket,

        #[cfg(feature = "us-keyboard-layout")]
        0x14 => CapsLock,
        #[cfg(feature = "japanese-keyboard-layout")]
        0xF0 => Eisu,

        0x41 => A,
        0x53 => S,
        0x44 => D,
        0x46 => F,
        0x47 => G,
        0x48 => H,
        0x4A => J,
        0x4B => K,
        0x4C => L,

        #[cfg(feature = "us-keyboard-layout")]
        0xBA => SemiColon,
        #[cfg(feature = "japanese-keyboard-layout")]
        0xBB => SemiColon,

        #[cfg(feature = "us-keyboard-layout")]
        0xDE => SingleQuote,
        #[cfg(feature = "japanese-keyboard-layout")]
        0xBA => Colon,

        #[cfg(feature = "japanese-keyboard-layout")]
        0xDD => CloseSquareBracket,

        0x0D => Enter,
        0xA0 => LShift,
        0x5A => Z,
        0x58 => X,
        0x43 => C,
        0x56 => V,
        0x42 => B,
        0x4E => N,
        0x4D => M,
        0xBC => Comma,
        0xBE => Dot,
        0xBF => Slash,

        #[cfg(feature = "japanese-keyboard-layout")]
        0xE2 => BackSlash,

        0xA1 => RShift,
        0xA2 => LCtrl,
        0x5B => LMeta,
        0xA4 => LAlt,

        #[cfg(feature = "japanese-keyboard-layout")]
        0x1D => Muhenkan,

        0x20 => Space,

        #[cfg(feature = "japanese-keyboard-layout")]
        0x1C => Henkan,

        #[cfg(feature = "japanese-keyboard-layout")]
        0xF2 => KatakanaHiragana,

        0xA5 => RAlt,
        0x5C => RMeta,
        0x5D => Application,
        0xA3 => RCtrl,
        0x2D => Insert,
        0x2E => Delete,
        0x25 => LeftArrow,
        0x24 => Home,
        0x23 => End,
        0x26 => UpArrow,
        0x28 => DownArrow,
        0x21 => PageUp,
        0x22 => PageDown,
        0x27 => RightArrow,
        0x61 => Numpad1,
        0x62 => Numpad2,
        0x63 => Numpad3,
        0x64 => Numpad4,
        0x65 => Numpad5,
        0x66 => Numpad6,
        0x67 => Numpad7,
        0x68 => Numpad8,
        0x69 => Numpad9,
        0x60 => Numpad0,
        0x6E => NumpadDot,
        0x6F => NumpadSlash,
        0x6A => NumpadAsterisk,
        0x6D => NumpadMinus,
        0x6B => NumpadPlus,
        0x1B => Esc,
        0x70 => F1,
        0x71 => F2,
        0x72 => F3,
        0x73 => F4,
        0x74 => F5,
        0x75 => F6,
        0x76 => F7,
        0x77 => F8,
        0x78 => F9,
        0x79 => F10,
        0x7A => F11,
        0x7B => F12,
        0x7C => F13,
        0x7D => F14,
        0x7E => F15,
        0x7F => F16,
        0x80 => F17,
        0x81 => F18,
        0x82 => F19,
        0x83 => F20,
        0x84 => F21,
        0x85 => F22,
        0x86 => F23,
        0x87 => F24,
        0x2C => PrintScreen,
        _ => return None,
    })
}

pub(super) const fn from_button(button: Button) -> u16 {
    use Button::*;
    match button {
        LeftButton => 0x01,
        RightButton => 0x02,
        MiddleButton => 0x04,
        SideButton1 => 0x05,
        SideButton2 => 0x06,

        #[cfg(feature = "us-keyboard-layout")]
        Tilde => 0xC0,
        #[cfg(feature = "japanese-keyboard-layout")]
        HankakuZenkaku => 0xC0,

        Key1 => 0x31,
        Key2 => 0x32,
        Key3 => 0x33,
        Key4 => 0x34,
        Key5 => 0x35,
        Key6 => 0x36,
        Key7 => 0x37,
        Key8 => 0x38,
        Key9 => 0x39,
        Key0 => 0x30,
        Minus => 0xBD,

        #[cfg(feature = "us-keyboard-layout")]
        Equal => 0xBB,
        #[cfg(feature = "japanese-keyboard-layout")]
        Hat => 0xDE,

        #[cfg(feature = "japanese-keyboard-layout")]
        Yen => 0xDC,

        Backspace => 0x08,
        Tab => 0x09,
        Q => 0x51,
        W => 0x57,
        E => 0x45,
        R => 0x52,
        T => 0x54,
        Y => 0x59,
        U => 0x55,
        I => 0x49,
        O => 0x4F,
        P => 0x50,

        #[cfg(feature = "us-keyboard-layout")]
        OpenSquareBracket => 0xDB,
        #[cfg(feature = "japanese-keyboard-layout")]
        At => 0xC0,

        #[cfg(feature = "us-keyboard-layout")]
        CloseSquareBracket => 0xDD,
        #[cfg(feature = "japanese-keyboard-layout")]
        OpenSquareBracket => 0xDB,

        #[cfg(feature = "us-keyboard-layout")]
        CapsLock => 0x14,
        #[cfg(feature = "japanese-keyboard-layout")]
        Eisu => 0xF0,

        A => 0x41,
        S => 0x53,
        D => 0x44,
        F => 0x46,
        G => 0x47,
        H => 0x48,
        J => 0x4A,
        K => 0x4B,
        L => 0x4C,

        #[cfg(feature = "us-keyboard-layout")]
        SemiColon => 0xBA,
        #[cfg(feature = "japanese-keyboard-layout")]
        SemiColon => 0xBB,

        #[cfg(feature = "us-keyboard-layout")]
        SingleQuote => 0xDE,
        #[cfg(feature = "japanese-keyboard-layout")]
        Colon => 0xBA,

        #[cfg(feature = "japanese-keyboard-layout")]
        CloseSquareBracket => 0xDD,

        Enter => 0x0D,
        LShift => 0xA0,
        Z => 0x5A,
        X => 0x58,
        C => 0x43,
        V => 0x56,
        B => 0x42,
        N => 0x4E,
        M => 0x4D,
        Comma => 0xBC,
        Dot => 0xBE,
        Slash => 0xBF,

        #[cfg(feature = "japanese-keyboard-layout")]
        BackSlash => 0xE2,

        RShift => 0xA1,
        LCtrl => 0xA2,
        LMeta => 0x5B,
        LAlt => 0xA4,

        #[cfg(feature = "japanese-keyboard-layout")]
        Muhenkan => 0x1D,

        Space => 0x20,

        #[cfg(feature = "japanese-keyboard-layout")]
        Henkan => 0x1C,

        #[cfg(feature = "japanese-keyboard-layout")]
        KatakanaHiragana => 0xF2,

        RAlt => 0xA5,
        RMeta => 0x5C,
        Application => 0x5D,
        RCtrl => 0xA3,
        Insert => 0x2D,
        Delete => 0x2E,
        LeftArrow => 0x25,
        Home => 0x24,
        End => 0x23,
        UpArrow => 0x26,
        DownArrow => 0x28,
        PageUp => 0x21,
        PageDown => 0x22,
        RightArrow => 0x27,
        Numpad1 => 0x61,
        Numpad2 => 0x62,
        Numpad3 => 0x63,
        Numpad4 => 0x64,
        Numpad5 => 0x65,
        Numpad6 => 0x66,
        Numpad7 => 0x67,
        Numpad8 => 0x68,
        Numpad9 => 0x69,
        Numpad0 => 0x60,
        NumpadDot => 0x6E,
        NumpadSlash => 0x6F,
        NumpadAsterisk => 0x6A,
        NumpadMinus => 0x6D,
        NumpadPlus => 0x6B,
        Esc => 0x1B,
        F1 => 0x70,
        F2 => 0x71,
        F3 => 0x72,
        F4 => 0x73,
        F5 => 0x74,
        F6 => 0x75,
        F7 => 0x76,
        F8 => 0x77,
        F9 => 0x78,
        F10 => 0x79,
        F11 => 0x7A,
        F12 => 0x7B,
        F13 => 0x7C,
        F14 => 0x7D,
        F15 => 0x7E,
        F16 => 0x7F,
        F17 => 0x80,
        F18 => 0x81,
        F19 => 0x82,
        F20 => 0x83,
        F21 => 0x84,
        F22 => 0x85,
        F23 => 0x86,
        F24 => 0x87,
        PrintScreen => 0x2C,
    }
}
