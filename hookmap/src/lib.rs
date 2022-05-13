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
pub mod utils;

#[doc(hidden)]
pub mod macros;

mod hook;
mod runtime;

pub use runtime::interceptor;

/// Representation of keyboard and mouse events.
pub mod device {
    pub use hookmap_core::button::{Button, ButtonAction, ButtonKind};
    pub use hookmap_core::event::{ButtonEvent, CursorEvent, NativeEventOperation, WheelEvent};
    pub use hookmap_core::mouse;
}

/// A prelude for conveniently defining hotkeys.
pub mod prelude {
    // Macros
    pub use super::{buttons, hotkey, seq};

    pub use super::{
        device::*,
        hotkey::{Context, Hotkey},
        interceptor::{Filter, Interceptor},
        utils,
    };
}
