use std::sync::Arc;

use hookmap_core::button::Button;
use hookmap_core::event::NativeEventOperation;

use super::layer::LayerIndex;
use crate::runtime::hook::{self, LayerQuerySender, LayerState};

#[derive(Clone)]
pub struct Procedure<E>(Arc<dyn Fn(E) + Send + Sync>);

impl<E> Procedure<E> {
    pub fn new(f: Arc<dyn Fn(E) + Send + Sync>) -> Self {
        Procedure(f)
    }
}

impl<E> std::fmt::Debug for Procedure<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Procedure").finish()
    }
}

#[derive(Debug, Clone)]
pub(crate) enum HookAction<E> {
    Procedure {
        procedure: Procedure<E>,
        native: NativeEventOperation,
    },
    Press(Button),
    Release(Button),
    EnableLayer {
        tx: LayerQuerySender<LayerIndex>,
        id: LayerIndex,
    },
    DisableLayer {
        tx: LayerQuerySender<LayerIndex>,
        id: LayerIndex,
    },
    Block,
}

#[derive(Debug, Clone)]
pub(crate) struct Hook<E> {
    layer_id: LayerIndex,
    action: HookAction<E>,
}

impl<E> Hook<E> {
    pub(crate) fn new(layer_id: LayerIndex, action: HookAction<E>) -> Self {
        Self { layer_id, action }
    }
    pub(crate) fn layer_id(&self) -> LayerIndex {
        self.layer_id
    }
}

impl<E> hook::Hook<E> for Hook<E> {
    fn run(&self, event: E) {
        match &self.action {
            HookAction::Procedure { procedure, .. } => procedure.0(event),
            HookAction::Press(button) => button.press(),
            HookAction::Release(button) => button.release(),
            HookAction::EnableLayer { tx, id } => tx.send(LayerState::Enabled, *id),
            HookAction::DisableLayer { tx, id } => tx.send(LayerState::Disabled, *id),
            HookAction::Block => {}
        }
    }
}

impl<E> hook::InputHook<E> for Hook<E> {
    fn native_event_operation(&self) -> NativeEventOperation {
        match &self.action {
            HookAction::Procedure { native, .. } => *native,
            HookAction::Press(_) => NativeEventOperation::Block,
            HookAction::Release(_) => NativeEventOperation::Block,
            HookAction::EnableLayer { .. } => NativeEventOperation::Block,
            HookAction::DisableLayer { .. } => NativeEventOperation::Dispatch,
            HookAction::Block => NativeEventOperation::Block,
        }
    }
}
