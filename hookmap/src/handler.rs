mod fetcher;
mod register;
mod storage;

pub(crate) use fetcher::{ButtonFetcher, MouseFetcher};
pub(crate) use register::Register;
pub(crate) use storage::Storage;

use crate::interface::Conditions;
use hookmap_core::EventBlock;
use std::sync::Arc;

type Callback<E> = Arc<dyn Fn(E) + Send + Sync>;

pub(crate) struct Handler<E> {
    pub(crate) callback: Callback<E>,
    pub(crate) conditions: Arc<Conditions>,
    pub(crate) event_block: EventBlock,
}

impl<E> Handler<E> {
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
