use super::Handler;
use hookmap_core::{Button, ButtonEvent, MouseCursorEvent, MouseWheelEvent};
use std::{collections::HashMap, sync::Arc};

#[derive(Default)]
pub(crate) struct Storage {
    pub(crate) button_on_press: HashMap<Button, Vec<Arc<Handler<ButtonEvent>>>>,
    pub(crate) button_on_release: HashMap<Button, Vec<Arc<Handler<ButtonEvent>>>>,
    pub(crate) mouse_cursor: Vec<Arc<Handler<MouseCursorEvent>>>,
    pub(crate) mouse_wheel: Vec<Arc<Handler<MouseWheelEvent>>>,
}
