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
pub use interface::{ButtonSet, Cond, ConditionalHook, Hook, SelectHandleTarget, SetEventBlock};
pub use runtime::interruption::Interruption;

pub mod button {
    pub use super::interface::{All, Any, ToButtonWithState};
}

pub mod register {
    pub use super::interface::{ButtonRegister, MouseCursorRegister, MouseWheelRegister};
}

pub mod interruption {
    pub use super::runtime::interruption::EventReceiver;
}
