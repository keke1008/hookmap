use std::sync::Arc;

use super::{
    compute_event_block,
    storage::{ButtonStorage, HookInfo, MouseStorage, RemapStorage, Storage},
};
use crate::{button::ButtonSet, hotkey::Action, ButtonInput};
use hookmap_core::{ButtonEvent, EventBlock, MouseCursorEvent, MouseWheelEvent};

pub(super) struct Fetchers {
    pub(super) on_press_fetcher: ButtonFetcher,
    pub(super) on_release_fetcher: ButtonFetcher,
    pub(super) mouse_cursor_fetcher: MouseFetcher<MouseCursorEvent>,
    pub(super) mouse_wheel_fetcher: MouseFetcher<MouseWheelEvent>,
}

impl From<Storage> for Fetchers {
    fn from(storage: Storage) -> Self {
        let remap = Arc::new(storage.remap);
        Self {
            on_press_fetcher: ButtonFetcher::new(storage.on_press, Arc::clone(&remap)),
            on_release_fetcher: ButtonFetcher::new(storage.on_release, remap),
            mouse_cursor_fetcher: MouseFetcher::new(storage.mouse_cursor),
            mouse_wheel_fetcher: MouseFetcher::new(storage.mouse_wheel),
        }
    }
}

pub(crate) struct FetchResult<E> {
    pub(super) actions: Vec<Action<E>>,
    pub(super) event_block: EventBlock,
}

pub(super) struct ButtonFetcher {
    storage: ButtonStorage,
    remap: Arc<RemapStorage>,
}

impl ButtonFetcher {
    pub(crate) fn new(storage: ButtonStorage, remap: Arc<RemapStorage>) -> Self {
        Self { storage, remap }
    }

    fn try_remap(&self, event: ButtonEvent) -> Result<ButtonSet, ButtonEvent> {
        if let Some(remaps) = self.remap.get(&event.target) {
            if let Some(remap) = remaps.iter().find(|remap| remap.remappable(event.action)) {
                return Ok(remap.target.clone());
            }
        }
        Err(event)
    }

    fn fetch_inner<T, F>(&self, event: &ButtonEvent, f: F) -> (Vec<Action<ButtonEvent>>, Vec<T>)
    where
        F: Fn(&HookInfo) -> T,
    {
        let all_hooks = self
            .storage
            .all
            .iter()
            .filter(|hook| hook.satisfies_condition())
            .map(|hook| (hook.action.clone(), f(hook)));
        if let Some(hooks) = self.storage.just.get(&event.target) {
            hooks
                .iter()
                .filter(|hook| hook.satisfies_condition())
                .map(|hook| (hook.action.clone(), f(hook)))
                .chain(all_hooks)
                .unzip()
        } else {
            all_hooks.unzip()
        }
    }

    pub(crate) fn fetch(&self, event: &ButtonEvent) -> FetchResult<ButtonEvent> {
        match self.try_remap(*event) {
            Ok(remap_target) => FetchResult {
                actions: vec![Action::from(move |_| remap_target.press())],
                event_block: EventBlock::Block,
            },
            Err(event) => {
                let (actions, event_blocks) = self.fetch_inner(&event, |hook| hook.event_block);
                FetchResult {
                    actions,
                    event_block: compute_event_block(&event_blocks),
                }
            }
        }
    }
}

pub(crate) struct MouseFetcher<E: Clone> {
    storage: MouseStorage<E>,
}

impl<E: Clone> MouseFetcher<E> {
    pub(crate) fn new(storage: MouseStorage<E>) -> Self {
        Self { storage }
    }

    pub(crate) fn fetch(&self) -> FetchResult<E> {
        let (actions, event_blocks): (Vec<_>, Vec<_>) = self
            .storage
            .iter()
            .filter(|hook| hook.modifier_keys.satisfies_condition())
            .map(|hook| (hook.action.clone(), hook.event_block))
            .unzip();

        FetchResult {
            actions,
            event_block: compute_event_block(&event_blocks),
        }
    }
}
