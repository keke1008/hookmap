//! A core library for [`hookmap`].
//!
//! [`hookmap`]: https://crates.io/crates/hookmap
//!
//! This library provides abstracted input emulation and global hooks for keyboard and mouse.
//!
//! ## Feature flags
//!
//! * `block-input-event`: Set button events to be blocked by default.
//! * `us-keyboard-layout` (default): Set the variants of [`Button`] to the buttons on the us keyboard.
//! * `japanese-keyboard-layout`: Set the variants of [`Button`] to the buttons on the japanese keyboard.

pub mod common;

mod sys;

pub use common::{
    button::{Button, ButtonAction, ButtonKind, ButtonOperation},
    event::{ButtonEvent, Event, MouseCursorEvent, MouseWheelEvent, NativeEventOperation},
    handler::HookHandler,
    mouse::{EmulateMouseCursor, EmulateMouseWheel, Mouse},
};
