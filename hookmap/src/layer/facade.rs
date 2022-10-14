use super::detector::{DetectedLayer, Detector, LayerAction};
use super::relation::{Relation, Relations};
use super::state::{LayerIndex, LayerState};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) struct LayerFacade {
    relations: Relations,
    detector: Detector,
}

impl LayerFacade {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn create_independent_layer(
        &mut self,
        state: &mut LayerState,
        init_state: bool,
    ) -> LayerIndex {
        let layer_index = state.create_layer(init_state);
        self.relations.add_relation(Relation::new(layer_index));
        self.detector.add_layer(layer_index);

        layer_index
    }

    pub(crate) fn create_sync_layer(
        &mut self,
        state: &mut LayerState,
        parent: LayerIndex,
        init_state: bool,
    ) -> LayerIndex {
        let layer_index = state.create_layer(init_state);

        self.detector.add_layer(layer_index);

        let mut relation = Relation::new(layer_index);
        let parent_relation = self.relations.get(parent);
        for ancestor in parent_relation.iter_self_and_ancestors() {
            relation.add_ancestor(ancestor);
            self.detector.add_ancestor(layer_index, ancestor);
        }
        for descendant in parent_relation.iter_descendants() {
            relation.add_descendant(descendant);
            self.detector.add_descendant(layer_index, descendant);
        }

        self.relations.add_relation(relation);

        layer_index
    }

    pub(crate) fn create_child_layer(
        &mut self,
        state: &mut LayerState,
        parent: LayerIndex,
        init_state: bool,
    ) -> LayerIndex {
        let layer_index = state.create_layer(init_state);
        self.relations.add_relation(Relation::new(layer_index));
        self.detector.add_layer(layer_index);

        let parent_relation = self.relations.get(parent).clone();
        for ancestor in parent_relation.iter_self_and_ancestors() {
            self.add_ancestor(layer_index, ancestor);
            self.add_descendant(ancestor, layer_index);
        }

        layer_index
    }

    pub(crate) fn add_ancestor(&mut self, target: LayerIndex, ancestor: LayerIndex) {
        self.relations.get_mut(target).add_ancestor(ancestor);
        self.detector.add_ancestor(target, ancestor);
    }

    pub(crate) fn add_descendant(&mut self, target: LayerIndex, descendant: LayerIndex) {
        self.relations.get_mut(target).add_descendant(descendant);
        self.detector.add_descendant(target, descendant);
    }

    pub(crate) fn is_active(&self, state: &LayerState, index: LayerIndex) -> bool {
        self.relations.get(index).is_active(state)
    }

    pub(crate) fn iter_detected<'a>(
        &'a self,
        state: &'a mut LayerState,
        changed_layer: LayerIndex,
        action: LayerAction,
    ) -> impl Iterator<Item = DetectedLayer> + 'a {
        self.detector
            .iter_detected(state, changed_layer, action, &self.relations)
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::layer::DetectedEvent::{self, *};
    use LayerAction::*;

    use super::*;

    #[test_case(true, true => (true, true);     "enabled_parent_and_enabled_sync")]
    #[test_case(true, false => (true, false);   "enabled_parent_and_disabled_sync")]
    #[test_case(false, true => (false, false);  "disabled_parent_and_enabled_sync")]
    #[test_case(false, false => (false, false); "disabled_parent_and_disable_sync")]
    fn test_parent_and_sync_layer(init_parent: bool, init_child: bool) -> (bool, bool) {
        let mut facade = LayerFacade::new();
        let mut state = LayerState::default();
        let parent = facade.create_independent_layer(&mut state, init_parent);
        let child = facade.create_sync_layer(&mut state, parent, init_child);

        let parent_is_active = facade.is_active(&state, parent);
        let child_is_active = facade.is_active(&state, child);

        (parent_is_active, child_is_active)
    }

    #[test_case(true, true =>  (false, true);    "enabled_parent_and_enabled_child")]
    #[test_case(true, true => (false, true);    "enabled_parent_and_disabled_child")]
    #[test_case(false, true => (false, false);  "disabled_parent_and_enabled_child")]
    #[test_case(false, false => (false, false); "disabled_parent_and_disable_child")]
    fn test_parent_and_child_layer(init_parent: bool, init_child: bool) -> (bool, bool) {
        let mut facade = LayerFacade::new();
        let mut state = LayerState::default();
        let parent = facade.create_independent_layer(&mut state, init_parent);
        let child = facade.create_child_layer(&mut state, parent, init_child);

        let parent_is_active = facade.is_active(&state, parent);
        let child_is_active = facade.is_active(&state, child);

        (parent_is_active, child_is_active)
    }

    fn test_layer_changes(
        action: LayerAction,
        parent_init: bool,
        child_init: bool,
        create_layer_method: fn(&mut LayerFacade, &mut LayerState, LayerIndex, bool) -> LayerIndex,
        changed_layer: fn(LayerIndex, LayerIndex) -> LayerIndex,
    ) -> (Option<DetectedEvent>, Option<DetectedEvent>) {
        let mut facade = LayerFacade::new();
        let mut state = LayerState::default();

        let parent = facade.create_independent_layer(&mut state, parent_init);
        let child = create_layer_method(&mut facade, &mut state, parent, child_init);

        let mut parent_detected = None;
        let mut child_detected = None;

        for detected in facade.iter_detected(&mut state, changed_layer(parent, child), action) {
            let event = match detected.index {
                index if index == parent => &mut parent_detected,
                index if index == child => &mut child_detected,
                index => panic!("Unknown layer: {index:?}"),
            };
            if event.is_some() {
                panic!("Duplicate event occured");
            }
            *event = Some(detected.event);
        }

        (parent_detected, child_detected)
    }

    #[test_case(Enable,  true,  true  => (None, None);                       "enable_enabled_child_with_enabled_parent")]
    #[test_case(Enable,  true,  false => (Some(Inactivate), Some(Activate)); "enable_disabled_child_with_enabled_parent")]
    #[test_case(Enable,  false, true  => (None, None);                       "enable_enabled_child_with_disabled_parent")]
    #[test_case(Enable,  false, false => (None, None);                       "enable_disabled_child_with_disabled_parent")]
    #[test_case(Disable, true,  true  => (Some(Activate), Some(Inactivate)); "disable_enabled_child_with_enabled_parent")]
    #[test_case(Disable, true,  false => (None, None);                       "disable_disabled_child_with_enabled_parent")]
    #[test_case(Disable, false, true  => (None, None);                       "disable_enabled_child_with_disabled_parent")]
    #[test_case(Disable, false, false => (None, None);                       "disable_disabled_child_with_disabled_parent")]
    fn test_child_layer(
        action: LayerAction,
        parent_init: bool,
        child_init: bool,
    ) -> (Option<DetectedEvent>, Option<DetectedEvent>) {
        test_layer_changes(
            action,
            parent_init,
            child_init,
            LayerFacade::create_child_layer,
            |_, c| c,
        )
    }

    #[test_case(Enable,  true,  true  => (None, None);            "enable_enabled_child_with_enabled_parent")]
    #[test_case(Enable,  true,  false => (None, Some(Activate));  "enable_disabled_child_with_enabled_parent")]
    #[test_case(Enable,  false, true  => (None, None);            "enable_enabled_child_with_disabled_parent")]
    #[test_case(Enable,  false, false => (None, None);            "enable_disabled_child_with_disabled_parent")]
    #[test_case(Disable, true,  true  => (None, Some(Inactivate));"disable_enabled_child_with_enabled_parent")]
    #[test_case(Disable, true,  false => (None, None);            "disable_disabled_child_with_enabled_parent")]
    #[test_case(Disable, false, true  => (None, None);            "disable_enabled_child_with_disabled_parent")]
    #[test_case(Disable, false, false => (None, None);            "disable_disabled_child_with_disabled_parent")]
    fn test_sync_layer(
        action: LayerAction,
        parent_init: bool,
        child_init: bool,
    ) -> (Option<DetectedEvent>, Option<DetectedEvent>) {
        test_layer_changes(
            action,
            parent_init,
            child_init,
            LayerFacade::create_sync_layer,
            |_, c| c,
        )
    }

    #[test_case(Enable,  true,  true  => (None, None);             "enable_enabled_parent_with_enabled_child")]
    #[test_case(Enable,  true,  false => (None, None);             "enable_enabled_parent_with_disabled_child")]
    #[test_case(Enable,  false, true  => (None, Some(Activate));   "enable_disabled_parent_with_enabled_child")]
    #[test_case(Enable,  false, false => (Some(Activate), None);   "enable_disabled_parent_with_disabled_child")]
    #[test_case(Disable, true,  true  => (None, Some(Inactivate)); "disable_enabled_parent_with_enabled_child")]
    #[test_case(Disable, true,  false => (Some(Inactivate), None); "disable_enabled_parent_with_disabled_child")]
    #[test_case(Disable, false, true  => (None, None);             "disable_disabled_parent_with_enabled_child")]
    #[test_case(Disable, false, false => (None, None);             "disable_disabled_parent_with_disabled_child")]
    fn test_parent_layer_with_child_layer(
        action: LayerAction,
        parent_init: bool,
        child_init: bool,
    ) -> (Option<DetectedEvent>, Option<DetectedEvent>) {
        test_layer_changes(
            action,
            parent_init,
            child_init,
            LayerFacade::create_child_layer,
            |p, _| p,
        )
    }

    #[test_case(Enable,  true,  true  => (None, None);                         "enable_enabled_parent_with_enabled_child")]
    #[test_case(Enable,  true,  false => (None, None);                         "enable_enabled_parent_with_disabled_child")]
    #[test_case(Enable,  false, true  => (Some(Activate), Some(Activate));     "enable_disabled_parent_with_enabled_child")]
    #[test_case(Enable,  false, false => (Some(Activate), None);               "enable_disabled_parent_with_disabled_child")]
    #[test_case(Disable, true,  true  => (Some(Inactivate), Some(Inactivate)); "disable_enabled_parent_with_enabled_child")]
    #[test_case(Disable, true,  false => (Some(Inactivate), None);             "disable_enabled_parent_with_disabled_child")]
    #[test_case(Disable, false, true  => (None, None);                         "disable_disabled_parent_with_enabled_child")]
    #[test_case(Disable, false, false => (None, None);                         "disable_disabled_parent_with_disabled_child")]
    fn test_parent_layer_with_sync_layer(
        action: LayerAction,
        parent_init: bool,
        child_init: bool,
    ) -> (Option<DetectedEvent>, Option<DetectedEvent>) {
        test_layer_changes(
            action,
            parent_init,
            child_init,
            LayerFacade::create_sync_layer,
            |p, _| p,
        )
    }
}
