use std::hash::Hash;
use std::{collections::HashMap, sync::Arc};

use hookmap_core::button::{Button, ButtonAction};
use hookmap_core::event::{ButtonEvent, CursorEvent, WheelEvent};

use super::hook::Hook;
use super::layer::LayerIndex;
use crate::runtime::hook::{self, InputHookStorage, LayerCollection, LayerQuery, LayerState};

fn push_to_hashmap_vec<K: Eq + Hash, V>(map: &mut HashMap<K, Vec<V>>, key: K, value: V) {
    map.entry(key).or_default().push(value);
}

type ButtonHook = Arc<Hook<ButtonEvent>>;
type OptionalButtonHook = Arc<Hook<Option<ButtonEvent>>>;
type CursorHook = Arc<Hook<CursorEvent>>;
type WheelHook = Arc<Hook<WheelEvent>>;

#[derive(Debug, Default, Clone)]
pub(crate) struct LayerHookStorage {
    on_enabled: HashMap<LayerIndex, Vec<OptionalButtonHook>>,
    on_disabled: HashMap<LayerIndex, Vec<OptionalButtonHook>>,
}

impl LayerHookStorage {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn register_on_disabled(&mut self, layer_id: LayerIndex, hook: OptionalButtonHook) {
        push_to_hashmap_vec(&mut self.on_disabled, layer_id, hook);
    }
}

impl<S> hook::LayerHookStrage<S> for LayerHookStorage
where
    S: LayerCollection<LayerIdentifier = LayerIndex>,
{
    type LayerIdentifier = LayerIndex;
    type Hook = OptionalButtonHook;

    fn fetch(&self, query: &LayerQuery<Self::LayerIdentifier>, state: &S) -> Vec<Self::Hook> {
        let hooks = match query.update {
            LayerState::Enabled => &self.on_enabled,
            LayerState::Disabled => &self.on_disabled,
        };

        if let Some(hooks) = hooks.get(&query.id) {
            hooks
                .iter()
                .filter(|hook| state.is_enabled(hook.layer_id()))
                .map(Arc::clone)
                .collect()
        } else {
            Vec::new()
        }
    }
}

#[derive(Debug, Default, Clone)]
pub(crate) struct HotkeyStorage {
    remap_on_press: HashMap<Button, Vec<OptionalButtonHook>>,
    remap_on_release: HashMap<Button, Vec<OptionalButtonHook>>,
    on_press: HashMap<Button, Vec<ButtonHook>>,
    on_release: HashMap<Button, Vec<OptionalButtonHook>>,
    mouse_cursor: Vec<CursorHook>,
    mouse_wheel: Vec<WheelHook>,
}

impl HotkeyStorage {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn register_remap_on_press(&mut self, button: Button, hook: OptionalButtonHook) {
        push_to_hashmap_vec(&mut self.remap_on_press, button, hook);
    }

    pub(crate) fn register_remap_on_release(&mut self, button: Button, hook: OptionalButtonHook) {
        push_to_hashmap_vec(&mut self.remap_on_release, button, hook);
    }

    pub(crate) fn register_on_press(&mut self, button: Button, hook: ButtonHook) {
        push_to_hashmap_vec(&mut self.on_press, button, hook);
    }

    pub(crate) fn register_on_release(&mut self, button: Button, hook: OptionalButtonHook) {
        push_to_hashmap_vec(&mut self.on_release, button, hook);
    }

    pub(crate) fn register_mouse_cursor(&mut self, hook: CursorHook) {
        self.mouse_cursor.push(hook);
    }

    pub(crate) fn register_mouse_wheel(&mut self, hook: WheelHook) {
        self.mouse_wheel.push(hook);
    }
}

impl<S: LayerCollection<LayerIdentifier = LayerIndex>> InputHookStorage<S> for HotkeyStorage {
    type LayerIdentifier = LayerIndex;

    type RemapHook = OptionalButtonHook;
    type OnPressHook = ButtonHook;
    type OnReleaseHook = OptionalButtonHook;
    type MouseCursorHook = CursorHook;
    type MouseWheelHook = WheelHook;

    fn fetch_remap_hook(&self, event: ButtonEvent, state: &S) -> Option<Self::RemapHook> {
        let storage = match event.action {
            ButtonAction::Press => &self.remap_on_press,
            ButtonAction::Release => &self.remap_on_release,
        };
        storage
            .get(&event.target)?
            .iter()
            .find(|hook| state.is_enabled(hook.layer_id()))
            .cloned()
    }

    fn fetch_on_press_hook(&self, event: ButtonEvent, state: &S) -> Vec<Self::OnPressHook> {
        if let Some(hooks) = self.on_press.get(&event.target) {
            hooks
                .iter()
                .filter(|hook| state.is_enabled(hook.layer_id()))
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
                .filter(|hook| state.is_enabled(hook.layer_id()))
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }

    fn fetch_mouse_cursor_hook(&self, _: CursorEvent, state: &S) -> Vec<Self::MouseCursorHook> {
        self.mouse_cursor
            .iter()
            .filter(|hook| state.is_enabled(hook.layer_id()))
            .cloned()
            .collect()
    }

    fn fetch_mouse_wheel_hook(&self, _: WheelEvent, state: &S) -> Vec<Self::MouseWheelHook> {
        self.mouse_wheel
            .iter()
            .filter(|hook| state.is_enabled(hook.layer_id()))
            .cloned()
            .collect()
    }
}
