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
pub mod utils;

mod hook;
mod runtime;

pub mod event {
    pub use hookmap_core::{ButtonEvent, MouseCursorEvent, MouseWheelEvent, NativeEventOperation};
}

pub mod mouse {
    pub use hookmap_core::{EmulateMouseCursor, EmulateMouseWheel, Mouse};
}

pub use runtime::interceptor;
