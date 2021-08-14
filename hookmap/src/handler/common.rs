use crate::cond::Conditions;
use hookmap_core::common::event::EventBlock;
use std::{fmt::Debug, sync::Arc};

pub(crate) struct SatisfiedHandler<'a, E: Copy + Debug + PartialEq + Send + 'static> {
    handler: Vec<&'a HandlerFunction<E>>,
    event: E,
}

impl<'a, E: Copy + Debug + PartialEq + Send + 'static> SatisfiedHandler<'a, E> {
    pub(super) fn new(handler: Vec<&'a HandlerFunction<E>>, event: E) -> Self {
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

pub(crate) struct HandlerFunction<E: Send + Debug + 'static> {
    pub(crate) callback: Arc<dyn Fn(E) + Send + Sync>,
    pub(crate) conditions: Arc<Conditions>,
    pub(crate) event_block: EventBlock,
}

impl<E: Send + Debug + 'static> HandlerFunction<E> {
    pub(crate) fn new(
        callback: Arc<dyn Fn(E) + Send + Sync>,
        conditions: Arc<Conditions>,
        event_block: EventBlock,
    ) -> Self {
        Self {
            callback,
            conditions,
            event_block,
        }
    }
}

impl<E: Send + Debug + 'static> Debug for HandlerFunction<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HandlerFunction")
    }
}

#[derive(Debug)]
pub(crate) struct HandlerVec<E: Copy + Send + Debug + 'static>(Vec<HandlerFunction<E>>);

impl<E: Copy + Send + Debug + 'static> HandlerVec<E> {
    pub(crate) fn push(
        &mut self,
        handler: Arc<dyn Fn(E) + Send + Sync>,
        conditions: Arc<Conditions>,
        event_block: EventBlock,
    ) {
        let handler_function = HandlerFunction::new(handler, conditions, event_block);
        self.0.push(handler_function);
    }

    pub(crate) fn get_satisfied(&self) -> Vec<&HandlerFunction<E>> {
        self.0
            .iter()
            .filter(move |handler| handler.conditions.is_satisfied())
            .collect()
    }
}

impl<E: Copy + Send + Debug> Default for HandlerVec<E> {
    fn default() -> Self {
        Self(Vec::default())
    }
}
