use super::relation::Relations;
use super::state::{LayerIndex, LayerState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LayerAction {
    Enable,
    Disable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DetectedEvent {
    Activate,
    Inactivate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ChangedLayer {
    before_state: bool,
    update_state: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hook {
    target_layer: LayerIndex,
    changed_layer_state_before_change: Option<bool>,
    event_on_detected: DetectedEvent,
}

impl Hook {
    fn detect(
        &self,
        changed_layer: LayerIndex,
        relations: &Relations,
        state: &mut LayerState,
    ) -> bool {
        let relation = relations.get(self.target_layer);

        if let Some(state_before_change) = self.changed_layer_state_before_change {
            let changed_layer_state = state.get(changed_layer);
            if changed_layer_state != state_before_change {
                return false;
            }

            state.set(changed_layer, !state_before_change);
            let detected = relation.is_active(state);
            state.set(changed_layer, changed_layer_state);
            detected
        } else {
            relation.is_active(state)
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(super) struct Detector {
    on_enable: Vec<Vec<Hook>>,
    on_disable: Vec<Vec<Hook>>,
}

impl Detector {
    pub(super) fn add_layer(&mut self, index: LayerIndex) {
        assert_eq!(self.on_enable.len(), self.on_disable.len());
        assert_eq!(self.on_enable.len(), index.0);

        self.on_enable.push(Vec::new());
        self.on_disable.push(Vec::new());

        self.add_ancestor(index, index);
    }

    pub(super) fn add_ancestor(&mut self, target: LayerIndex, ancestor: LayerIndex) {
        self.on_enable[ancestor.0].push(Hook {
            target_layer: target,
            changed_layer_state_before_change: Some(false),
            event_on_detected: DetectedEvent::Activate,
        });

        self.on_disable[ancestor.0].push(Hook {
            target_layer: target,
            changed_layer_state_before_change: None,
            event_on_detected: DetectedEvent::Inactivate,
        });
    }

    pub(super) fn add_descendant(&mut self, target: LayerIndex, descendant: LayerIndex) {
        self.on_enable[descendant.0].push(Hook {
            target_layer: target,
            changed_layer_state_before_change: None,
            event_on_detected: DetectedEvent::Inactivate,
        });

        self.on_disable[descendant.0].push(Hook {
            target_layer: target,
            changed_layer_state_before_change: Some(true),
            event_on_detected: DetectedEvent::Activate,
        });
    }

    pub(super) fn iter_detected<'a>(
        &'a self,
        state: &'a mut LayerState,
        changed_layer: LayerIndex,
        action: LayerAction,
        relations: &'a Relations,
    ) -> impl Iterator<Item = DetectedLayer> + 'a {
        let storage = match action {
            LayerAction::Disable => &self.on_disable,
            LayerAction::Enable => &self.on_enable,
        };

        storage[changed_layer.0]
            .iter()
            .filter(move |hook| hook.detect(changed_layer, relations, state))
            .map(DetectedLayer::new)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DetectedLayer {
    pub(crate) index: LayerIndex,
    pub(crate) event: DetectedEvent,
}

impl DetectedLayer {
    fn new(hook: &Hook) -> Self {
        Self {
            index: hook.target_layer,
            event: hook.event_on_detected,
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;
    use crate::layer::relation::Relation;
    use DetectedEvent::*;
    use LayerAction::*;

    #[test_case(Enable,  true  => None;             "enable_enabled_layer")]
    #[test_case(Enable,  false => Some(Activate);  "enable_disabled_layer")]
    #[test_case(Disable, true  => Some(Inactivate); "disable_enabled_layer")]
    #[test_case(Disable, false =>  None;           "disable_disabled_layer")]
    fn test_single_layer(action: LayerAction, init_state: bool) -> Option<DetectedEvent> {
        let mut state = LayerState::default();
        let mut relations = Relations::default();
        let mut detector = Detector::default();

        let layer = state.create_layer(init_state);
        relations.add_relation(Relation::new(layer));
        detector.add_layer(layer);

        let detected: Vec<_> = detector
            .iter_detected(&mut state, layer, action, &relations)
            .collect();

        assert!(detected.len() <= 1);
        detected.get(0).map(|layer| layer.event)
    }
}
