//! Registers hotkeys and simulates keyboard and mouse input.
//!
//! # Feature flags
//!
//! * `us-keyboard-layout` (default): Use US keyboard layout. This changes the [`Button`] variant.
//! * `japanese-keyboard-layout`: Use Japanese keyboard layout. This changes the [`Button`] variant.
//!
//! [`Button`]: crate::device::Button

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
    pub use hookmap_core::event::{ButtonEvent, CursorEvent, WheelEvent};
    pub use hookmap_core::hook::NativeEventOperation;
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
