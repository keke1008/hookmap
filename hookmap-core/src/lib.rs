pub mod common;
mod macros;

#[cfg(target_os = "windows")]
mod windows;

pub use common::{event::EventBlock, handler::INPUT_HANDLER};

pub mod keyboard {
    pub use super::common::keyboard::{EmulateKeyboardInput, Key, KeyboardAction, KeyboardEvent};
}

pub mod mouse {
    pub use super::common::mouse::{EmulateMouseInput, MouseAction, MouseEvent, MouseInput};
}
