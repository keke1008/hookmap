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

mod hook;
mod runtime;
// mod utils;

pub use hookmap_core::{Button, ButtonAction, EmulateMouseCursor, EmulateMouseWheel, Mouse};
pub use runtime::interceptor;
// pub use utils::Utils;
