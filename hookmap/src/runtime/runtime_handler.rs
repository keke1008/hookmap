use std::rc::Rc;

use crate::handler::{ButtonHandler, Handler, HandlerVec};
use hookmap_core::{Key, Mouse};

#[derive(Debug)]
pub(super) struct RuntimeHandler {
    pub(super) keyboard: ButtonHandler<Key>,
    pub(super) mouse_button: ButtonHandler<Mouse>,
    pub(super) mouse_cursor: HandlerVec<(i32, i32)>,
    pub(super) mouse_wheel: HandlerVec<i32>,
}

impl From<Handler> for RuntimeHandler {
    fn from(handler: Handler) -> Self {
        Self {
            keyboard: Rc::try_unwrap(handler.keyboard).unwrap().into_inner(),
            mouse_button: Rc::try_unwrap(handler.mouse_button).unwrap().into_inner(),
            mouse_cursor: Rc::try_unwrap(handler.mouse_cursor).unwrap().into_inner(),
            mouse_wheel: Rc::try_unwrap(handler.mouse_wheel).unwrap().into_inner(),
        }
    }
}
