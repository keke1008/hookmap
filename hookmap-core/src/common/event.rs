/// Indicates whether to pass the generated event to the next program or not .
#[derive(Debug, Clone, Copy)]
pub enum EventBlock {
    /// Do not pass the generated event to the next program.
    Block,

    /// Pass the generated event to the next program.
    Unblock,
}

/// Information about the generated event.
/// When dropped, this will send whether to pass the event to the next program or not
/// to the thread where the hook is installed.
#[derive(Debug)]
pub struct EventDetail<T, A> {
    /// Target of the generated event.
    pub target: T,

    /// Action of the generated event.
    pub action: A,
}

impl<T, A> EventDetail<T, A> {
    /// Creates a new `Event<T, A>`.
    pub fn new(target: T, action: A) -> Self {
        Self { target, action }
    }
}
