use std::collections::HashMap;
use std::sync::Arc;

use hookmap_core::button::{Button, ButtonAction};
use hookmap_core::event::{ButtonEvent, CursorEvent, WheelEvent};

use crate::layer::{DetectedEvent, LayerAction, LayerFacade, LayerIndex, LayerState};

use crate::runtime::hook::{Hook, HookAction, Procedure};
use crate::runtime::storage::{InputHookFetcher, LayerHookFetcher};

#[derive(Debug, Default)]
pub(super) struct ButtonHookStorage(HashMap<Button, Vec<Hook<ButtonEvent>>>);

impl ButtonHookStorage {
    fn iter_filter_event(&self, event: &ButtonEvent) -> impl Iterator<Item = &Hook<ButtonEvent>> {
        self.0.get(&event.target).into_iter().flatten()
    }

    pub(super) fn register_specific(
        &mut self,
        layer: LayerIndex,
        button: Button,
        action: Arc<HookAction<ButtonEvent>>,
    ) {
        self.0
            .entry(button)
            .or_default()
            .push(Hook::new(layer, action, None));
    }
}

#[derive(Debug)]
pub(super) struct MouseHookStorage<E> {
    hooks: Vec<Hook<E>>,
}

impl<E> MouseHookStorage<E> {
    fn fetch(&self, state: &LayerState, facade: &LayerFacade) -> Vec<Arc<HookAction<E>>> {
        self.hooks
            .iter()
            .filter(|hook| facade.is_active(state, hook.layer_index()))
            .map(Hook::action)
            .collect()
    }

    pub(super) fn register(&mut self, layer: LayerIndex, action: Arc<HookAction<E>>) {
        self.hooks.push(Hook::new(layer, action, None));
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
        state: &LayerState,
        facade: &LayerFacade,
    ) -> Option<Arc<HookAction<ButtonEvent>>> {
        let storage = match event.action {
            ButtonAction::Press => &self.on_press_exclusive,
            ButtonAction::Release => &self.on_release_exclusive,
        };
        storage
            .iter_filter_event(&event)
            .find(|hook| hook.is_runnable(event.target, state, facade))
            .map(Hook::action)
    }

    fn fetch_button_hook(
        &self,
        event: ButtonEvent,
        state: &LayerState,
        facade: &LayerFacade,
    ) -> Vec<Arc<HookAction<ButtonEvent>>> {
        let storage = match event.action {
            ButtonAction::Press => &self.on_press,
            ButtonAction::Release => &self.on_release,
        };
        storage
            .iter_filter_event(&event)
            .filter(|hook| hook.is_runnable(event.target, state, facade))
            .map(Hook::action)
            .collect()
    }

    fn fetch_mouse_cursor_hook(
        &self,
        state: &LayerState,
        facade: &LayerFacade,
    ) -> Vec<Arc<HookAction<CursorEvent>>> {
        self.mouse_cursor.fetch(state, facade)
    }

    fn fetch_mouse_wheel_hook(
        &self,
        state: &LayerState,
        facade: &LayerFacade,
    ) -> Vec<Arc<HookAction<WheelEvent>>> {
        self.mouse_wheel.fetch(state, facade)
    }
}

type LayerHookAction = Arc<HookAction<ButtonEvent>>;

fn assert_is_procedure_optional(action: &LayerHookAction) {
    if let HookAction::Procedure { procedure, .. } = &**action {
        assert!(matches!(procedure, Procedure::Optional(_)));
    }
}

#[derive(Debug, Default)]
pub(super) struct LayerHookStorage {
    on_activated: HashMap<LayerIndex, Vec<LayerHookAction>>,
    on_inactivated: HashMap<LayerIndex, Vec<LayerHookAction>>,
}

impl LayerHookStorage {
    pub(super) fn register_on_activated(&mut self, layer: LayerIndex, action: LayerHookAction) {
        assert_is_procedure_optional(&action);
        self.on_activated.entry(layer).or_default().push(action);
    }
    pub(super) fn register_on_inactivated(&mut self, layer: LayerIndex, action: LayerHookAction) {
        assert_is_procedure_optional(&action);
        self.on_inactivated.entry(layer).or_default().push(action);
    }
}

impl LayerHookFetcher for LayerHookStorage {
    fn fetch(
        &self,
        layer: LayerIndex,
        update: LayerAction,
        mut state: LayerState,
        facade: &LayerFacade,
    ) -> Vec<Arc<HookAction<ButtonEvent>>> {
        facade
            .iter_detected(&mut state, layer, update)
            .flat_map(|detected| {
                let storage = match detected.event {
                    DetectedEvent::Activate => &self.on_activated,
                    DetectedEvent::Inactivate => &self.on_inactivated,
                };
                storage
                    .get(&detected.index)
                    .into_iter()
                    .flatten()
                    .map(Arc::clone)
            })
            .collect()
    }
}
