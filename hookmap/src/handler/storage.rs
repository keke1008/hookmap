use super::Handler;
use hookmap_core::{Button, ButtonEvent, MouseCursorEvent, MouseWheelEvent};
use std::{collections::HashMap, sync::Arc};

pub(crate) type ButtonStorage = HashMap<Button, Vec<Arc<Handler<ButtonEvent>>>>;
type MouseStorage<E> = Vec<Arc<Handler<E>>>;
pub(crate) type MouseCursorStorage = MouseStorage<MouseCursorEvent>;
pub(crate) type MouseWheelStorage = MouseStorage<MouseWheelEvent>;

#[derive(Debug, Default)]
pub(crate) struct Storage {
    pub(crate) button_on_press: ButtonStorage,
    pub(crate) button_on_release: ButtonStorage,
    pub(crate) mouse_cursor: MouseCursorStorage,
    pub(crate) mouse_wheel: MouseWheelStorage,
}
