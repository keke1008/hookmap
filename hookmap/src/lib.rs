//! # Hookmap
//!
//! Register hotkeys and emulate input.
//!
//! ## Handler order.
//!
//! Handlers are called in the order in which they are registered.
//!
//! ## Feature flags
//!
//! * `block-input-event`: Set button events to be blocked by default.

mod button;
mod cond;
mod handler;
mod interface;
mod runtime;

pub use button::{All, Any, ButtonSet};
pub use cond::Cond;
pub use hookmap_core::{
    Button, ButtonAction, ButtonInput, ButtonState, EmulateMouseCursor, EmulateMouseWheel,
};
pub use interface::{ConditionalHook, Hook, SelectHandleTarget};
pub use runtime::interruption;
