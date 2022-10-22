use std::sync::Arc;

use super::{
    flag::{FlagIndex, FlagState},
    view::View,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FlagChange {
    Enabled,
    Disabled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ViewChange {
    Enabled,
    Disabled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Observer {
    view: Arc<View>,
    view_change: ViewChange,
    flag_before_change: Option<bool>,
}

impl Observer {
    fn detect(&self, changed_flag: FlagIndex, state: &mut FlagState) -> bool {
        if let Some(flag_before_change) = self.flag_before_change {
            let previous_flag_state = state.get(changed_flag);
            if previous_flag_state != flag_before_change {
                return false;
            }

            state.set(changed_flag, !flag_before_change);
            let detected = self.view.is_enabled(state);
            state.set(changed_flag, previous_flag_state);

            detected
        } else {
            self.view.is_enabled(state)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DetectedView {
    pub(crate) view: Arc<View>,
    pub(crate) change: ViewChange,
}

impl From<&Observer> for DetectedView {
    fn from(observers: &Observer) -> Self {
        Self {
            view: Arc::clone(&observers.view),
            change: observers.view_change,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) struct Detector {
    on_enable: Vec<Vec<Observer>>,
    on_disabled: Vec<Vec<Observer>>,
}

impl Detector {
    fn resize(&mut self, flag: FlagIndex) {
        if self.on_enable.len() <= flag.0 {
            self.on_enable.resize_with(flag.0 + 1, Vec::new);
        }
        if self.on_disabled.len() <= flag.0 {
            self.on_disabled.resize_with(flag.0 + 1, Vec::new);
        }
    }

    pub(crate) fn observe(&mut self, view: Arc<View>) {
        for flag in view.iter_enabled_flags() {
            self.resize(flag);
            self.on_enable[flag.0].push(Observer {
                view: Arc::clone(&view),
                view_change: ViewChange::Enabled,
                flag_before_change: Some(false),
            });
            self.on_disabled[flag.0].push(Observer {
                view: Arc::clone(&view),
                view_change: ViewChange::Disabled,
                flag_before_change: None,
            });
        }

        for flag in view.iter_disabled_flags() {
            self.resize(flag);
            self.on_enable[flag.0].push(Observer {
                view: Arc::clone(&view),
                view_change: ViewChange::Disabled,
                flag_before_change: None,
            });
            self.on_disabled[flag.0].push(Observer {
                view: Arc::clone(&view),
                view_change: ViewChange::Enabled,
                flag_before_change: Some(true),
            });
        }
    }

    pub(crate) fn iter_detected<'a>(
        &'a self,
        state: &'a mut FlagState,
        changed_flag: FlagIndex,
        flag_change: FlagChange,
    ) -> impl Iterator<Item = DetectedView> + 'a {
        let observers = match flag_change {
            FlagChange::Enabled => &self.on_enable,
            FlagChange::Disabled => &self.on_disabled,
        };

        observers
            .get(changed_flag.0)
            .into_iter()
            .flatten()
            .filter(move |observer| observer.detect(changed_flag, state))
            .map(DetectedView::from)
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::condition::view::ViewBuilder;

    use super::*;

    use FlagChange::*;

    #[test_case(true,  Enabled  => None)]
    #[test_case(true,  Disabled => Some(ViewChange::Disabled))]
    #[test_case(false, Enabled  => Some(ViewChange::Enabled))]
    #[test_case(false, Disabled => None)]
    fn enabled_flag(flag_state: bool, flag_change: FlagChange) -> Option<ViewChange> {
        let mut state = FlagState::default();
        let mut detector = Detector::default();

        let flag = state.create_flag(flag_state);
        let view = Arc::new(ViewBuilder::new().enabled(flag).build());
        detector.observe(Arc::clone(&view));

        let detected: Vec<_> = detector
            .iter_detected(&mut state, flag, flag_change)
            .collect();
        assert!(detected.len() <= 1);
        detected.get(0).map(|v| {
            assert!(Arc::ptr_eq(&view, &v.view));
            v.change
        })
    }

    #[test_case(true,  Enabled  => None)]
    #[test_case(true,  Disabled => Some(ViewChange::Enabled))]
    #[test_case(false, Enabled  => Some(ViewChange::Disabled))]
    #[test_case(false, Disabled => None)]
    fn disabled_flag(flag_state: bool, flag_change: FlagChange) -> Option<ViewChange> {
        let mut state = FlagState::default();
        let mut detector = Detector::default();

        let flag = state.create_flag(flag_state);
        let view = Arc::new(ViewBuilder::new().disabled(flag).build());
        detector.observe(Arc::clone(&view));

        let detected: Vec<_> = detector
            .iter_detected(&mut state, flag, flag_change)
            .collect();
        assert!(detected.len() <= 1);
        detected.get(0).map(|v| {
            assert!(Arc::ptr_eq(&view, &v.view));
            v.change
        })
    }

    #[test_case(true,  ViewBuilder::enabled,  true,  ViewBuilder::enabled,  Enabled  => None)]
    #[test_case(true,  ViewBuilder::enabled,  true,  ViewBuilder::enabled,  Disabled => Some(ViewChange::Disabled))]
    #[test_case(true,  ViewBuilder::enabled,  true,  ViewBuilder::disabled, Enabled  => None)]
    #[test_case(true,  ViewBuilder::enabled,  true,  ViewBuilder::disabled, Disabled => None)]
    #[test_case(true,  ViewBuilder::enabled,  false, ViewBuilder::enabled,  Enabled  => None)]
    #[test_case(true,  ViewBuilder::enabled,  false, ViewBuilder::enabled,  Disabled => None)]
    #[test_case(true,  ViewBuilder::enabled,  false, ViewBuilder::disabled, Enabled  => None)]
    #[test_case(true,  ViewBuilder::enabled,  false, ViewBuilder::disabled, Disabled => Some(ViewChange::Disabled))]
    #[test_case(true,  ViewBuilder::disabled, true,  ViewBuilder::enabled,  Enabled  => None)]
    #[test_case(true,  ViewBuilder::disabled, true,  ViewBuilder::enabled,  Disabled => Some(ViewChange::Enabled))]
    #[test_case(true,  ViewBuilder::disabled, true,  ViewBuilder::disabled, Enabled  => None)]
    #[test_case(true,  ViewBuilder::disabled, true,  ViewBuilder::disabled, Disabled => None)]
    #[test_case(true,  ViewBuilder::disabled, false, ViewBuilder::enabled,  Enabled  => None)]
    #[test_case(true,  ViewBuilder::disabled, false, ViewBuilder::enabled,  Disabled => None)]
    #[test_case(true,  ViewBuilder::disabled, false, ViewBuilder::disabled, Enabled  => None)]
    #[test_case(true,  ViewBuilder::disabled, false, ViewBuilder::disabled, Disabled => Some(ViewChange::Enabled))]
    #[test_case(false, ViewBuilder::enabled,  true,  ViewBuilder::enabled,  Enabled  => Some(ViewChange::Enabled))]
    #[test_case(false, ViewBuilder::enabled,  true,  ViewBuilder::enabled,  Disabled => None)]
    #[test_case(false, ViewBuilder::enabled,  true,  ViewBuilder::disabled, Enabled  => None)]
    #[test_case(false, ViewBuilder::enabled,  true,  ViewBuilder::disabled, Disabled => None)]
    #[test_case(false, ViewBuilder::enabled,  false, ViewBuilder::enabled,  Enabled  => None)]
    #[test_case(false, ViewBuilder::enabled,  false, ViewBuilder::enabled,  Disabled => None)]
    #[test_case(false, ViewBuilder::enabled,  false, ViewBuilder::disabled, Enabled  => Some(ViewChange::Enabled))]
    #[test_case(false, ViewBuilder::enabled,  false, ViewBuilder::disabled, Disabled => None)]
    #[test_case(false, ViewBuilder::disabled, true,  ViewBuilder::enabled,  Enabled  => Some(ViewChange::Disabled))]
    #[test_case(false, ViewBuilder::disabled, true,  ViewBuilder::enabled,  Disabled => None)]
    #[test_case(false, ViewBuilder::disabled, true,  ViewBuilder::disabled, Enabled  => None)]
    #[test_case(false, ViewBuilder::disabled, true,  ViewBuilder::disabled, Disabled => None)]
    #[test_case(false, ViewBuilder::disabled, false, ViewBuilder::enabled,  Enabled  => None)]
    #[test_case(false, ViewBuilder::disabled, false, ViewBuilder::enabled,  Disabled => None)]
    #[test_case(false, ViewBuilder::disabled, false, ViewBuilder::disabled, Enabled  => Some(ViewChange::Disabled))]
    #[test_case(false, ViewBuilder::disabled, false, ViewBuilder::disabled, Disabled => None)]
    fn two_flags(
        f1: bool,
        f1_register: fn(ViewBuilder, FlagIndex) -> ViewBuilder,
        f2: bool,
        f2_register: fn(ViewBuilder, FlagIndex) -> ViewBuilder,
        f1_change: FlagChange,
    ) -> Option<ViewChange> {
        let mut state = FlagState::default();
        let mut detector = Detector::default();

        let f1 = state.create_flag(f1);
        let f2 = state.create_flag(f2);

        let view = ViewBuilder::new();
        let view = f1_register(view, f1);
        let view = f2_register(view, f2);
        let view = Arc::new(view.build());
        detector.observe(Arc::clone(&view));

        let detected: Vec<_> = detector.iter_detected(&mut state, f1, f1_change).collect();
        assert!(detected.len() <= 1);
        detected.get(0).map(|v| {
            assert!(Arc::ptr_eq(&view, &v.view));
            v.change
        })
    }
}
