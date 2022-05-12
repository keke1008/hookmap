//! Register hotkeys and emulate input.
//!
//! ## Handler order.
//!
//! Handlers are called in the order in which they are registered.
//!
//! ## Feature flags
//!
//! * `block-input-event`: Set button events to be blocked by default.

pub mod hotkey;
// pub mod utils;

#[doc(hidden)]
pub mod macros;

mod button;
mod hook;
mod runtime;

/// Items used for button and mouse event.
pub mod event {
    pub use hookmap_core::event::{ButtonEvent, CursorEvent, NativeEventOperation, WheelEvent};
}

/// keyboard and mouse, and their inputs.
pub mod devices {
    pub use super::button::{Button, ButtonAction, Sequence, SequenceOperation};
}

pub use runtime::interceptor;

/// A prelude for conveniently defining hotkeys.
pub mod prelude {
    pub use super::{
        buttons,
        devices::{Button, ButtonAction},
        event::{ButtonEvent, CursorEvent, NativeEventOperation, WheelEvent},
        hotkey,
        hotkey::{ContextBuilder, Hotkey},
        interceptor::{Filter, Interceptor},
        seq,
        // utils::Utils,
    };
}
