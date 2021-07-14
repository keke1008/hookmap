pub mod common;
mod macros;

#[cfg(target_os = "windows")]
mod windows;

pub use common::{
    event::{Event, EventBlock},
    handler::{HandlerFunction, InputHandler, INPUT_HANDLER},
    keyboard::{EmulateKeyboardInput, Key, KeyboardAction, KeyboardEvent},
    mouse::{EmulateMouseInput, MouseAction, MouseEvent, MouseInput},
};
