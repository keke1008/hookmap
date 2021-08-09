use crate::modifier::{ModifierButtonSet, ModifierChecker};
use hookmap_core::{Button, ButtonAction};
use std::{cell::RefCell, collections::HashMap, fmt::Debug, hash::Hash, rc::Rc, sync::Arc};

pub(crate) struct HandlerFunction<I: Send + Debug + 'static> {
    callback: Box<dyn FnMut(I) + Send>,
    modifier: Arc<ModifierButtonSet>,
}

impl<I: Send + Debug + 'static> HandlerFunction<I> {
    pub(crate) fn new(
        callback: Box<dyn FnMut(I) + Send>,
        modifier: Arc<ModifierButtonSet>,
    ) -> Self {
        Self { callback, modifier }
    }
}

impl<I: Send + Debug + 'static> Debug for HandlerFunction<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HandlerFunction")
    }
}

#[derive(Debug)]
pub(crate) struct HandlerVec<I: Copy + Send + Debug + 'static>(Vec<HandlerFunction<I>>);

impl<I: Copy + Send + Debug + 'static> HandlerVec<I> {
    pub(crate) fn push(
        &mut self,
        handler: Box<dyn FnMut(I) + Send>,
        modifier: Arc<ModifierButtonSet>,
    ) {
        let handler_function = HandlerFunction::new(Box::new(handler), modifier);
        self.0.push(handler_function);
    }

    pub(crate) fn call_available(&mut self, event_info: I) {
        let mut modifier_checker = ModifierChecker::new();
        self.0
            .iter_mut()
            .filter(move |handler| modifier_checker.check(&handler.modifier))
            .for_each(move |handler| (handler.callback)(event_info))
    }
}

impl<I: Copy + Send + Debug> Default for HandlerVec<I> {
    fn default() -> Self {
        Self(Vec::default())
    }
}

#[derive(Debug)]
pub(crate) struct HandlerMap<B, I>(HashMap<B, HandlerVec<I>>)
where
    B: Eq + Hash + Debug,
    I: Copy + Send + Debug + 'static;

impl<B, I> HandlerMap<B, I>
where
    B: Eq + Hash + Debug,
    I: Copy + Send + Debug + 'static,
{
    pub(crate) fn get(&mut self, button: B) -> &mut HandlerVec<I> {
        self.0.entry(button).or_default()
    }

    pub(crate) fn call_available(&mut self, button: B, event_info: I) {
        self.0
            .get_mut(&button)
            .map(|handler| handler.call_available(event_info))
            .unwrap_or_default()
    }
}

impl<K, I> Default for HandlerMap<K, I>
where
    K: Eq + Hash + Debug,
    I: Copy + Send + Debug,
{
    fn default() -> Self {
        Self(HashMap::default())
    }
}

#[derive(Debug, Default)]
pub(crate) struct ButtonHandler {
    pub(crate) on_press_or_release: HandlerMap<Button, ButtonAction>,
    pub(crate) on_press: HandlerMap<Button, ()>,
    pub(crate) on_release: HandlerMap<Button, ()>,
    pub(crate) on_release_alone: HandlerMap<Button, ()>,
}

#[derive(Debug, Default)]
pub struct Handler {
    pub(crate) button: Rc<RefCell<ButtonHandler>>,
    pub(crate) mouse_cursor: Rc<RefCell<HandlerVec<(i32, i32)>>>,
    pub(crate) mouse_wheel: Rc<RefCell<HandlerVec<i32>>>,
}
