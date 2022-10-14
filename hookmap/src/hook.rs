//! Hook types

pub(crate) mod input;
pub(crate) mod layer;

pub mod hooks;
pub mod interruption;

pub(crate) use hooks::{Actions, HookAction, HookActionRunner, Procedure};

pub use hooks::{Layer, OptionalProcedure, RequiredProcedure};
