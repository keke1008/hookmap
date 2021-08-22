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
    Button, ButtonAction, ButtonInput, ButtonState, EmulateMouseCursor, EmulateMouseWheel, Mouse,
};
pub use interface::{
    ButtonSet, Cond, ConditionalHook, Hook, SelectHandleTarget, SetEventBlock, Utils,
};
pub use runtime::interruption::Interruption;

pub mod button {
    pub use super::interface::{
        All, Any, BorrowedEmulateButtonInput, EmulateButtonInput, EmulateButtonState,
        ToButtonWithState,
    };
}

pub mod register {
    pub use super::interface::{ButtonRegister, MouseCursorRegister, MouseWheelRegister};
}

pub mod interruption {
    pub use super::runtime::interruption::EventReceiver;
}
