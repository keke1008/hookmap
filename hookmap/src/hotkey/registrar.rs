use std::sync::{Arc, Mutex};

use hookmap_core::button::Button;
use hookmap_core::event::{ButtonEvent, CursorEvent, WheelEvent};

use crate::layer::LayerIndex;
use crate::runtime::{hook::HookAction, interruption::InterruptionStorage};

use super::interruption::Interruption;
use super::layer::LayerCreator;
use super::storage::{InputHookStorage, LayerHookStorage};

#[derive(Debug, Default)]
pub(super) struct HotkeyStorage {
    input_storage: InputHookStorage,
    interruption_storage: Arc<Mutex<InterruptionStorage>>,
    layer_storage: LayerHookStorage,
}

impl HotkeyStorage {
    pub(super) fn destruct(
        self,
    ) -> (
        InputHookStorage,
        Arc<Mutex<InterruptionStorage>>,
        LayerHookStorage,
    ) {
        (
            self.input_storage,
            self.interruption_storage,
            self.layer_storage,
        )
    }

    pub(super) fn remap(
        &mut self,
        layer: LayerIndex,
        target: Button,
        behavior: Button,
        layer_creator: &mut LayerCreator,
    ) {
        let enabled_layer = layer_creator.create_sync_layer(layer, false);

        self.input_storage.on_press_exclusive.register_specific(
            layer,
            target,
            Arc::new(HookAction::RemapPress {
                button: behavior,
                layer: enabled_layer,
            }),
        );

        self.input_storage.on_release.register_specific(
            enabled_layer,
            target,
            Arc::new(HookAction::RemapRelease {
                button: behavior,
                layer: enabled_layer,
            }),
        );
    }

    pub(super) fn on_press(
        &mut self,
        layer: LayerIndex,
        target: Button,
        action: HookAction<ButtonEvent>,
    ) {
        self.input_storage
            .on_press
            .register_specific(layer, target, Arc::new(action));
    }

    pub(super) fn on_release(
        &mut self,
        layer: LayerIndex,
        target: Button,
        action: HookAction<ButtonEvent>,
        layer_creator: &mut LayerCreator,
    ) {
        let enabled_layer = layer_creator.create_sync_layer(layer, false);

        self.input_storage.on_press.register_specific(
            layer,
            target,
            Arc::new(HookAction::EnableLayer(enabled_layer)),
        );

        self.input_storage.on_release.register_specific(
            enabled_layer,
            target,
            Arc::new(HookAction::DisableLayer(enabled_layer)),
        );

        self.layer_storage
            .register_on_inactivated(enabled_layer, Arc::new(action));
    }

    pub(super) fn disable(&mut self, layer: LayerIndex, target: Button) {
        self.input_storage
            .on_press
            .register_specific(layer, target, Arc::new(HookAction::Block));

        self.input_storage
            .on_release
            .register_specific(layer, target, Arc::new(HookAction::Block));
    }

    pub(super) fn mouse_cursor(&mut self, layer: LayerIndex, action: HookAction<CursorEvent>) {
        self.input_storage
            .mouse_cursor
            .register(layer, Arc::new(action));
    }

    pub(super) fn mouse_wheel(&mut self, layer: LayerIndex, action: HookAction<WheelEvent>) {
        self.input_storage
            .mouse_wheel
            .register(layer, Arc::new(action));
    }

    pub(super) fn on_layer_activated(
        &mut self,
        layer: LayerIndex,
        action: HookAction<ButtonEvent>,
    ) {
        self.layer_storage
            .register_on_activated(layer, Arc::new(action));
    }

    pub(super) fn on_layer_inactivated(
        &mut self,
        layer: LayerIndex,
        action: HookAction<ButtonEvent>,
    ) {
        self.layer_storage
            .register_on_inactivated(layer, Arc::new(action));
    }

    pub(super) fn interruption(&mut self) -> Interruption {
        Interruption::new(Arc::clone(&self.interruption_storage))
    }
}
