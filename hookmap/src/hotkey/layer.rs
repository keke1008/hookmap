use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};

use hookmap_core::event::ButtonEvent;

use crate::layer::{LayerAction, LayerFacade, LayerIndex, LayerState};
use crate::runtime::hook::LayerEvent;

#[derive(Debug, Clone)]
pub struct Layer {
    index: LayerIndex,
    state: Arc<Mutex<LayerState>>,
    tx: Sender<LayerEvent>,
}

impl Layer {
    pub(crate) fn new(
        index: LayerIndex,
        state: Arc<Mutex<LayerState>>,
        tx: Sender<LayerEvent>,
    ) -> Self {
        Self { index, state, tx }
    }

    pub(super) fn index(&self) -> LayerIndex {
        self.index
    }

    fn send(&self, action: LayerAction, inherited_event: Option<ButtonEvent>) {
        let mut state = self.state.lock().unwrap();
        match action {
            LayerAction::Disable => state.disable(self.index),
            LayerAction::Enable => state.enable(self.index),
        }

        let event = LayerEvent {
            action,
            layer: self.index,
            snapshot: state.clone(),
            inherited_event,
        };

        self.tx.send(event).unwrap();
    }

    pub fn enable(&self) {
        self.send(LayerAction::Enable, None);
    }

    pub fn disable(&self) {
        self.send(LayerAction::Disable, None);
    }

    pub fn enable_with_event(&self, inherited_event: Option<ButtonEvent>) {
        self.send(LayerAction::Enable, inherited_event);
    }

    pub fn disable_with_event(&self, inherited_event: Option<ButtonEvent>) {
        self.send(LayerAction::Disable, inherited_event);
    }
}

#[derive(Debug)]
pub(super) struct LayerCreator {
    facade: LayerFacade,
    state: Arc<Mutex<LayerState>>,
    tx: Sender<LayerEvent>,
    rx: Receiver<LayerEvent>,
}

impl Default for LayerCreator {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            facade: LayerFacade::new(),
            state: Arc::default(),
            tx,
            rx,
        }
    }
}

impl LayerCreator {
    pub(super) fn new() -> Self {
        Self::default()
    }

    pub(super) fn wrap_layer(&self, index: LayerIndex) -> Layer {
        Layer::new(index, Arc::clone(&self.state), self.tx.clone())
    }

    pub(super) fn create_child_layer(
        &mut self,
        parent: LayerIndex,
        init_state: bool,
    ) -> LayerIndex {
        self.facade
            .create_child_layer(&mut *self.state.lock().unwrap(), parent, init_state)
    }

    pub(super) fn create_sync_layer(&mut self, parent: LayerIndex, init_state: bool) -> LayerIndex {
        self.facade
            .create_sync_layer(&mut *self.state.lock().unwrap(), parent, init_state)
    }

    pub(super) fn create_independent_layer(&mut self, init_state: bool) -> LayerIndex {
        self.facade
            .create_independent_layer(&mut *self.state.lock().unwrap(), init_state)
    }

    pub(super) fn destruct(
        self,
    ) -> (
        LayerFacade,
        Arc<Mutex<LayerState>>,
        Sender<LayerEvent>,
        Receiver<LayerEvent>,
    ) {
        (self.facade, self.state, self.tx, self.rx)
    }
}
