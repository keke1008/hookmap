mod button;
mod common;
mod mouse;
pub(crate) use button::{ButtonCallbackMap, ButtonEventCallback};
pub(crate) use common::{HandlerVec, SatisfiedHandler};
pub(crate) use mouse::MouseEventCallBack;

use hookmap_core::{MouseCursorEvent, MouseWheelEvent};

use std::{cell::RefCell, fmt::Debug, rc::Rc};

#[derive(Debug, Default)]
pub struct EventCallback {
    pub(crate) button: Rc<RefCell<ButtonEventCallback>>,
    pub(crate) mouse_cursor: Rc<RefCell<MouseEventCallBack<MouseCursorEvent>>>,
    pub(crate) mouse_wheel: Rc<RefCell<MouseEventCallBack<MouseWheelEvent>>>,
}
