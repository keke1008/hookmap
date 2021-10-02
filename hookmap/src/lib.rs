//! Register hotkeys and emulate input.
//!
//! ## Handler order.
//!
//! Handlers are called in the order in which they are registered.
//!
//! ## Feature flags
//!
//! * `block-input-event`: Set button events to be blocked by default.

pub mod button;
pub mod interface;
pub mod macros;

mod hotkey;
mod runtime;
mod utils;

pub use button::{ALT, CTRL, META, SHIFT};
pub use hookmap_core::{Button, ButtonAction, EmulateMouseCursor, EmulateMouseWheel, Mouse};
pub use interface::{Hotkey, SelectHandleTarget, SetEventBlock};
pub use runtime::hook;
pub use utils::Utils;

pub mod register {
    pub use super::interface::{
        ButtonEventHandlerEntry, MouseCursorHotKeyEntry, MouseWheelHotkeyEntry,
    };
}
