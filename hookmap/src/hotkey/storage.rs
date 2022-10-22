use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;

use hookmap_core::button::{Button, ButtonAction};
use hookmap_core::event::{ButtonEvent, CursorEvent, WheelEvent};

use crate::condition::detector::{Detector, ViewChange};
use crate::condition::flag::FlagState;
use crate::condition::view::View;

use crate::runtime::hook::{FlagEvent, Hook, HookAction, Procedure};
use crate::runtime::storage::{FlagHookFetcher, InputHookFetcher};

#[derive(Debug, Default)]
pub(super) struct ButtonHookStorage(HashMap<Button, Vec<Hook<ButtonEvent>>>);

impl ButtonHookStorage {
    fn iter_filter_event(&self, event: &ButtonEvent) -> impl Iterator<Item = &Hook<ButtonEvent>> {
        self.0.get(&event.target).into_iter().flatten()
    }

    pub(super) fn register_specific(
        &mut self,
        view: Arc<View>,
        button: Button,
        action: Arc<HookAction<ButtonEvent>>,
    ) {
        self.0
            .entry(button)
            .or_default()
            .push(Hook::new(view, action));
    }
}

#[derive(Debug)]
pub(super) struct MouseHookStorage<E> {
    hooks: Vec<Hook<E>>,
}

impl<E> MouseHookStorage<E> {
    fn fetch(&self, state: &FlagState) -> Vec<Arc<HookAction<E>>> {
        self.hooks
            .iter()
            .filter(|hook| hook.is_runnable(state))
            .map(Hook::action)
            .collect()
    }

    pub(super) fn register(&mut self, view: Arc<View>, action: Arc<HookAction<E>>) {
        self.hooks.push(Hook::new(view, action));
    }
}

impl<E> Default for MouseHookStorage<E> {
    fn default() -> Self {
        Self { hooks: Vec::new() }
    }
}

#[derive(Debug, Default)]
pub(super) struct InputHookStorage {
    pub(super) on_press_exclusive: ButtonHookStorage,
    pub(super) on_release_exclusive: ButtonHookStorage,
    pub(super) on_press: ButtonHookStorage,
    pub(super) on_release: ButtonHookStorage,
    pub(super) mouse_cursor: MouseHookStorage<CursorEvent>,
    pub(super) mouse_wheel: MouseHookStorage<WheelEvent>,
}

impl InputHookFetcher for InputHookStorage {
    fn fetch_exclusive_button_hook(
        &self,
        event: ButtonEvent,
        state: &FlagState,
    ) -> Option<Arc<HookAction<ButtonEvent>>> {
        let storage = match event.action {
            ButtonAction::Press => &self.on_press_exclusive,
            ButtonAction::Release => &self.on_release_exclusive,
        };
        storage
            .iter_filter_event(&event)
            .find(|hook| hook.is_runnable(state))
            .map(Hook::action)
    }

    fn fetch_button_hook(
        &self,
        event: ButtonEvent,
        state: &FlagState,
    ) -> Vec<Arc<HookAction<ButtonEvent>>> {
        let storage = match event.action {
            ButtonAction::Press => &self.on_press,
            ButtonAction::Release => &self.on_release,
        };
        storage
            .iter_filter_event(&event)
            .filter(|hook| hook.is_runnable(state))
            .map(Hook::action)
            .collect()
    }

    fn fetch_mouse_cursor_hook(&self, state: &FlagState) -> Vec<Arc<HookAction<CursorEvent>>> {
        self.mouse_cursor.fetch(state)
    }

    fn fetch_mouse_wheel_hook(&self, state: &FlagState) -> Vec<Arc<HookAction<WheelEvent>>> {
        self.mouse_wheel.fetch(state)
    }
}

type FlagHookAction = Arc<HookAction<ButtonEvent>>;

fn assert_is_procedure_optional(action: &FlagHookAction) {
    if let HookAction::Procedure { procedure, .. } = &**action {
        assert!(matches!(procedure, Procedure::Optional(_)));
    }
}

#[derive(Debug, Clone)]
struct ArcPtrKey<T>(Arc<T>);
impl<T> Hash for ArcPtrKey<T> {
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
pub(super) struct FlagHookStorage {
    on_enabled: HashMap<ArcPtrKey<View>, Vec<FlagHookAction>>,
    on_disabled: HashMap<ArcPtrKey<View>, Vec<FlagHookAction>>,
    detector: Detector,
}

impl FlagHookStorage {
    fn register_view(&mut self, view: &ArcPtrKey<View>) {
        if self.on_enabled.get(view).is_none() && self.on_disabled.get(view).is_none() {
            self.detector.observe(Arc::clone(&view.0));
        }
    }

    pub(super) fn register_on_activated(&mut self, view: Arc<View>, action: FlagHookAction) {
        assert_is_procedure_optional(&action);

        let view = ArcPtrKey(view);
        self.register_view(&view);
        self.on_enabled.entry(view).or_default().push(action);
    }

    pub(super) fn register_on_inactivated(&mut self, view: Arc<View>, action: FlagHookAction) {
        assert_is_procedure_optional(&action);
        let view = ArcPtrKey(view);
        self.register_view(&view);
        self.on_disabled.entry(view).or_default().push(action);
    }
}

impl FlagHookFetcher for FlagHookStorage {
    fn fetch(&self, mut event: FlagEvent) -> Vec<Arc<HookAction<ButtonEvent>>> {
        self.detector
            .iter_detected(&mut event.snapshot, event.flag_index, event.change)
            .flat_map(|detected| {
                let storage = match detected.change {
                    ViewChange::Enabled => &self.on_enabled,
                    ViewChange::Disabled => &self.on_disabled,
                };
                storage
                    .get(&ArcPtrKey(detected.view))
                    .into_iter()
                    .flatten()
                    .map(Arc::clone)
            })
            .collect()
    }
}
