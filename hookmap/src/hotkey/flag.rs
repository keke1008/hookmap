use std::sync::mpsc::SyncSender;
use std::sync::{Arc, Mutex};

use hookmap_core::event::ButtonEvent;

use crate::condition::detector::FlagChange;
use crate::condition::flag::{FlagIndex, FlagState};
use crate::runtime::hook::FlagEvent;

#[derive(Debug, Clone)]
pub struct Flag {
    index: FlagIndex,
    state: Arc<Mutex<FlagState>>,
    tx: SyncSender<FlagEvent>,
}

impl Flag {
    pub(crate) fn new(
        index: FlagIndex,
        state: Arc<Mutex<FlagState>>,
        tx: SyncSender<FlagEvent>,
    ) -> Self {
        Self { index, state, tx }
    }

    pub(super) fn index(&self) -> FlagIndex {
        self.index
    }

    fn send(&self, change: FlagChange, inherited_event: Option<ButtonEvent>) {
        let mut state = self.state.lock().unwrap();
        let snapshot = state.clone();
        match change {
            FlagChange::Enabled => state.enable(self.index),
            FlagChange::Disabled => state.disable(self.index),
        };

        let event = FlagEvent {
            flag_index: self.index,
            change,
            snapshot,
            inherited_event,
        };

        self.tx.send(event).unwrap();
    }

    pub fn enable(&self) {
        self.send(FlagChange::Enabled, None);
    }

    pub fn disable(&self) {
        self.send(FlagChange::Disabled, None);
    }

    pub fn enable_with_event(&self, inherited_event: Option<ButtonEvent>) {
        self.send(FlagChange::Enabled, inherited_event);
    }

    pub fn disable_with_event(&self, inherited_event: Option<ButtonEvent>) {
        self.send(FlagChange::Disabled, inherited_event);
    }
}

impl From<&Flag> for FlagIndex {
    fn from(flag: &Flag) -> Self {
        flag.index()
    }
}
