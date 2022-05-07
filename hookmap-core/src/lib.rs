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

pub mod button;
pub mod event;

mod sys;

pub use sys::{install_hook, mouse, uninstall_hook};
