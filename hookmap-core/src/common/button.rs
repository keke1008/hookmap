pub trait InstallButtonHook {
    fn install() {}
}

pub trait ButtonInput {
    /// Emulates a button press operation.
    fn press(&self);

    /// Emulates a button release operation.
    fn release(&self);

    /// Presses a button and releases it immediately.
    fn click(&self) {
        self.press();
        self.release();
    }
}

pub trait ButtonState {
    /// Returns `true` if a button is pressed.
    fn is_pressed(&self) -> bool;
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

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Button {
    LeftButton,
    RightButton,
    MiddleButton,
    SideButton1,
    SideButton2,
    Backspace,
    Tab,
    Enter,
    Shift,
    Ctrl,
    Alt,
    CapsLock,
    Esc,
    Henkan,
    Muhenkan,
    Space,
    PageUp,
    PageDown,
    End,
    Home,
    LeftArrow,
    UpArrow,
    RightArrow,
    DownArrow,
    PrintScreen,
    Insert,
    Delete,
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    LMeta,
    RMeta,
    Application,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadAsterisk,
    NumpadPlus,
    NumpadMinus,
    NumpadDot,
    NumpadSlash,
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
    Numlock,
    ScrollLock,
    LShift,
    RShift,
    LCtrl,
    RCtrl,
    LAlt,
    RAlt,
    Colon,
    SemiColon,
    Comma,
    Minus,
    Dot,
    Slash,
    At,
    LeftSquareBracket,
    BackSlashWithVerticalBar,
    RightSquareBracket,
    Hat,
    BackSlashWithUnderLine,
    Eisuu,
    KatakanaHiragana,
    HannkakuZenkaku,
    OtherKey(u32),
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
