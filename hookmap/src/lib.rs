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
pub mod macros;
pub mod utils;

mod button;
mod hook;
mod runtime;

/// Items used for button and mouse event.
pub mod event {
    pub use hookmap_core::{ButtonEvent, MouseCursorEvent, MouseWheelEvent, NativeEventOperation};
}

/// keyboard and mouse, and their inputs.
pub mod devices {
    pub use super::button::{
        Button, ButtonAction, ButtonInput, ButtonState, Sequence, SequenceOperation, ALT, CTRL,
        META, SHIFT,
    };
    pub use hookmap_core::{EmulateMouseCursor, EmulateMouseWheel, Mouse};
}

pub use runtime::interceptor;

/// A prelude for conveniently defining hotkeys.
pub mod prelude {
    pub use super::{
        arg,
        devices::{
            Button, ButtonAction, ButtonInput, ButtonState, EmulateMouseCursor, EmulateMouseWheel,
            Mouse,
        },
        hotkey,
        hotkey::{Hotkey, RegisterHotkey},
        interceptor::{Filter, Interceptor},
        send, seq,
        utils::Utils,
    };
}
