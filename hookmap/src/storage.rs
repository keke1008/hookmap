pub(crate) mod action;
mod hook;
mod procedure;

use std::collections::HashMap;
use std::sync::Arc;

use hookmap_core::event::{ButtonEvent, CursorEvent, WheelEvent};

use crate::condition::detector::{Detector, FlagChange, ViewChange};
use crate::condition::flag::{FlagIndex, FlagState};
use crate::condition::view::View;

use action::HookAction;
use hook::Hook;
use procedure::{OptionalProcedure, Procedure};

fn runnables<'a, T>(
    hooks: &'a [Hook<T>],
    state: &'a FlagState,
) -> impl Iterator<Item = Arc<T>> + 'a {
    hooks
        .iter()
        .filter(|hook| hook.is_runnable(state))
        .map(Hook::action)
}

#[derive(Debug)]
pub(crate) struct InputHooks<T, E> {
    actions: Vec<Hook<T>>,
    procedures: Vec<Hook<Procedure<E>>>,
}

impl<T, E> Default for InputHooks<T, E> {
    fn default() -> Self {
        Self {
            actions: Vec::new(),
            procedures: Vec::new(),
        }
    }
}

impl<T, E> InputHooks<T, E> {
    pub(crate) fn add_action(&mut self, view: Arc<View>, action: T) {
        self.actions.push(Hook::new(view, Arc::new(action)));
    }

    pub(crate) fn add_procedure(&mut self, view: Arc<View>, procedure: Procedure<E>) {
        self.procedures.push(Hook::new(view, Arc::new(procedure)));
    }

    pub(crate) fn filter(&self, state: &FlagState) -> (Vec<Arc<T>>, Vec<Arc<Procedure<E>>>) {
        (
            runnables(&self.actions, state).collect(),
            runnables(&self.procedures, state).collect(),
        )
    }

    pub(crate) fn find(&self, state: &FlagState) -> (Option<Arc<T>>, Option<Arc<Procedure<E>>>) {
        (
            runnables(&self.actions, state).next(),
            runnables(&self.procedures, state).next(),
        )
    }
}

#[derive(Debug, Default)]
pub(crate) struct InputHookStorage {
    pub(crate) remap_on_press: InputHooks<HookAction, ButtonEvent>,
    pub(crate) remap_on_release: InputHooks<HookAction, ButtonEvent>,
    pub(crate) on_press: InputHooks<HookAction, ButtonEvent>,
    pub(crate) on_release: InputHooks<HookAction, ButtonEvent>,
    pub(crate) mouse_cursor: InputHooks<HookAction, CursorEvent>,
    pub(crate) mouse_wheel: InputHooks<HookAction, WheelEvent>,
}

#[derive(Debug, Clone)]
struct ArcPtrKey<T>(Arc<T>);
impl<T> std::hash::Hash for ArcPtrKey<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Arc::as_ptr(&self.0).hash(state);
    }
}
impl<T> PartialEq for ArcPtrKey<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}
impl<T> Eq for ArcPtrKey<T> {}

#[derive(Debug, Default)]
struct ViewHooks {
    actions: Vec<Arc<HookAction>>,
    procedures: Vec<Arc<Procedure<ButtonEvent>>>,
}

impl ViewHooks {
    pub(crate) fn iter(
        &self,
    ) -> (
        impl Iterator<Item = Arc<HookAction>> + '_,
        impl Iterator<Item = Arc<Procedure<ButtonEvent>>> + '_,
    ) {
        (
            self.actions.iter().cloned(),
            self.procedures.iter().cloned(),
        )
    }
}

#[derive(Debug, Default)]
pub(crate) struct ViewHookStorage {
    on_enabled: HashMap<ArcPtrKey<View>, ViewHooks>,
    on_disabled: HashMap<ArcPtrKey<View>, ViewHooks>,
    detector: Detector,
}

impl ViewHookStorage {
    fn register_view(&mut self, view: &ArcPtrKey<View>) {
        if self.on_enabled.get(view).is_none() && self.on_disabled.get(view).is_none() {
            self.detector.observe(Arc::clone(&view.0));
        }
    }

    pub(crate) fn add_action_on_enabled(&mut self, view: Arc<View>, action: HookAction) {
        let view = ArcPtrKey(view);
        self.register_view(&view);
        self.on_enabled
            .entry(view)
            .or_default()
            .actions
            .push(Arc::new(action));
    }

    pub(crate) fn add_procedure_on_enabled(
        &mut self,
        view: Arc<View>,
        procedure: OptionalProcedure<ButtonEvent>,
    ) {
        let view = ArcPtrKey(view);
        self.register_view(&view);
        self.on_enabled
            .entry(view)
            .or_default()
            .procedures
            .push(Arc::new(Procedure::Optional(procedure)));
    }

    pub(crate) fn add_action_on_disabled(&mut self, view: Arc<View>, action: HookAction) {
        let view = ArcPtrKey(view);
        self.register_view(&view);
        self.on_disabled
            .entry(view)
            .or_default()
            .actions
            .push(Arc::new(action));
    }

    pub(crate) fn add_procedure_on_disabled(
        &mut self,
        view: Arc<View>,
        procedure: OptionalProcedure<ButtonEvent>,
    ) {
        let view = ArcPtrKey(view);
        self.register_view(&view);
        self.on_disabled
            .entry(view)
            .or_default()
            .procedures
            .push(Arc::new(Procedure::Optional(procedure)));
    }

    pub(crate) fn fetch(
        &self,
        mut snapshot: FlagState,
        flag_index: FlagIndex,
        flag_change: FlagChange,
    ) -> (Vec<Arc<HookAction>>, Vec<Arc<Procedure<ButtonEvent>>>) {
        let (mut acc_actions, mut acc_procedures): (Vec<_>, Vec<_>) = Default::default();

        self.detector
            .iter_detected(&mut snapshot, flag_index, flag_change)
            .flat_map(|detected| {
                let storage = match detected.change {
                    ViewChange::Enabled => &self.on_enabled,
                    ViewChange::Disabled => &self.on_disabled,
                };
                storage.get(&ArcPtrKey(detected.view))
            })
            .map(ViewHooks::iter)
            .for_each(|(actions, procedures)| {
                acc_actions.extend(actions);
                acc_procedures.extend(procedures);
            });

        (acc_actions, acc_procedures)
    }
}
