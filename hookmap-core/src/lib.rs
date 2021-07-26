pub mod common;
mod macros;

#[cfg(target_os = "windows")]
mod windows;

pub use common::{
    event::{Event, EventBlock},
    handler::{HandlerFunction, InputHandler, INPUT_HANDLER},
    keyboard::{Key, KeyboardEvent},
    mouse::{EmulateMouseCursor, EmulateMouseWheel, Mouse, MouseCursor, MouseEvent, MouseWheel},
    ButtonAction, EmulateButtonInput,
};
