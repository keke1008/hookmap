//! A core library of [hookmap].
//!
//! [hookmap]: https://crates.io/crates/hookmap
//!
//! This library provides input simulation and global hooks for keyboard and mouse.
//!
//! ## Feature flags
//!
//! * `us-keyboard-layout` (default): Use US keyboard layout. This changes the [`Button`] variant.
//! * `japanese-keyboard-layout`: Use Japanese keyboard layout. This changes the [`Button`] variant.
//!
//! [`Button`]: button::Button
//!

pub mod button;
pub mod event;

mod sys;

pub use sys::{install_hook, mouse, uninstall_hook};
