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
//! * [`ButtonOperation`] for [`Button`]
//! * [`EmulateMouseCursor`] for [`Mouse`]
//! * [`EmulateMouseWheel`] for [`Mouse`]
//! * [`HookInstaller`] for [`InputHandler`]
//!
//! [`HookInstaller`]: crate::common::handler::HookInstaller
//!
//! ## Feature flags
//!
//! * `block-input-event`: Set the default `EventBlock` value to `EventBlock::Block`.

pub mod common;
mod macros;

#[cfg(target_os = "windows")]
mod windows;

pub use common::{
    button::{Button, ButtonAction, ButtonKind, ButtonOperation},
    event::{ButtonEvent, EventBlock, MouseCursorEvent, MouseWheelEvent},
    handler::{EventCallback, EventCallbackGenerator, EventHandler, InputHandler},
    mouse::{EmulateMouseCursor, EmulateMouseWheel, Mouse},
};
