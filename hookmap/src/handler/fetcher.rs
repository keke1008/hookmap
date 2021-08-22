use crate::handler::Handler;
use hookmap_core::{Button, ButtonEvent};
use std::{collections::HashMap, fmt::Debug, sync::Arc};

#[derive(Debug)]
pub(crate) struct VecFetcher<E: Debug> {
    storage: Vec<Arc<Handler<E>>>,
}

impl<E: Debug> VecFetcher<E> {
    pub(crate) fn new(storage: Vec<Arc<Handler<E>>>) -> Self {
        Self { storage }
    }

    pub(crate) fn fetch(&self) -> Vec<Arc<Handler<E>>> {
        self.storage
            .iter()
            .filter(|handler| handler.conditions.is_satisfied())
            .cloned()
            .collect()
    }
}

pub(crate) type MouseFetcher<E> = VecFetcher<E>;

#[derive(Debug)]
pub(crate) struct ButtonFetcher {
    storage: HashMap<Button, VecFetcher<ButtonEvent>>,
}

impl ButtonFetcher {
    pub(crate) fn new(storage: HashMap<Button, Vec<Arc<Handler<ButtonEvent>>>>) -> Self {
        let storage = storage
            .into_iter()
            .map(|(button, handlers)| (button, VecFetcher::new(handlers)))
            .collect();
        Self { storage }
    }

    pub(crate) fn fetch(&self, button: &Button) -> Vec<Arc<Handler<ButtonEvent>>> {
        self.storage
            .get(button)
            .map(VecFetcher::fetch)
            .unwrap_or_default()
    }
}
