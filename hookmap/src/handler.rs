use crate::{
    event::EventInfo,
    modifier::{ModifierChecker, ModifierSet},
};
use derive_new::new;
use hookmap_core::{ButtonAction, EventBlock, Key, Mouse};
use std::{cell::RefCell, collections::HashMap, fmt::Debug, hash::Hash, rc::Rc, sync::Arc};

#[derive(new)]
pub(crate) struct HandlerFunction<I: Send + Debug> {
    callback: Box<dyn FnMut(EventInfo<I>) + Send>,
    modifier: Arc<ModifierSet>,
}

impl<I: Send + Debug> HandlerFunction<I> {
    fn call(&mut self, info: I) -> EventBlock {
        EventInfo::new(info).send_with(self.callback)
    }
}

pub(crate) struct HandlerVec<I: Copy + Send + Debug>(Vec<HandlerFunction<I>>);

impl<I: Copy + Send + Debug> HandlerVec<I> {
    pub(crate) fn push<F>(&mut self, handler: F, modifier: Arc<ModifierSet>)
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

impl<I: Copy + Send + Debug> Debug for HandlerVec<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HandlerVec")
    }
}

pub(crate) struct HandlerMap<B, I>(HashMap<B, HandlerVec<I>>)
where
    B: Eq + Hash,
    I: Copy + Send + Debug;

impl<B, I> HandlerMap<B, I>
where
    B: Eq + Hash,
    I: Copy + Send + Debug,
{
    pub(crate) fn get(&mut self, button: B) -> &mut HandlerVec<I> {
        self.0.entry(button).or_default()
    }

    pub(crate) fn call_available(&mut self, button: B, event_info: I) -> Vec<EventBlock> {
        self.0
            .get_mut(&button)
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

pub(crate) struct ButtonHandler<T: Eq + Hash> {
    pub(crate) on_press_or_release: HandlerMap<T, ButtonAction>,
    pub(crate) on_press: HandlerMap<T, ()>,
    pub(crate) on_release: HandlerMap<T, ()>,
}

impl<T: Eq + Hash> Debug for ButtonHandler<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ButtonHandler")
    }
}

impl<T: Eq + Hash> Default for ButtonHandler<T> {
    fn default() -> Self {
        Self {
            on_press: Default::default(),
            on_release: Default::default(),
            on_press_or_release: Default::default(),
        }
    }
}

#[derive(Default)]
pub struct Handler {
    pub(crate) keyboard: Rc<RefCell<ButtonHandler<Key>>>,
    pub(crate) mouse_button: Rc<RefCell<ButtonHandler<Mouse>>>,
    pub(crate) mouse_cursor: Rc<RefCell<HandlerVec<(i32, i32)>>>,
    pub(crate) mouse_wheel: Rc<RefCell<HandlerVec<i32>>>,
}

impl Debug for Handler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Handler")
    }
}
