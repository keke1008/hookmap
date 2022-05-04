use variant_count::VariantCount;

/// A button input action.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum ButtonAction {
    Press,
    Release,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum ButtonKind {
    Key,
    Mouse,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, VariantCount)]
pub enum Button {
    LeftButton,
    RightButton,
    MiddleButton,
    SideButton1,
    SideButton2,

    #[cfg(feature = "us-keyboard-layout")]
    Tilde,
    #[cfg(feature = "japanese-keyboard-layout")]
    HankakuZenkaku,

    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key0,
    Minus,

    #[cfg(feature = "us-keyboard-layout")]
    Equal,
    #[cfg(feature = "japanese-keyboard-layout")]
    Hat,

    #[cfg(feature = "japanese-keyboard-layout")]
    Yen,

    Backspace,
    Tab,
    Q,
    W,
    E,
    R,
    T,
    Y,
    U,
    I,
    O,
    P,

    #[cfg(feature = "us-keyboard-layout")]
    OpenSquareBracket,
    #[cfg(feature = "japanese-keyboard-layout")]
    At,

    #[cfg(feature = "us-keyboard-layout")]
    CloseSquareBracket,
    #[cfg(feature = "japanese-keyboard-layout")]
    OpenSquareBracket,

    #[cfg(feature = "us-keyboard-layout")]
    CapsLock,
    #[cfg(feature = "japanese-keyboard-layout")]
    Eisu,

    A,
    S,
    D,
    F,
    G,
    H,
    J,
    K,
    L,

    #[cfg(feature = "us-keyboard-layout")]
    SemiColon,
    #[cfg(feature = "japanese-keyboard-layout")]
    SemiColon,

    #[cfg(feature = "us-keyboard-layout")]
    SingleQuote,
    #[cfg(feature = "japanese-keyboard-layout")]
    Colon,

    #[cfg(feature = "japanese-keyboard-layout")]
    CloseSquareBracket,

    Enter,
    LShift,
    Z,
    X,
    C,
    V,
    B,
    N,
    M,
    Comma,
    Dot,
    Slash,

    #[cfg(feature = "japanese-keyboard-layout")]
    BackSlash,

    RShift,
    LCtrl,
    LSuper,
    LAlt,

    #[cfg(feature = "japanese-keyboard-layout")]
    Muhenkan,

    Space,

    #[cfg(feature = "japanese-keyboard-layout")]
    Henkan,

    #[cfg(feature = "japanese-keyboard-layout")]
    KatakanaHiragana,

    RAlt,
    RSuper,
    Application,
    RCtrl,
    Insert,
    Delete,
    LeftArrow,
    Home,
    End,
    UpArrow,
    DownArrow,
    PageUp,
    PageDown,
    RightArrow,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    Numpad0,
    NumpadDot,
    NumpadSlash,
    NumpadAsterisk,
    NumpadMinus,
    NumpadPlus,
    Esc,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    PrintScreen,

    Shift,
    Ctrl,
    Alt,
    Super,
}

impl Button {
    pub fn kind(&self) -> ButtonKind {
        match self {
            Button::LeftButton
            | Button::RightButton
            | Button::MiddleButton
            | Button::SideButton1
            | Button::SideButton2 => ButtonKind::Mouse,
            _ => ButtonKind::Key,
        }
    }
}
