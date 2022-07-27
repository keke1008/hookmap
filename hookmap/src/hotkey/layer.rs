use crate::{
    hook::layer::{LayerIndex, LayerQuerySender},
    runtime::hook::LayerState,
};

#[derive(Debug, Clone)]
pub struct Layer {
    tx: LayerQuerySender,
    layer_id: LayerIndex,
}

impl Layer {
    pub(super) fn new(tx: LayerQuerySender, layer_id: LayerIndex) -> Self {
        Self { tx, layer_id }
    }

    pub(super) fn id(&self) -> LayerIndex {
        self.layer_id
    }

    pub fn enable(&self) {
        self.tx.send(LayerState::Enabled, self.layer_id);
    }

    pub fn disable(&self) {
        self.tx.send(LayerState::Disabled, self.layer_id);
    }
}
