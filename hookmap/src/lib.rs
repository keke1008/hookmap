//! Register hotkeys and emulate input.
//!
//! ## Handler order.
//!
//! Handlers are called in the order in which they are registered.
//!
//! ## Feature flags
//!
//! * `block-input-event`: Set button events to be blocked by default.

mod handler;
mod interface;
mod runtime;

pub use hookmap_core::{
    Button, ButtonAction, ButtonInput as EmulateButtonInput, ButtonState, EmulateMouseCursor,
    EmulateMouseWheel, Mouse,
};
pub use interface::{
    All, Any, ButtonRegister, ButtonSet, Cond, ConditionalHook,
    DownCastableButtonState as EmulateButtonState, Hook, MouseCursorRegister, MouseWheelRegister,
    SelectHandleTarget,
};
pub use runtime::interruption;
