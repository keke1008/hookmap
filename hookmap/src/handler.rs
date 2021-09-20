mod fetcher;
mod register;
mod storage;

pub(crate) use fetcher::{ButtonFetcher, MouseFetcher};
pub(crate) use register::Register;
pub(crate) use storage::Storage;

use crate::interface::Conditions;
use hookmap_core::EventBlock;
use std::{fmt::Debug, sync::Arc};

pub(crate) type Callback<E> = Arc<dyn Fn(E) + Send + Sync>;

pub(crate) struct Handler<E: Debug> {
    pub(crate) callback: Callback<E>,
    pub(crate) conditions: Arc<Conditions>,
    pub(crate) event_block: EventBlock,
}

impl<E: Debug> Handler<E> {
    pub(crate) fn new(
        callback: Callback<E>,
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

impl<E: Debug> Debug for Handler<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Handler")
            .field("conditions", &self.conditions)
            .field("event_block", &self.event_block)
            .finish()
    }
}
