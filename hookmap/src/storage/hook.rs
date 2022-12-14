use std::sync::Arc;

use crate::condition::{flag::FlagState, view::View};

#[derive(Debug)]
pub(crate) struct Hook<T> {
    view: Arc<View>,
    action: T,
}

impl<T> Hook<T> {
    pub(crate) fn new(view: Arc<View>, action: T) -> Self {
        Self { view, action }
    }

    pub(crate) fn action(&self) -> &T {
        &self.action
    }

    pub(crate) fn is_runnable(&self, state: &FlagState) -> bool {
        self.view.is_enabled(state)
    }
}
