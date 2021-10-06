use variant_count::VariantCount;

pub trait ButtonOperation {
    fn generate_press_event(self, recursive: bool);
    fn generate_release_event(self, recursive: bool);
    fn read_is_pressed(self) -> bool;
}

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

    #[cfg(feature = "us_keyboard_layout")]
    Tilde,
    #[cfg(feature = "japanese_keyboard_layout")]
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

    #[cfg(feature = "us_keyboard_layout")]
    Equal,
    #[cfg(feature = "japanese_keyboard_layout")]
    Hat,

    #[cfg(feature = "japanese_keyboard_layout")]
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

    #[cfg(feature = "us_keyboard_layout")]
    OpenSquareBracket,
    #[cfg(feature = "japanese_keyboard_layout")]
    At,

    #[cfg(feature = "us_keyboard_layout")]
    CloseSquareBracket,
    #[cfg(feature = "japanese_keyboard_layout")]
    OpenSquareBracket,

    #[cfg(feature = "us_keyboard_layout")]
    CapsLock,
    #[cfg(feature = "japanese_keyboard_layout")]
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
    SemiColon,

    #[cfg(feature = "us_keyboard_layout")]
    SingleQuote,
    #[cfg(feature = "japanese_keyboard_layout")]
    Colon,

    #[cfg(feature = "japanese_keyboard_layout")]
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

    #[cfg(feature = "japanese_keyboard_layout")]
    BackSlash,

    RShift,
    LCtrl,
    LMeta,
    LAlt,

    #[cfg(feature = "japanese_keyboard_layout")]
    Muhenkan,

    Space,

    #[cfg(feature = "japanese_keyboard_layout")]
    Henkan,

    #[cfg(feature = "japanese_keyboard_layout")]
    KatakanaHiragana,

    RAlt,
    RMeta,
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

    Void,

    Unassigned1,
    Unassigned2,
    Unassigned3,
    Unassigned4,
    Unassigned5,
    Unassigned6,
    Unassigned7,
    Unassigned8,
    Unassigned9,
    Unassigned10,
    Unassigned11,
    Unassigned12,
    Unassigned13,
    Unassigned14,
    Unassigned15,
    Unassigned16,
    Unassigned17,
    Unassigned18,
    Unassigned19,
    Unassigned20,
    Unassigned21,
    Unassigned22,
    Unassigned23,
    Unassigned24,
    Unassigned25,
    Unassigned26,
    Unassigned27,
    Unassigned28,
    Unassigned29,
    Unassigned30,
    Unassigned31,
    Unassigned32,
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
