//! Determining whether or not to execute hooks.
//!
//! Layer is a concept that determine whether hooks are executed or not, and has the following features.
//! * A hook always belong to one layer.
//! * Layers build a tree structure. (1 parent, 0 or more children)
//! * Each layer has an internal boolean value, that can be modified by [`LayerRef::enable`] and
//! [`LayerRef::disable`].
//! * A layer is active when self and all ancestors are enabled and all descendants are disabled.
//! * Hooks belonging to the active layer can be executed.

pub(super) mod relation;

pub(crate) mod detector;
pub(crate) mod facade;
pub(crate) mod state;

pub(crate) use detector::{DetectedEvent, LayerAction};
pub(crate) use facade::LayerFacade;
pub(crate) use state::{LayerIndex, LayerState};
