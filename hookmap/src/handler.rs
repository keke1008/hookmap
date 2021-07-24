use crate::{
    event::{Button, EventInfo},
    modifier::{Modifier, ModifierChecker},
};
use derive_new::new;
use hookmap_core::{EventBlock, Key, MouseInput};
use std::{collections::HashMap, fmt::Debug, hash::Hash};

#[derive(new)]
pub(crate) struct HandlerFunction<I: Send + Debug> {
    handler: Box<dyn FnMut(EventInfo<I>) + Send>,
    modifier: Modifier,
}

impl<I: Send + Debug> HandlerFunction<I> {
    fn call(&mut self, event_info: I) -> EventBlock {
        let (event_detail, rx) = EventInfo::channel(event_info);
        (self.handler)(event_detail);
        rx.recv().unwrap()
    }
}

pub(crate) struct HandlerVec<I: Copy + Send + Debug>(Vec<HandlerFunction<I>>);

impl<I: Copy + Send + Debug> HandlerVec<I> {
    pub(crate) fn push<F>(&mut self, handler: F, modifier: Modifier)
    where
        F: FnMut(EventInfo<I>) + Send + 'static,
    {
        let handler_function = HandlerFunction::new(Box::new(handler), modifier);
        self.0.push(handler_function);
    }

    pub(crate) fn call_available(&mut self, event_info: I) -> Vec<EventBlock> {
        let mut modifier_checker = ModifierChecker::new();
        self.0
            .iter_mut()
            .filter(move |handler| modifier_checker.check(&handler.modifier))
            .map(move |handler| handler.call(event_info))
            .collect()
    }
}

impl<I: Copy + Send + Debug> Default for HandlerVec<I> {
    fn default() -> Self {
        Self(Vec::default())
    }
}

pub(crate) struct HandlerMap<K, I>(HashMap<K, HandlerVec<I>>)
where
    K: Eq + Hash,
    I: Copy + Send + Debug;

impl<K, I> HandlerMap<K, I>
where
    K: Eq + Hash,
    I: Copy + Send + Debug,
{
    pub(crate) fn get(&mut self, key: K) -> &mut HandlerVec<I> {
        self.0.entry(key).or_default()
    }

    pub(crate) fn call_available(&mut self, key: K, event_info: I) -> Vec<EventBlock> {
        self.0
            .get_mut(&key)
            .map(|handler| handler.call_available(event_info))
            .unwrap_or_default()
    }
}

impl<K, I> Default for HandlerMap<K, I>
where
    K: Eq + Hash,
    I: Copy + Send + Debug,
{
    fn default() -> Self {
        Self(HashMap::default())
    }
}

#[derive(Default)]
pub(crate) struct KeyboardHandler {
    pub(crate) on_press_or_release: HandlerMap<Key, Button>,
    pub(crate) on_press: HandlerMap<Key, ()>,
    pub(crate) on_release: HandlerMap<Key, ()>,
}

#[derive(Default)]
pub(crate) struct MouseHandler {
    pub(crate) on_press_or_release: HandlerMap<MouseInput, Button>,
    pub(crate) on_press: HandlerMap<MouseInput, ()>,
    pub(crate) on_release: HandlerMap<MouseInput, ()>,
    pub(crate) wheel: HandlerVec<i32>,
    pub(crate) cursor: HandlerVec<(i32, i32)>,
}

#[derive(Default)]
pub struct Handler {
    pub(crate) keyboard: KeyboardHandler,
    pub(crate) mouse: MouseHandler,
}

impl Debug for Handler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Handler")
    }
}
