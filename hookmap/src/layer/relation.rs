use bitvec::prelude::BitVec;

use super::state::{LayerIndex, LayerState};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Relation {
    self_and_ancestors: BitVec,
    descendants: BitVec,
}

fn set_with_extend(target: &mut BitVec, index: usize, state: bool) {
    let len = target.len();

    if len <= index {
        let lack = index - len + 1;
        target.reserve(len + lack);

        for _ in 0..lack {
            target.push(false);
        }
    }

    target.set(index, state);
}

impl Relation {
    pub(super) fn new(index: LayerIndex) -> Self {
        let mut self_and_ancestors = BitVec::new();
        set_with_extend(&mut self_and_ancestors, index.0, true);

        Relation {
            self_and_ancestors,
            descendants: BitVec::new(),
        }
    }

    pub(super) fn add_ancestor(&mut self, ancestor: LayerIndex) {
        set_with_extend(&mut self.self_and_ancestors, ancestor.0, true);
    }

    pub(super) fn add_descendant(&mut self, descendant: LayerIndex) {
        set_with_extend(&mut self.descendants, descendant.0, true);
    }

    pub(super) fn iter_self_and_ancestors(&self) -> impl Iterator<Item = LayerIndex> + '_ {
        self.self_and_ancestors.iter_ones().map(LayerIndex)
    }

    pub(super) fn iter_descendants(&self) -> impl Iterator<Item = LayerIndex> + '_ {
        self.descendants.iter_ones().map(LayerIndex)
    }

    pub(super) fn is_active(&self, enabled_layer: &LayerState) -> bool {
        self.self_and_ancestors
            .as_raw_slice()
            .iter()
            .zip(enabled_layer.as_raw_slice())
            .all(|(&ancestor, state)| ancestor & state == ancestor)
            && self
                .descendants
                .as_raw_slice()
                .iter()
                .zip(enabled_layer.as_raw_slice())
                .all(|(&descendant, state)| descendant & state == 0)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(super) struct Relations(Vec<Relation>);

impl Relations {
    pub(super) fn add_relation(&mut self, relation: Relation) {
        self.0.push(relation);
    }

    pub(super) fn get(&self, index: LayerIndex) -> &Relation {
        &self.0[index.0]
    }

    pub(super) fn get_mut(&mut self, index: LayerIndex) -> &mut Relation {
        &mut self.0[index.0]
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case(true)]
    #[test_case(false)]
    fn test_single_layer_activation(init_state: bool) {
        let mut state = LayerState::default();
        let layer = state.create_layer(init_state);
        let relation = Relation::new(layer);

        assert_eq!(relation.is_active(&mut state), init_state);
    }

    #[test_case(true, true)]
    #[test_case(true, false)]
    #[test_case(false, true)]
    #[test_case(false, false)]
    fn test_independent_two_layers_activation(state1: bool, state2: bool) {
        let mut state = LayerState::default();
        let layer1 = state.create_layer(state1);
        let relation1 = Relation::new(layer1);
        let layer2 = state.create_layer(state2);
        let relation2 = Relation::new(layer2);

        assert_eq!(relation1.is_active(&state), state1);
        assert_eq!(relation2.is_active(&state), state2);
    }

    #[test_case(true, true => (false, true))]
    #[test_case(true, false => (true, false))]
    #[test_case(false, true => (false, false))]
    #[test_case(false, false => (false, false))]
    fn test_parent_and_child_layers_activation(
        parent_state: bool,
        child_state: bool,
    ) -> (bool, bool) {
        let mut state = LayerState::default();
        let parent = state.create_layer(parent_state);
        let mut parent_relation = Relation::new(parent);
        let child = state.create_layer(child_state);
        let mut child_relation = Relation::new(child);

        parent_relation.add_descendant(child);
        child_relation.add_ancestor(parent);

        let parent_active = parent_relation.is_active(&state);
        let child_active = child_relation.is_active(&state);

        (parent_active, child_active)
    }
}
