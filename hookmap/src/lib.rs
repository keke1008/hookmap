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
pub mod macros;

mod hotkey;
mod interface;
mod runtime;
// mod utils;

pub use button::{ButtonInput, ButtonState};
pub use hookmap_core::{Button, ButtonAction, EmulateMouseCursor, EmulateMouseWheel, Mouse};
pub use interface::{ConditionalHook, Hotkey, SelectHandleTarget, SetEventBlock};
pub use runtime::interruption::Interruption;
// pub use utils::Utils;

pub mod register {
    pub use super::interface::{
        ButtonEventHandlerEntry, MouseCursorHotKeyEntry, MouseWheelHotkeyEntry,
    };
}

pub mod interruption {
    pub use super::runtime::interruption::EventReceiver;
}
