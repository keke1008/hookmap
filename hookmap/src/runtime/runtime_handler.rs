use crate::handler::{ButtonEventCallback, EventCallback, MouseEventCallBack};
use hookmap_core::{MouseCursorEvent, MouseWheelEvent};
use std::rc::Rc;

#[derive(Debug)]
pub(super) struct RuntimeHandler {
    pub(super) button: ButtonEventCallback,
    pub(super) mouse_cursor: MouseEventCallBack<MouseCursorEvent>,
    pub(super) mouse_wheel: MouseEventCallBack<MouseWheelEvent>,
}

impl From<EventCallback> for RuntimeHandler {
    fn from(handler: EventCallback) -> Self {
        Self {
            button: Rc::try_unwrap(handler.button).unwrap().into_inner(),
            mouse_cursor: Rc::try_unwrap(handler.mouse_cursor).unwrap().into_inner(),
            mouse_wheel: Rc::try_unwrap(handler.mouse_wheel).unwrap().into_inner(),
        }
    }
}
