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

mod handler;
mod interface;
mod runtime;
mod utils;

pub use button::{ButtonInput, ButtonSet, ButtonState, ALT, CTRL, META, SHIFT};
pub use hookmap_core::{Button, ButtonAction, EmulateMouseCursor, EmulateMouseWheel, Mouse};
pub use interface::{Cond, ConditionalHook, Hook, SelectHandleTarget, SetEventBlock};
pub use runtime::interruption::Interruption;
pub use utils::Utils;

pub mod register {
    pub use super::interface::{
        ButtonEventHandlerEntry, MouseCursorEventHandlerEntry, MouseWheelEventHandlerEntry,
    };
}

pub mod interruption {
    pub use super::runtime::interruption::EventReceiver;
}
