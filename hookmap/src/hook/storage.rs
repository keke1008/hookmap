use std::{collections::HashMap, sync::Arc};

use hookmap_core::button::{Button, ButtonAction};
use hookmap_core::event::{ButtonEvent, CursorEvent, WheelEvent};

use super::hook::{InputHook, LayerHook, RemapHook};
use super::layer::LayerIndex;
use crate::runtime::hook::{self, InputHookStorage, LayerQuery, LayerState, LayerStateUpdate};

#[derive(Debug, Default, Clone)]
pub(crate) struct LayerHookStorage {
    pub(crate) on_enabled: HashMap<LayerIndex, Vec<Arc<LayerHook>>>,
    pub(crate) on_disabled: HashMap<LayerIndex, Vec<Arc<LayerHook>>>,
}

impl LayerHookStorage {
    pub(crate) fn new() -> Self {
        Self::default()
    }
}

impl<S> hook::LayerHookStrage<S> for LayerHookStorage
where
    S: LayerState<LayerIdentifier = LayerIndex>,
{
    type LayerIdentifier = LayerIndex;
    type Hook = Arc<LayerHook>;

    fn fetch(&self, query: &LayerQuery<Self::LayerIdentifier>, state: &S) -> Vec<Self::Hook> {
        let hooks = match query.update {
            LayerStateUpdate::Enabled => &self.on_enabled,
            LayerStateUpdate::Disabled => &self.on_disabled,
        };

        if let Some(hooks) = hooks.get(&query.id) {
            hooks
                .iter()
                .filter(|hook| state.is_enabled(hook.id()))
                .map(Arc::clone)
                .collect()
        } else {
            Vec::new()
        }
    }
}

#[derive(Debug, Default, Clone)]
pub(crate) struct HotkeyStorage {
    pub(crate) remap_on_press: HashMap<Button, Vec<Arc<RemapHook>>>,
    pub(crate) remap_on_release: HashMap<Button, Vec<Arc<RemapHook>>>,
    pub(crate) on_press: HashMap<Button, Vec<Arc<InputHook<ButtonEvent>>>>,
    pub(crate) on_release: HashMap<Button, Vec<Arc<InputHook<Option<ButtonEvent>>>>>,
    pub(crate) mouse_cursor: Vec<Arc<InputHook<CursorEvent>>>,
    pub(crate) mouse_wheel: Vec<Arc<InputHook<WheelEvent>>>,
}

impl HotkeyStorage {
    pub(crate) fn new() -> Self {
        Self::default()
    }
}

impl<S: LayerState<LayerIdentifier = LayerIndex>> InputHookStorage<S> for HotkeyStorage {
    type LayerIdentifier = LayerIndex;

    type RemapHook = Arc<RemapHook>;
    type OnPressHook = Arc<InputHook<ButtonEvent>>;
    type OnReleaseHook = Arc<InputHook<Option<ButtonEvent>>>;
    type MouseCursorHook = Arc<InputHook<CursorEvent>>;
    type MouseWheelHook = Arc<InputHook<WheelEvent>>;

    fn fetch_remap_hook(&self, event: ButtonEvent, state: &S) -> Option<Self::RemapHook> {
        let storage = match event.action {
            ButtonAction::Press => &self.remap_on_press,
            ButtonAction::Release => &self.remap_on_release,
        };
        storage
            .get(&event.target)?
            .iter()
            .find(|hook| state.is_enabled(hook.id()))
            .cloned()
    }

    fn fetch_on_press_hook(&self, event: ButtonEvent, state: &S) -> Vec<Self::OnPressHook> {
        if let Some(hooks) = self.on_press.get(&event.target) {
            hooks
                .iter()
                .filter(|hook| state.is_enabled(hook.id()))
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }

    fn fetch_on_release_hook(&self, event: ButtonEvent, state: &S) -> Vec<Self::OnReleaseHook> {
        if let Some(hooks) = self.on_release.get(&event.target) {
            hooks
                .iter()
                .filter(|hook| state.is_enabled(hook.id()))
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }

    fn fetch_mouse_cursor_hook(&self, _: CursorEvent, state: &S) -> Vec<Self::MouseCursorHook> {
        self.mouse_cursor
            .iter()
            .filter(|hook| state.is_enabled(hook.id()))
            .cloned()
            .collect()
    }

    fn fetch_mouse_wheel_hook(&self, _: WheelEvent, state: &S) -> Vec<Self::MouseWheelHook> {
        self.mouse_wheel
            .iter()
            .filter(|hook| state.is_enabled(hook.id()))
            .cloned()
            .collect()
    }
}
