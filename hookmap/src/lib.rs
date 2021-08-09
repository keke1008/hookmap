//! # Hookmap
//!
//! Register hotkeys and emulate input.
//!
//! ## Handler order.
//!
//! Handlers are called in the order in which they are registered.
//!
//! ## Block input event
//!
//! If you want to block the button event, call [`Button::block_input`]. otherwise, call [`Button::unblock_input`].
//!
//! ## Feature flags
//!
//! * `block-input-event`: Set button events to be blocked by default.

pub mod interface;

mod handler;
mod modifier;
mod runtime;

pub use hookmap_core::{
    Button, ButtonAction, ButtonInput, ButtonState, EmulateMouseCursor, EmulateMouseWheel,
};
pub use interface::{Hook, Modifier, SelectHandleTarget};
pub use runtime::interruption;
