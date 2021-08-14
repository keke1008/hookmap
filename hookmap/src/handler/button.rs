use super::{HandlerVec, SatisfiedHandler};
use hookmap_core::{Button, ButtonEvent};
use std::{collections::HashMap, fmt::Debug};

#[derive(Debug, Default)]
pub(crate) struct ButtonCallbackMap(HashMap<Button, HandlerVec<ButtonEvent>>);

impl ButtonCallbackMap {
    pub(crate) fn get_mut(&mut self, button: Button) -> &mut HandlerVec<ButtonEvent> {
        self.0.entry(button).or_default()
    }

    pub(crate) fn get_satisfied(&self, event: ButtonEvent) -> SatisfiedHandler<ButtonEvent> {
        let handlers = match self.0.get(&event.target) {
            Some(handler) => handler.get_satisfied(),
            None => vec![],
        };
        SatisfiedHandler::new(handlers, event)
    }
}

#[derive(Debug, Default)]
pub(crate) struct ButtonEventCallback {
    pub(crate) on_press: ButtonCallbackMap,
    pub(crate) on_release: ButtonCallbackMap,
    pub(crate) on_release_alone: ButtonCallbackMap,
}
