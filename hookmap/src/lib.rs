//! # Hookmap
//!
//! Register hotkeys and emulate input.
//!
//! ## Handler order.
//!
//! Handlers are called in the order in which they are registered.
//!
//! ## EventBlock
//!
//! [`EventBlock`] can be used in the following cases,
//! and the higier it is, the higher the priority.
//!
//! * The argument of [`modifier_key`] or [`modifier_mouse_button`]
//! * The closue argument passed to [`ButtonRegister::on_*`]
//!     ( call [`block_event`], or [`unblock_event`] to change [`EventBlock`])
//!
//! [`modifier_key`]: crate::SelectHandleTarget::modifier_key
//! [`modifier_mouse_button`]: crate::SelectHandleTarget::modifier_mouse_button
//! [`ButtonRegister::on_*`]: crate::interface::ButtonRegister
//! [`block_event`]: crate::EventInfo::block_event
//! [`unblock_event`]: crate::EventInfo::unblock_event
//!
//! ## Feature flags
//!
//! * `block-input-event`: Set the default `EventBlock` value to `EventBlock::Block`.
//!

pub mod event;
pub mod interface;

mod handler;
mod modifier;
mod runtime;

pub use event::EventInfo;
pub use hookmap_core::{
    ButtonAction, EmulateButtonInput, EmulateMouseCursor, EmulateMouseWheel, EventBlock, Key, Mouse,
};
pub use interface::{Hook, Modifier, SelectHandleTarget};
pub use runtime::interruption::Interruption;
