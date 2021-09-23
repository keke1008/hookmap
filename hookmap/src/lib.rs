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
pub mod hotkey;
pub mod macros;

mod interface;
mod runtime;
mod storage;
mod utils;

pub use button::{ButtonInput, ButtonSet, ButtonState, ALT, CTRL, META, SHIFT};
pub use hookmap_core::{Button, ButtonAction, EmulateMouseCursor, EmulateMouseWheel, Mouse};
pub use hotkey::ConditionUnit;
pub use interface::{ConditionalHook, Hook, SelectHandleTarget, SetEventBlock};
pub use runtime::interruption::Interruption;
pub use utils::Utils;

pub mod register {
    pub use super::interface::{
        ButtonEventHandlerEntry, MouseCursorHotKeyEntry, MouseWheelHotkeyEntry,
    };
}

pub mod interruption {
    pub use super::runtime::interruption::EventReceiver;
}
