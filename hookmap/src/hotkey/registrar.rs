use std::sync::Arc;

use hookmap_core::button::Button;
use hookmap_core::event::{ButtonEvent, CursorEvent, NativeEventOperation, WheelEvent};

use super::layer::Layer;

use crate::hook::hook::{Hook, HookAction, Procedure};
use crate::hook::layer::{LayerIndex, LayerQuerySender, LayerState};
use crate::hook::storage::{HotkeyStorage, LayerHookStorage};

#[derive(Debug)]
pub(super) struct Context {
    layer_id: LayerIndex,
    native_event_operation: NativeEventOperation,
}

impl Context {
    pub(super) fn new(layer_id: LayerIndex, native_event_operation: NativeEventOperation) -> Self {
        Self {
            layer_id,
            native_event_operation,
        }
    }

    pub(super) fn replace_layer_id(&self, layer_id: LayerIndex) -> Self {
        Self {
            layer_id,
            native_event_operation: self.native_event_operation,
        }
    }
}

#[doc(hidden)]
#[derive(Debug)]
pub struct Registrar {
    pub(super) state: LayerState,
    pub(super) layer_storage: LayerHookStorage,
    pub(super) hotkey_storage: HotkeyStorage,
    tx: LayerQuerySender,
}

impl Registrar {
    pub(super) fn new(
        state: LayerState,
        layer_storage: LayerHookStorage,
        hotkey_storage: HotkeyStorage,
        tx: LayerQuerySender,
    ) -> Self {
        Self {
            state,
            layer_storage,
            hotkey_storage,
            tx,
        }
    }

    pub(super) fn remap(&mut self, context: &Context, target: Button, behavior: Button) {
        let on_press_hook = Arc::new(Hook::new(context.layer_id, HookAction::Press(behavior)));
        self.hotkey_storage
            .register_remap_on_press(target, on_press_hook);

        let enabled_layer = self.state.create_inheritance_layer(context.layer_id, false);

        let on_release_hook = Arc::new(Hook::new(enabled_layer, HookAction::Release(behavior)));
        self.hotkey_storage
            .register_remap_on_release(target, on_release_hook);
    }

    pub(super) fn on_press(
        &mut self,
        context: &Context,
        target: Button,
        procedure: Procedure<ButtonEvent>,
    ) {
        let run_action_hook = Arc::new(Hook::new(
            context.layer_id,
            HookAction::Procedure {
                procedure,
                native: context.native_event_operation,
            },
        ));
        self.hotkey_storage
            .register_on_press(target, run_action_hook);
    }

    pub(super) fn on_release(
        &mut self,
        context: &Context,
        target: Button,
        procedure: Procedure<Option<ButtonEvent>>,
    ) {
        let enabled_layer = self.state.create_inheritance_layer(context.layer_id, false);

        let enable_layer_hook = Arc::new(Hook::new(
            context.layer_id,
            HookAction::EnableLayer {
                tx: self.tx.clone(),
                id: enabled_layer,
            },
        ));
        self.hotkey_storage
            .register_on_press(target, enable_layer_hook);

        let disable_layer_hook = Arc::new(Hook::new(
            enabled_layer,
            HookAction::DisableLayer {
                tx: self.tx.clone(),
                id: enabled_layer,
            },
        ));
        self.hotkey_storage
            .register_on_release(target, disable_layer_hook);

        let run_action_hook = Arc::new(Hook::new(
            enabled_layer,
            HookAction::Procedure {
                procedure,
                native: context.native_event_operation,
            },
        ));
        self.hotkey_storage
            .register_on_release(target, Arc::clone(&run_action_hook));
        for ancestor in self.state.iter_ancestors(context.layer_id) {
            self.layer_storage
                .register_on_disabled(ancestor, Arc::clone(&run_action_hook));
        }
    }

    pub(super) fn mouse_cursor(&mut self, context: &Context, procedure: Procedure<CursorEvent>) {
        let run_action_hook = Arc::new(Hook::new(
            context.layer_id,
            HookAction::Procedure {
                procedure,
                native: context.native_event_operation,
            },
        ));
        self.hotkey_storage.register_mouse_cursor(run_action_hook);
    }

    pub(super) fn mouse_wheel(&mut self, context: &Context, procedure: Procedure<WheelEvent>) {
        let run_action_hook = Arc::new(Hook::new(
            context.layer_id,
            HookAction::Procedure {
                procedure,
                native: context.native_event_operation,
            },
        ));
        self.hotkey_storage.register_mouse_wheel(run_action_hook);
    }

    pub(super) fn disable(&mut self, context: &Context, target: Button) {
        let disable_on_press_hook = Arc::new(Hook::new(context.layer_id, HookAction::Block));
        self.hotkey_storage
            .register_on_press(target, disable_on_press_hook);
        let disable_on_release_hook = Arc::new(Hook::new(context.layer_id, HookAction::Block));
        self.hotkey_storage
            .register_on_release(target, disable_on_release_hook);
    }

    pub(super) fn layer(&mut self, context: &Context, init_state: bool) -> Layer {
        let layer_id = self.state.create_layer(context.layer_id, init_state);
        Layer::new(self.tx.clone(), layer_id)
    }

    pub(super) fn inheritance_layer(&mut self, context: &Context, init_state: bool) -> Layer {
        let layer_id = self
            .state
            .create_inheritance_layer(context.layer_id, init_state);
        Layer::new(self.tx.clone(), layer_id)
    }
}
