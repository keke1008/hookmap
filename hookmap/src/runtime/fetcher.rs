use super::{
    compute_event_block,
    storage::{ButtonStorage, MouseStorage},
};
use crate::hotkey::Action;
use hookmap_core::{ButtonEvent, EventBlock};
use smart_default::SmartDefault;

#[derive(SmartDefault)]
pub(crate) struct FetchResult<E> {
    pub(super) actions: Vec<Action<E>>,
    #[default(EventBlock::Unblock)]
    pub(super) event_block: EventBlock,
}

pub(crate) struct ButtonFetcher {
    storage: ButtonStorage,
}

impl ButtonFetcher {
    pub(crate) fn new(storage: ButtonStorage) -> Self {
        Self { storage }
    }

    fn fetch_inner(&self, event: &ButtonEvent) -> Option<FetchResult<ButtonEvent>> {
        let (actions, event_blocks): (Vec<_>, Vec<_>) = self
            .storage
            .get(&event.target)?
            .iter()
            .filter(|hook| hook.satisfies_condition())
            .map(|hook| (hook.action.clone(), hook.event_block))
            .unzip();

        Some(FetchResult {
            actions,
            event_block: compute_event_block(&event_blocks),
        })
    }

    pub(crate) fn fetch(&self, event: &ButtonEvent) -> FetchResult<ButtonEvent> {
        self.fetch_inner(event).unwrap_or_default()
    }
}

pub(crate) struct MouseFetcher<E: Clone> {
    storage: MouseStorage<E>,
}

impl<E: Clone> MouseFetcher<E> {
    pub(crate) fn new(storage: MouseStorage<E>) -> Self {
        Self { storage }
    }

    fn fetch_inner(&self) -> Option<FetchResult<E>> {
        let (actions, event_blocks): (Vec<_>, Vec<_>) = self
            .storage
            .iter()
            .filter(|hook| hook.modifier.satisfies_condition())
            .map(|hook| (hook.action.clone(), hook.event_block))
            .unzip();

        Some(FetchResult {
            actions,
            event_block: compute_event_block(&event_blocks),
        })
    }

    pub(crate) fn fetch(&self) -> FetchResult<E> {
        self.fetch_inner().unwrap_or_default()
    }
}
