//! Registers hotkeys and simulates keyboard and mouse input.
//!
//! # Feature flags
//!
//! * `us-keyboard-layout` (default): Use US keyboard layout. This changes the [`Button`] variant.
//! * `japanese-keyboard-layout`: Use Japanese keyboard layout. This changes the [`Button`] variant.
//!
//! [`Button`]: crate::device::Button

pub mod hotkey;
// pub mod utils;

#[doc(hidden)]
pub mod macros;

pub mod layer;

mod runtime;

/// Representation of keyboard and mouse events.
pub mod device {
    pub use hookmap_core::button::{Button, ButtonAction, ButtonKind};
    pub use hookmap_core::event::{ButtonEvent, CursorEvent, NativeEventOperation, WheelEvent};
    pub use hookmap_core::mouse;
}

/// A prelude for conveniently defining hotkeys.
pub mod prelude {
    // Macros
    pub use super::{each, modifiers, not, seq};

    pub use super::{
        device::*,
        hotkey::interruption::{Filter, Interruption, InterruptionReceiver},
        hotkey::Hotkey,
    };
}
