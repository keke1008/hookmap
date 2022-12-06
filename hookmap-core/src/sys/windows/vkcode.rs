use crate::button::Button;

use windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY;

pub(super) const fn into_button(vkcode: VIRTUAL_KEY) -> Option<Button> {
    use windows::Win32::UI::Input::KeyboardAndMouse::*;
    use Button::*;

    Some(match vkcode {
        VK_LBUTTON => LeftButton,
        VK_RBUTTON => RightButton,
        VK_MBUTTON => MiddleButton,
        VK_XBUTTON1 => SideButton1,
        VK_XBUTTON2 => SideButton2,

        #[cfg(feature = "us-keyboard-layout")]
        VK_OEM_3 => Tilde,
        #[cfg(feature = "japanese-keyboard-layout")]
        VK_OEM_AUTO => HankakuZenkaku,

        VK_1 => Key1,
        VK_2 => Key2,
        VK_3 => Key3,
        VK_4 => Key4,
        VK_5 => Key5,
        VK_6 => Key6,
        VK_7 => Key7,
        VK_8 => Key8,
        VK_9 => Key9,
        VK_0 => Key0,
        VK_OEM_MINUS => Minus,

        #[cfg(feature = "us-keyboard-layout")]
        VK_OEM_PLUS => Equal,
        #[cfg(feature = "japanese-keyboard-layout")]
        VK_OEM_7 => Hat,

        #[cfg(feature = "japanese-keyboard-layout")]
        VK_OEM_5 => Yen,

        VK_BACK => Backspace,
        VK_TAB => Tab,
        VK_Q => Q,
        VK_W => W,
        VK_E => E,
        VK_R => R,
        VK_T => T,
        VK_Y => Y,
        VK_U => U,
        VK_I => I,
        VK_O => O,
        VK_P => P,

        #[cfg(feature = "us-keyboard-layout")]
        VK_OEM_4 => OpenSquareBracket,
        #[cfg(feature = "japanese-keyboard-layout")]
        VK_OEM_3 => At,

        #[cfg(feature = "us-keyboard-layout")]
        VK_OEM_6 => CloseSquareBracket,
        #[cfg(feature = "japanese-keyboard-layout")]
        VK_OEM_4 => OpenSquareBracket,

        #[cfg(feature = "us-keyboard-layout")]
        VK_CAPITAL => CapsLock,
        #[cfg(feature = "japanese-keyboard-layout")]
        VK_OEM_ATTN => Eisu,

        VK_A => A,
        VK_S => S,
        VK_D => D,
        VK_F => F,
        VK_G => G,
        VK_H => H,
        VK_J => J,
        VK_K => K,
        VK_L => L,

        #[cfg(feature = "us-keyboard-layout")]
        VK_OEM_1 => SemiColon,
        #[cfg(feature = "japanese-keyboard-layout")]
        VK_OEM_PLUS => SemiColon,

        #[cfg(feature = "us-keyboard-layout")]
        VK_OEM_7 => SingleQuote,
        #[cfg(feature = "japanese-keyboard-layout")]
        VK_OEM_1 => Colon,

        #[cfg(feature = "japanese-keyboard-layout")]
        VK_OEM_6 => CloseSquareBracket,

        VK_RETURN => Enter,
        VK_LSHIFT => LShift,
        VK_Z => Z,
        VK_X => X,
        VK_C => C,
        VK_V => V,
        VK_B => B,
        VK_N => N,
        VK_M => M,
        VK_OEM_COMMA => Comma,
        VK_OEM_PERIOD => Dot,
        VK_OEM_2 => Slash,

        #[cfg(feature = "japanese-keyboard-layout")]
        VK_OEM_102 => BackSlash,

        VK_RSHIFT => RShift,
        VK_LCONTROL => LCtrl,
        VK_LWIN => LSuper,
        VK_LMENU => LAlt,

        #[cfg(feature = "japanese-keyboard-layout")]
        VK_NONCONVERT => Muhenkan,

        VK_SPACE => Space,

        #[cfg(feature = "japanese-keyboard-layout")]
        VK_CONVERT => Henkan,

        #[cfg(feature = "japanese-keyboard-layout")]
        VK_OEM_COPY => KatakanaHiragana,

        VK_RMENU => RAlt,
        VK_RWIN => RSuper,
        VK_APPS => Application,
        VK_RCONTROL => RCtrl,
        VK_INSERT => Insert,
        VK_DELETE => Delete,
        VK_LEFT => LeftArrow,
        VK_HOME => Home,
        VK_END => End,
        VK_UP => UpArrow,
        VK_DOWN => DownArrow,
        VK_PRIOR => PageUp,
        VK_NEXT => PageDown,
        VK_RIGHT => RightArrow,
        VK_NUMPAD1 => Numpad1,
        VK_NUMPAD2 => Numpad2,
        VK_NUMPAD3 => Numpad3,
        VK_NUMPAD4 => Numpad4,
        VK_NUMPAD5 => Numpad5,
        VK_NUMPAD6 => Numpad6,
        VK_NUMPAD7 => Numpad7,
        VK_NUMPAD8 => Numpad8,
        VK_NUMPAD9 => Numpad9,
        VK_NUMPAD0 => Numpad0,
        VK_DECIMAL => NumpadDot,
        VK_DIVIDE => NumpadSlash,
        VK_MULTIPLY => NumpadAsterisk,
        VK_SUBTRACT => NumpadMinus,
        VK_ADD => NumpadPlus,
        VK_ESCAPE => Esc,
        VK_F1 => F1,
        VK_F2 => F2,
        VK_F3 => F3,
        VK_F4 => F4,
        VK_F5 => F5,
        VK_F6 => F6,
        VK_F7 => F7,
        VK_F8 => F8,
        VK_F9 => F9,
        VK_F10 => F10,
        VK_F11 => F11,
        VK_F12 => F12,
        VK_F13 => F13,
        VK_F14 => F14,
        VK_F15 => F15,
        VK_F16 => F16,
        VK_F17 => F17,
        VK_F18 => F18,
        VK_F19 => F19,
        VK_F20 => F20,
        VK_F21 => F21,
        VK_F22 => F22,
        VK_F23 => F23,
        VK_F24 => F24,
        VK_SNAPSHOT => PrintScreen,

        VK_VOLUME_MUTE => VolumeMute,
        VK_VOLUME_DOWN => VolumeDown,
        VK_VOLUME_UP => VolumeUp,
        VK_MEDIA_NEXT_TRACK => MediaNext,
        VK_MEDIA_PREV_TRACK => MediaPrevious,
        VK_MEDIA_STOP => MediaStop,
        VK_MEDIA_PLAY_PAUSE => MediaPlayPause,

        _ => return None,
    })
}

pub(super) const fn from_button(button: Button) -> VIRTUAL_KEY {
    use windows::Win32::UI::Input::KeyboardAndMouse::*;
    use Button::*;

    match button {
        LeftButton => VK_LBUTTON,
        RightButton => VK_RBUTTON,
        MiddleButton => VK_MBUTTON,
        SideButton1 => VK_XBUTTON1,
        SideButton2 => VK_XBUTTON2,

        #[cfg(feature = "us-keyboard-layout")]
        Tilde => VK_OEM_3,
        #[cfg(feature = "japanese-keyboard-layout")]
        HankakuZenkaku => VK_OEM_AUTO,

        Key1 => VK_1,
        Key2 => VK_2,
        Key3 => VK_3,
        Key4 => VK_4,
        Key5 => VK_5,
        Key6 => VK_6,
        Key7 => VK_7,
        Key8 => VK_8,
        Key9 => VK_9,
        Key0 => VK_0,
        Minus => VK_OEM_MINUS,

        #[cfg(feature = "us-keyboard-layout")]
        Equal => VK_OEM_PLUS,
        #[cfg(feature = "japanese-keyboard-layout")]
        Hat => VK_OEM_7,

        #[cfg(feature = "japanese-keyboard-layout")]
        Yen => VK_OEM_5,

        Backspace => VK_BACK,
        Tab => VK_TAB,
        Q => VK_Q,
        W => VK_W,
        E => VK_E,
        R => VK_R,
        T => VK_T,
        Y => VK_Y,
        U => VK_U,
        I => VK_I,
        O => VK_O,
        P => VK_P,

        #[cfg(feature = "us-keyboard-layout")]
        OpenSquareBracket => VK_OEM_4,
        #[cfg(feature = "japanese-keyboard-layout")]
        At => VK_OEM_3,

        #[cfg(feature = "us-keyboard-layout")]
        CloseSquareBracket => VK_OEM_6,
        #[cfg(feature = "japanese-keyboard-layout")]
        OpenSquareBracket => VK_OEM_4,

        #[cfg(feature = "us-keyboard-layout")]
        CapsLock => VK_CAPITAL,
        #[cfg(feature = "japanese-keyboard-layout")]
        Eisu => VK_OEM_ATTN,

        A => VK_A,
        S => VK_S,
        D => VK_D,
        F => VK_F,
        G => VK_G,
        H => VK_H,
        J => VK_J,
        K => VK_K,
        L => VK_L,

        #[cfg(feature = "us-keyboard-layout")]
        SemiColon => VK_OEM_1,
        #[cfg(feature = "japanese-keyboard-layout")]
        SemiColon => VK_OEM_PLUS,

        #[cfg(feature = "us-keyboard-layout")]
        SingleQuote => VK_OEM_7,
        #[cfg(feature = "japanese-keyboard-layout")]
        Colon => VK_OEM_1,

        #[cfg(feature = "japanese-keyboard-layout")]
        CloseSquareBracket => VK_OEM_6,

        Enter => VK_RETURN,
        LShift => VK_LSHIFT,
        Z => VK_Z,
        X => VK_X,
        C => VK_C,
        V => VK_V,
        B => VK_B,
        N => VK_N,
        M => VK_M,
        Comma => VK_OEM_COMMA,
        Dot => VK_OEM_PERIOD,
        Slash => VK_OEM_2,

        #[cfg(feature = "japanese-keyboard-layout")]
        BackSlash => VK_OEM_102,

        RShift => VK_RSHIFT,
        LCtrl => VK_LCONTROL,
        LSuper => VK_LWIN,
        LAlt => VK_LMENU,

        #[cfg(feature = "japanese-keyboard-layout")]
        Muhenkan => VK_NONCONVERT,

        Space => VK_SPACE,

        #[cfg(feature = "japanese-keyboard-layout")]
        Henkan => VK_CONVERT,

        #[cfg(feature = "japanese-keyboard-layout")]
        KatakanaHiragana => VK_OEM_COPY,

        RAlt => VK_RMENU,
        RSuper => VK_RWIN,
        Application => VK_APPS,
        RCtrl => VK_RCONTROL,
        Insert => VK_RCONTROL,
        Delete => VK_DELETE,
        LeftArrow => VK_LEFT,
        Home => VK_HOME,
        End => VK_END,
        UpArrow => VK_UP,
        DownArrow => VK_DOWN,
        PageUp => VK_PRIOR,
        PageDown => VK_NEXT,
        RightArrow => VK_RIGHT,
        Numpad1 => VK_NUMPAD1,
        Numpad2 => VK_NUMPAD2,
        Numpad3 => VK_NUMPAD3,
        Numpad4 => VK_NUMPAD4,
        Numpad5 => VK_NUMPAD5,
        Numpad6 => VK_NUMPAD6,
        Numpad7 => VK_NUMPAD7,
        Numpad8 => VK_NUMPAD8,
        Numpad9 => VK_NUMPAD9,
        Numpad0 => VK_NUMPAD0,
        NumpadDot => VK_DECIMAL,
        NumpadSlash => VK_DIVIDE,
        NumpadAsterisk => VK_MULTIPLY,
        NumpadMinus => VK_SUBTRACT,
        NumpadPlus => VK_ADD,
        Esc => VK_ESCAPE,
        F1 => VK_F1,
        F2 => VK_F2,
        F3 => VK_F3,
        F4 => VK_F4,
        F5 => VK_F5,
        F6 => VK_F6,
        F7 => VK_F7,
        F8 => VK_F8,
        F9 => VK_F9,
        F10 => VK_F10,
        F11 => VK_F11,
        F12 => VK_F12,
        F13 => VK_F13,
        F14 => VK_F14,
        F15 => VK_F15,
        F16 => VK_F16,
        F17 => VK_F17,
        F18 => VK_F18,
        F19 => VK_F19,
        F20 => VK_F20,
        F21 => VK_F21,
        F22 => VK_F22,
        F23 => VK_F23,
        F24 => VK_F24,
        PrintScreen => VK_SNAPSHOT,

        VolumeMute => VK_VOLUME_MUTE,
        VolumeDown => VK_VOLUME_DOWN,
        VolumeUp => VK_VOLUME_UP,
        MediaNext => VK_MEDIA_NEXT_TRACK,
        MediaPrevious => VK_MEDIA_PREV_TRACK,
        MediaStop => VK_MEDIA_STOP,
        MediaPlayPause => VK_MEDIA_PLAY_PAUSE,

        Shift | Ctrl | Alt | Super => unreachable!(),
    }
}
