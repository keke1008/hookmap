use std::sync::Arc;

use hookmap_core::button::{Button, ButtonAction};
use hookmap_core::event::{ButtonEvent, NativeEventOperation};

use super::layer::LayerIndex;
use crate::runtime::hook::{self, Hook, LayerQuerySender, LayerStateUpdate};

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
    Procedure(Procedure<E>),
    EnableLayer {
        tx: LayerQuerySender<LayerIndex>,
        id: LayerIndex,
    },
    DisableLayer {
        tx: LayerQuerySender<LayerIndex>,
        id: LayerIndex,
    },
    Noop,
}

impl<E> HookAction<E> {
    fn run(&self, event: E) {
        match self {
            Self::Procedure(procedure) => procedure.0(event),
            Self::EnableLayer { tx, id } => tx.send(LayerStateUpdate::Enabled, *id),
            Self::DisableLayer { tx, id } => tx.send(LayerStateUpdate::Disabled, *id),
            Self::Noop => {}
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct LayerHook {
    layer_id: LayerIndex,
    action: HookAction<Option<ButtonEvent>>,
}

impl LayerHook {
    pub(crate) fn new(layer_id: LayerIndex, action: HookAction<Option<ButtonEvent>>) -> Self {
        Self { layer_id, action }
    }

    pub(crate) fn id(&self) -> LayerIndex {
        self.layer_id
    }
}

impl Hook<Option<ButtonEvent>> for LayerHook {
    fn run(&self, event: Option<ButtonEvent>) {
        self.action.run(event);
    }
}

#[derive(Debug, Clone)]
pub(crate) struct InputHook<E> {
    layer_id: LayerIndex,
    action: HookAction<E>,
    native_event_operation: NativeEventOperation,
}

impl<E> InputHook<E> {
    pub(crate) fn new(
        layer_id: LayerIndex,
        action: HookAction<E>,
        native_event_operation: NativeEventOperation,
    ) -> Self {
        Self {
            layer_id,
            action,
            native_event_operation,
        }
    }

    pub(crate) fn id(&self) -> LayerIndex {
        self.layer_id
    }
}

impl<E> Hook<E> for InputHook<E> {
    fn run(&self, event: E) {
        self.action.run(event);
    }
}

impl<E> hook::InputHook<E> for InputHook<E> {
    fn native_event_operation(&self) -> NativeEventOperation {
        self.native_event_operation
    }
}

#[derive(Debug, Clone)]
pub(crate) struct RemapHook {
    layer_id: LayerIndex,
    button: Button,
}

impl RemapHook {
    pub(crate) fn new(layer_id: LayerIndex, button: Button) -> Self {
        Self { layer_id, button }
    }
}

impl Hook<Option<ButtonEvent>> for RemapHook {
    fn run(&self, event: Option<ButtonEvent>) {
        match event {
            Some(e) => match e.action {
                ButtonAction::Press => self.button.press(),
                ButtonAction::Release => self.button.release(),
            },
            None => self.button.release(),
        }
    }
}

impl hook::InputHook<Option<ButtonEvent>> for RemapHook {
    fn native_event_operation(&self) -> NativeEventOperation {
        NativeEventOperation::Block
    }
}

impl RemapHook {
    pub(crate) fn id(&self) -> LayerIndex {
        self.layer_id
    }
}
