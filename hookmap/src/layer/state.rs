use bitvec::prelude::BitVec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LayerIndex(pub(super) usize);

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) struct LayerState(BitVec);

impl LayerState {
    pub(crate) fn set(&mut self, index: LayerIndex, state: bool) {
        self.0.set(index.0, state);
    }

    pub(crate) fn enable(&mut self, index: LayerIndex) {
        self.set(index, true);
    }

    pub(crate) fn disable(&mut self, index: LayerIndex) {
        self.set(index, false);
    }

    pub(super) fn get(&self, index: LayerIndex) -> bool {
        *self.0.get(index.0).unwrap()
    }

    pub(super) fn create_layer(&mut self, state: bool) -> LayerIndex {
        self.0.push(state);
        LayerIndex(self.0.len() - 1)
    }

    pub(super) fn as_raw_slice(&self) -> &[usize] {
        self.0.as_raw_slice()
    }
}
