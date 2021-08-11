use crate::modifier::{ModifierButtonSet, ModifierChecker};
use hookmap_core::common::event::EventBlock;
use std::{fmt::Debug, sync::Arc};

pub(crate) struct SatisfiedHandler<'a, I: Copy + Debug + PartialEq + Send + 'static> {
    handler: Vec<&'a HandlerFunction<I>>,
    event: I,
}

impl<'a, I: Copy + Debug + PartialEq + Send + 'static> SatisfiedHandler<'a, I> {
    pub(super) fn new(handler: Vec<&'a HandlerFunction<I>>, event: I) -> Self {
        Self { handler, event }
    }

    pub(crate) fn extend(&mut self, other: Self) {
        assert_eq!(self.event, other.event);
        self.handler.extend(other.handler);
    }

    pub(crate) fn get_event_blocks(&self) -> Vec<EventBlock> {
        self.handler
            .iter()
            .map(|handler| handler.event_block)
            .collect()
    }

    pub(crate) fn call(&self) {
        self.handler
            .iter()
            .for_each(|handler| (handler.callback)(self.event));
    }
}

pub(crate) struct HandlerFunction<I: Send + Debug + 'static> {
    pub(crate) callback: Box<dyn Fn(I) + Send + Sync>,
    pub(crate) modifier: Arc<ModifierButtonSet>,
    pub(crate) event_block: EventBlock,
}

impl<I: Send + Debug + 'static> HandlerFunction<I> {
    pub(crate) fn new(
        callback: Box<dyn Fn(I) + Send + Sync>,
        modifier: Arc<ModifierButtonSet>,
        event_block: EventBlock,
    ) -> Self {
        Self {
            callback,
            modifier,
            event_block,
        }
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
        handler: Box<dyn Fn(I) + Send + Sync>,
        modifier: Arc<ModifierButtonSet>,
        event_block: EventBlock,
    ) {
        let handler_function = HandlerFunction::new(Box::new(handler), modifier, event_block);
        self.0.push(handler_function);
    }

    pub(crate) fn get_satisfied(&self) -> Vec<&HandlerFunction<I>> {
        let mut modifier_checker = ModifierChecker::new();
        self.0
            .iter()
            .filter(move |handler| modifier_checker.check(&handler.modifier))
            .collect()
    }
}

impl<I: Copy + Send + Debug> Default for HandlerVec<I> {
    fn default() -> Self {
        Self(Vec::default())
    }
}
