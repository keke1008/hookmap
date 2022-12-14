use bitvec::prelude::BitVec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FlagIndex(pub(super) usize);

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) struct FlagState(BitVec);

impl FlagState {
    pub(crate) fn create_flag(&mut self, state: bool) -> FlagIndex {
        self.0.push(state);
        FlagIndex(self.0.len() - 1)
    }

    pub(crate) fn set(&mut self, index: FlagIndex, state: bool) {
        self.0.set(index.0, state);
    }

    pub(crate) fn enable(&mut self, index: FlagIndex) {
        self.set(index, true);
    }

    pub(crate) fn disable(&mut self, index: FlagIndex) {
        self.set(index, false);
    }

    pub(super) fn get(&self, index: FlagIndex) -> bool {
        *self.0.get(index.0).unwrap()
    }

    pub(super) fn as_raw_slice(&self) -> &[usize] {
        self.0.as_raw_slice()
    }
}
