//! Keyboard and mouse events.

use super::button::{Button, ButtonAction};

/// Indicates button event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ButtonEvent {
    /// Target of the generated event.
    pub target: Button,

    /// Action of the generated event.
    pub action: ButtonAction,

    /// Whether this event was generated by this program.
    /// If you type on your keyboard and an event is generated, this value will be `false`.
    pub injected: bool,
}

/// Indicates mouse cursor event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CursorEvent {
    /// Mouse cursor movement `(x, y)`
    pub delta: (i32, i32),

    /// Whether this event was generated by this program.
    pub injected: bool,
}

/// Indicates mouse wheel event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WheelEvent {
    /// Amout of mouse wheel rotation
    /// Upward rotation takes a positive value, downward rotation a negative value.
    pub delta: i32,

    /// Whether this event was generated by this program.
    pub injected: bool,
}

/// An event
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Event {
    /// Button event
    Button(ButtonEvent),

    /// Mouse wheel event
    Wheel(WheelEvent),

    /// Mouse cursor event
    Cursor(CursorEvent),
}
