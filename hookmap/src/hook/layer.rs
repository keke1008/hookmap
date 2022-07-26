use crate::runtime::hook;

use std::sync::atomic::{AtomicUsize, Ordering};

use bitvec::{bitvec, prelude::BitVec};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct LayerIndex(usize);

impl hook::LayerIdentifier for LayerIndex {}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Layer {
    ancestors_mask: BitVec,
    descendant_mask: BitVec,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) struct LayerState {
    layers: Vec<Layer>,
    state: BitVec<AtomicUsize>,
}

fn set_with_extend(bits: &mut BitVec, index: usize, value: bool) {
    let len = bits.len();

    if len <= index {
        let lack = index - len + 1;
        bits.reserve(len + lack);

        for _ in 0..lack {
            bits.push(false);
        }
    }

    bits.set(index, value);
}

impl LayerState {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn create_root_layer(&mut self, init_state: bool) -> LayerIndex {
        self.state.push(init_state);

        let len = self.state.len();
        let mut enable_mask = bitvec![0; len];

        let root_index = len - 1;
        enable_mask.set(root_index, true);

        self.layers.push(Layer {
            ancestors_mask: enable_mask,
            descendant_mask: BitVec::new(),
        });

        LayerIndex(root_index)
    }

    pub(crate) fn create_inheritance_layer(
        &mut self,
        parent: LayerIndex,
        init_state: bool,
    ) -> LayerIndex {
        self.state.push(init_state);

        let len = self.state.len();
        let new_index = len - 1;

        let mut ancestors_mask = self.layers[parent.0].ancestors_mask.clone();

        set_with_extend(&mut ancestors_mask, new_index, true);

        self.layers.push(Layer {
            ancestors_mask,
            descendant_mask: BitVec::new(),
        });

        LayerIndex(new_index)
    }

    pub(crate) fn create_layer(&mut self, parent: LayerIndex, init_state: bool) -> LayerIndex {
        let new_index = self.create_inheritance_layer(parent, init_state);

        let ancestors_mask = self.layers[parent.0].ancestors_mask.clone();

        for ancestor_index in ancestors_mask.iter_ones() {
            let ancestor = &mut self.layers[ancestor_index];
            set_with_extend(&mut ancestor.descendant_mask, new_index.0, true);
        }

        new_index
    }

    pub(crate) fn iter_ancestors(&self, id: LayerIndex) -> impl Iterator<Item = LayerIndex> + '_ {
        self.layers[id.0].ancestors_mask.iter_ones().map(LayerIndex)
    }
}

impl hook::LayerState for LayerState {
    type LayerIdentifier = LayerIndex;

    fn is_enabled(&self, index: LayerIndex) -> bool {
        let layer = &self.layers[index.0];
        let state = self.state.as_raw_slice();

        state
            .iter()
            .zip(layer.ancestors_mask.as_raw_slice().iter())
            .all(|(state, ancestor)| (ancestor & !state.load(Ordering::Relaxed) == 0))
            && state
                .iter()
                .zip(layer.descendant_mask.as_raw_slice().iter())
                .all(|(state, descendant)| (descendant & state.load(Ordering::Relaxed)) == 0)
    }

    fn update_enable(&self, index: LayerIndex) {
        self.state.set_aliased(index.0, true);
    }

    fn update_disable(&self, index: LayerIndex) {
        self.state.set_aliased(index.0, false);
    }
}

pub(crate) type LayerQuerySender = hook::LayerQuerySender<LayerIndex>;

#[cfg(test)]
mod tests {
    use super::*;
    use hook::LayerState as _;

    fn create_state_and_root(init_state: bool) -> (LayerState, LayerIndex) {
        let mut state = LayerState::new();
        let root = state.create_root_layer(init_state);
        (state, root)
    }

    #[test]
    fn default_enabled_root_layer() {
        let (state, root) = create_state_and_root(true);
        assert!(state.is_enabled(root));
    }

    #[test]
    fn default_disabled_root_layer() {
        let (state, root) = create_state_and_root(false);
        assert!(!state.is_enabled(root));
    }

    #[test]
    fn enabled_root_layer() {
        let (state, root) = create_state_and_root(false);
        state.update_enable(root);
        assert!(state.is_enabled(root));
    }

    #[test]
    fn disabled_root_layer() {
        let (state, root) = create_state_and_root(true);
        state.update_disable(root);
        assert!(!state.is_enabled(root));
    }

    fn test_parent_and_child_layer(
        init_parent: bool,
        init_child: bool,
        parent_enabled: bool,
        child_enabled: bool,
    ) {
        let mut state = LayerState::new();
        let parent = state.create_root_layer(init_parent);
        let child = state.create_layer(parent, init_child);
        assert_eq!(state.is_enabled(parent), parent_enabled);
        assert_eq!(state.is_enabled(child), child_enabled);
    }

    #[test]
    fn enabled_parent_and_enabled_child() {
        test_parent_and_child_layer(true, true, false, true);
    }

    #[test]
    fn enabled_parent_and_disabled_child() {
        test_parent_and_child_layer(true, false, true, false);
    }

    #[test]
    fn disabled_parent_and_enabled_child() {
        test_parent_and_child_layer(false, true, false, false);
    }

    #[test]
    fn disabled_parent_and_disable_child() {
        test_parent_and_child_layer(false, false, false, false);
    }

    fn test_parent_and_inheritance_child_layer(
        init_parent: bool,
        init_child: bool,
        parent_enabled: bool,
        child_enabled: bool,
    ) {
        let mut state = LayerState::new();
        let parent = state.create_root_layer(init_parent);
        let child = state.create_inheritance_layer(parent, init_child);
        assert_eq!(state.is_enabled(parent), parent_enabled);
        assert_eq!(state.is_enabled(child), child_enabled);
    }

    #[test]
    fn enabled_parent_and_enabled_inheritance_child() {
        test_parent_and_inheritance_child_layer(true, true, true, true);
    }

    #[test]
    fn enabled_parent_and_disabled_inheritance_child() {
        test_parent_and_inheritance_child_layer(true, false, true, false);
    }

    #[test]
    fn disabled_parent_and_enabled_inheritance_child() {
        test_parent_and_inheritance_child_layer(false, true, false, false);
    }

    #[test]
    fn disabled_parent_and_disable_inheritance_child() {
        test_parent_and_inheritance_child_layer(false, false, false, false);
    }
}
