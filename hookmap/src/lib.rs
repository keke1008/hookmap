//! # Hookmap
//!
//! Register hotkeys and emulate input.

pub mod event;
pub mod interface;

mod handler;
mod modifier;
mod runtime;

pub use event::EventInfo;
pub use hookmap_core::{
    ButtonAction, EmulateButtonInput, EmulateMouseCursor, EmulateMouseWheel, EventBlock, Key,
    Mouse, MouseCursor, MouseWheel,
};
pub use interface::{Hook, Modifier, SelectHandleTarget};
pub use runtime::interruption;
