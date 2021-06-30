pub mod common;
mod macros;

#[cfg(target_os = "windows")]
mod windows;

pub mod keyboard {
    pub use super::common::keyboard::{
        EmulateKeyboardInput, Key, KeyboardAction, KeyboardEvent, KEYBOARD_HANDLER,
    };
}

pub mod mouse {
    pub use super::common::mouse::{
        EmulateMouseInput, MouseAction, MouseEvent, MouseInput, MOUSE_EVENT_HANDLER,
    };
}
