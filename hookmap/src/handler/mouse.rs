use super::{HandlerVec, SatisfiedHandler};
use crate::modifier::ModifierButtonSet;
use hookmap_core::EventBlock;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug, Default)]
pub(crate) struct MouseEventCallBack<E: Copy + Debug + PartialEq + Send + 'static>(HandlerVec<E>);

impl<E: Copy + Debug + PartialEq + Send + 'static> MouseEventCallBack<E> {
    pub(crate) fn push(
        &mut self,
        callback: Arc<dyn Fn(E) + Send + Sync>,
        modifier: Arc<ModifierButtonSet>,
        event_block: EventBlock,
    ) {
        self.0.push(callback, modifier, event_block);
    }

    pub(crate) fn get_satisfied(&self, event: E) -> SatisfiedHandler<E> {
        SatisfiedHandler::new(self.0.get_satisfied(), event)
    }
}
