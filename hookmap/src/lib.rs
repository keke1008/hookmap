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

pub mod cond;
pub mod interface;

mod button;
mod handler;
mod modifier;
mod runtime;

pub use button::{All, Any, ButtonSet};
pub use cond::Cond;
pub use hookmap_core::{
    Button, ButtonAction, ButtonInput, ButtonState, EmulateMouseCursor, EmulateMouseWheel,
};
pub use interface::{Hook, Modifier, SelectHandleTarget};
pub use runtime::interruption;
