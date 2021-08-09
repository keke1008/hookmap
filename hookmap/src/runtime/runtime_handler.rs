use crate::handler::{ButtonHandler, Handler, HandlerVec};
use std::rc::Rc;

#[derive(Debug)]
pub(super) struct RuntimeHandler {
    pub(super) button: ButtonHandler,
    pub(super) mouse_cursor: HandlerVec<(i32, i32)>,
    pub(super) mouse_wheel: HandlerVec<i32>,
}

impl From<Handler> for RuntimeHandler {
    fn from(handler: Handler) -> Self {
        Self {
            button: Rc::try_unwrap(handler.button).unwrap().into_inner(),
            mouse_cursor: Rc::try_unwrap(handler.mouse_cursor).unwrap().into_inner(),
            mouse_wheel: Rc::try_unwrap(handler.mouse_wheel).unwrap().into_inner(),
        }
    }
}
