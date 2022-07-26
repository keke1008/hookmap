use std::hash::Hash;
use std::{collections::HashMap, sync::Arc};

use hookmap_core::button::{Button, ButtonAction};
use hookmap_core::event::{ButtonEvent, CursorEvent, WheelEvent};

use super::hook::{InputHook, LayerHook, RemapHook};
use super::layer::LayerIndex;
use crate::runtime::hook::{self, InputHookStorage, LayerQuery, LayerState, LayerStateUpdate};

fn push_to_hashmap_vec<K: Eq + Hash, V>(map: &mut HashMap<K, Vec<V>>, key: K, value: V) {
    map.entry(key).or_default().push(value);
}

#[derive(Debug, Default, Clone)]
pub(crate) struct LayerHookStorage {
    on_enabled: HashMap<LayerIndex, Vec<Arc<LayerHook>>>,
    on_disabled: HashMap<LayerIndex, Vec<Arc<LayerHook>>>,
}

impl LayerHookStorage {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn register_on_disabled(&mut self, layer_id: LayerIndex, hook: Arc<LayerHook>) {
        push_to_hashmap_vec(&mut self.on_disabled, layer_id, hook);
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
    remap_on_press: HashMap<Button, Vec<Arc<RemapHook>>>,
    remap_on_release: HashMap<Button, Vec<Arc<RemapHook>>>,
    on_press: HashMap<Button, Vec<Arc<InputHook<ButtonEvent>>>>,
    on_release: HashMap<Button, Vec<Arc<InputHook<Option<ButtonEvent>>>>>,
    mouse_cursor: Vec<Arc<InputHook<CursorEvent>>>,
    mouse_wheel: Vec<Arc<InputHook<WheelEvent>>>,
}

impl HotkeyStorage {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn register_remap_on_press(&mut self, button: Button, hook: Arc<RemapHook>) {
        push_to_hashmap_vec(&mut self.remap_on_press, button, hook);
    }

    pub(crate) fn register_remap_on_release(&mut self, button: Button, hook: Arc<RemapHook>) {
        push_to_hashmap_vec(&mut self.remap_on_release, button, hook);
    }

    pub(crate) fn register_on_press(&mut self, button: Button, hook: Arc<InputHook<ButtonEvent>>) {
        push_to_hashmap_vec(&mut self.on_press, button, hook);
    }

    pub(crate) fn register_on_release(
        &mut self,
        button: Button,
        hook: Arc<InputHook<Option<ButtonEvent>>>,
    ) {
        push_to_hashmap_vec(&mut self.on_release, button, hook);
    }

    pub(crate) fn register_mouse_cursor(&mut self, hook: Arc<InputHook<CursorEvent>>) {
        self.mouse_cursor.push(hook);
    }

    pub(crate) fn register_mouse_wheel(&mut self, hook: Arc<InputHook<WheelEvent>>) {
        self.mouse_wheel.push(hook);
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
