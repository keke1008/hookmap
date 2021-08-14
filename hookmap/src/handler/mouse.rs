use super::{HandlerVec, SatisfiedHandler};
use crate::cond::Conditions;
use hookmap_core::EventBlock;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug, Default)]
pub(crate) struct MouseEventCallBack<E: Copy + Debug + PartialEq + Send + 'static>(HandlerVec<E>);

impl<E: Copy + Debug + PartialEq + Send + 'static> MouseEventCallBack<E> {
    pub(crate) fn push(
        &mut self,
        callback: Arc<dyn Fn(E) + Send + Sync>,
        conditions: Arc<Conditions>,
        event_block: EventBlock,
    ) {
        self.0.push(callback, conditions, event_block);
    }

    pub(crate) fn get_satisfied(&self, event: E) -> SatisfiedHandler<E> {
        SatisfiedHandler::new(self.0.get_satisfied(), event)
    }
}
