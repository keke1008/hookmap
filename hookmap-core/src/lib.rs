//! A core library for [`hookmap`].
//!
//! [`hookmap`]: https://crates.io/crates/hookmap
//!
//! This library provides abstracted input emulation and global hooks for keyboard and mouse.
//!
//! # Required traits.
//!
//! In order to support another OS, these traits need to be implemented.
//!
//! * [`EmulateButtonInput`] for [`Key`]
//! * [`EmulateButtonInput`] for [`Mouse`]
//! * [`EmulateMouseCursor`] for [`Mouse`]
//! * [`EmulateMouseWheel`] for [`Mouse`]
//! * [`InstallKeyboardHook`] for [`InputHandler`]
//! * [`InstallMouseHook`] for [`InputHandler`]
//! * [`HandleInput`] for [`InputHandler`]
//!
//! [`InstallKeyboardHook`]: crate::common::keyboard::InstallKeyboardHook
//! [`InstallMouseHook`]: crate::common::mouse::InstallMouseHook
//! [`HandleInput`]: crate::common::handler::HandleInput
//!
//! ## Feature flags
//!
//! * `block-input-event`: Set the default `EventBlock` value to `EventBlock::Block`.

pub mod common;
mod macros;

#[cfg(target_os = "windows")]
mod windows;

pub use common::{
    button::{Button, ButtonAction, ButtonInput, ButtonKind, ButtonState},
    event::{ButtonEvent, ButtonEventBlockMap},
    handler::{HandlerFunction, InputHandler},
    mouse::{EmulateMouseCursor, EmulateMouseWheel, Mouse},
    BUTTON_EVENT_BLOCK, INPUT_HANDLER,
};
