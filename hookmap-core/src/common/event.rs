use crate::ButtonAction;

/// Indicates whether to pass the generated event to the next program or not .
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventBlock {
    /// Do not pass the generated event to the next program.
    Block,

    /// Pass the generated event to the next program.
    Unblock,
}

/// Information about the generated event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ButtonEvent<T> {
    /// Target of the generated event.
    pub target: T,

    /// Action of the generated event.
    pub action: ButtonAction,
}

impl<T> ButtonEvent<T> {
    /// Creates a new `ButtonEvent<T, A>`.
    pub fn new(target: T, action: ButtonAction) -> Self {
        Self { target, action }
    }
}
