mod hook;
mod layer;

use std::hash::Hash;
use std::{collections::HashMap, sync::Arc};

use hookmap_core::button::{Button, ButtonAction};
use hookmap_core::event::{ButtonEvent, CursorEvent, WheelEvent};

pub(crate) use hook::{Hook, HookAction, Procedure};
pub(crate) use layer::{
    layer_query_channel, LayerIndex, LayerQuery, LayerQuerySender, LayerState, LayerTree,
};

#[inline]
fn push_to_hashmap_vec<K: Eq + Hash, V>(map: &mut HashMap<K, Vec<V>>, key: K, value: V) {
    map.entry(key).or_default().push(value);
}

#[inline]
fn filter_by_state_and_clone<E>(hooks: &[Arc<Hook<E>>], state: &LayerTree) -> Vec<Arc<Hook<E>>> {
    hooks
        .iter()
        .filter(|hook| state.is_enabled(hook.layer_id()))
        .cloned()
        .collect()
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

impl LayerHookStorage {
    pub(crate) fn fetch(&self, query: &LayerQuery, state: &LayerTree) -> Vec<OptionalButtonHook> {
        let hooks = match query.update {
            LayerState::Enabled => &self.on_enabled,
            LayerState::Disabled => &self.on_disabled,
        };
        hooks
            .get(&query.id)
            .map(|hooks| filter_by_state_and_clone(hooks, state))
            .unwrap_or_default()
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

impl HotkeyStorage {
    pub(crate) fn fetch_remap_hook(
        &self,
        event: ButtonEvent,
        state: &LayerTree,
    ) -> Option<OptionalButtonHook> {
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

    pub(crate) fn fetch_on_press_hook(
        &self,
        event: ButtonEvent,
        state: &LayerTree,
    ) -> Vec<ButtonHook> {
        self.on_press
            .get(&event.target)
            .map(|hooks| filter_by_state_and_clone(hooks, state))
            .unwrap_or_default()
    }

    pub(crate) fn fetch_on_release_hook(
        &self,
        event: ButtonEvent,
        state: &LayerTree,
    ) -> Vec<OptionalButtonHook> {
        self.on_release
            .get(&event.target)
            .map(|hooks| filter_by_state_and_clone(hooks, state))
            .unwrap_or_default()
    }

    pub(crate) fn fetch_mouse_cursor_hook(
        &self,
        _: CursorEvent,
        state: &LayerTree,
    ) -> Vec<CursorHook> {
        filter_by_state_and_clone(&self.mouse_cursor, state)
    }

    pub(crate) fn fetch_mouse_wheel_hook(
        &self,
        _: WheelEvent,
        state: &LayerTree,
    ) -> Vec<WheelHook> {
        filter_by_state_and_clone(&self.mouse_wheel, state)
    }
}
