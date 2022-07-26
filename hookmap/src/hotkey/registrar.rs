use std::sync::Arc;

use hookmap_core::button::Button;
use hookmap_core::event::{ButtonEvent, CursorEvent, NativeEventOperation, WheelEvent};

use super::layer::Layer;

use crate::hook::hook::{HookAction, InputHook, LayerHook, Procedure, RemapHook};
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
        self.hotkey_storage
            .remap_on_press
            .entry(target)
            .or_default()
            .push(Arc::new(RemapHook::new(context.layer_id, behavior)));

        self.hotkey_storage
            .remap_on_release
            .entry(target)
            .or_default()
            .push(Arc::new(RemapHook::new(
                self.state.create_inheritance_layer(context.layer_id, false),
                behavior,
            )));
    }

    pub(super) fn on_press(
        &mut self,
        context: &Context,
        target: Button,
        procedure: Procedure<ButtonEvent>,
    ) {
        self.hotkey_storage
            .on_press
            .entry(target)
            .or_default()
            .push(Arc::new(InputHook::new(
                context.layer_id,
                HookAction::Procedure(procedure),
                context.native_event_operation,
            )));
    }

    pub(super) fn on_release(
        &mut self,
        context: &Context,
        target: Button,
        procedure: Procedure<Option<ButtonEvent>>,
    ) {
        let enabled_layer = self.state.create_inheritance_layer(context.layer_id, false);

        self.hotkey_storage
            .on_press
            .entry(target)
            .or_default()
            .push(Arc::new(InputHook::new(
                context.layer_id,
                HookAction::EnableLayer {
                    tx: self.tx.clone(),
                    id: enabled_layer,
                },
                NativeEventOperation::Dispatch,
            )));

        let action = HookAction::Procedure(procedure);

        self.hotkey_storage
            .on_release
            .entry(target)
            .or_default()
            .push(Arc::new(InputHook::new(
                enabled_layer,
                action.clone(),
                context.native_event_operation,
            )));

        let layer_hook = Arc::new(LayerHook::new(context.layer_id, action));

        for ancestor in self.state.iter_ancestors(context.layer_id) {
            self.layer_storage
                .on_disabled
                .entry(ancestor)
                .or_default()
                .push(Arc::clone(&layer_hook));
        }
    }

    pub(super) fn mouse_cursor(&mut self, context: &Context, procedure: Procedure<CursorEvent>) {
        self.hotkey_storage
            .mouse_cursor
            .push(Arc::new(InputHook::new(
                context.layer_id,
                HookAction::Procedure(procedure),
                context.native_event_operation,
            )));
    }

    pub(super) fn mouse_wheel(&mut self, context: &Context, procedure: Procedure<WheelEvent>) {
        self.hotkey_storage
            .mouse_wheel
            .push(Arc::new(InputHook::new(
                context.layer_id,
                HookAction::Procedure(procedure),
                context.native_event_operation,
            )));
    }

    pub(super) fn disable(&mut self, context: &Context, target: Button) {
        self.hotkey_storage
            .on_press
            .entry(target)
            .or_default()
            .push(Arc::new(InputHook::new(
                context.layer_id,
                HookAction::Noop,
                NativeEventOperation::Block,
            )));

        self.hotkey_storage
            .on_release
            .entry(target)
            .or_default()
            .push(Arc::new(InputHook::new(
                context.layer_id,
                HookAction::Noop,
                NativeEventOperation::Block,
            )));
    }

    pub(super) fn layer(&mut self, context: &Context, init_state: bool) -> Layer {
        let layer_id = self.state.create_layer(context.layer_id, init_state);
        Layer::new(self.tx.clone(), layer_id)
    }
}
