pub mod common;
mod macros;

#[cfg(target_os = "windows")]
mod windows;

pub mod keyboard {
    pub use super::common::keyboard::{
        EmulateKeyboardInput, KeyboardAction, KeyboardEvent, KeyboardKey, KEYBOARD_EVENT_HANDLER,
    };
}
