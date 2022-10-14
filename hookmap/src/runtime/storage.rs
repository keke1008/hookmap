use std::sync::Arc;

use hookmap_core::event::{ButtonEvent, CursorEvent, NativeEventOperation, WheelEvent};

use crate::layer::{LayerAction, LayerFacade, LayerIndex, LayerState};

use super::hook::HookAction;

pub(crate) trait InputHookFetcher: Send {
    fn fetch_exclusive_button_hook(
        &self,
        event: ButtonEvent,
        state: &LayerState,
        facade: &LayerFacade,
    ) -> Option<Arc<HookAction<ButtonEvent>>>;

    fn fetch_button_hook(
        &self,
        event: ButtonEvent,
        state: &LayerState,
        facade: &LayerFacade,
    ) -> Vec<Arc<HookAction<ButtonEvent>>>;

    fn fetch_mouse_cursor_hook(
        &self,
        state: &LayerState,
        facade: &LayerFacade,
    ) -> Vec<Arc<HookAction<CursorEvent>>>;

    fn fetch_mouse_wheel_hook(
        &self,
        state: &LayerState,
        facade: &LayerFacade,
    ) -> Vec<Arc<HookAction<WheelEvent>>>;
}

pub(crate) trait InterruptionFetcher: Send {
    fn fetch_raw_hook(&mut self, event: ButtonEvent) -> (bool, NativeEventOperation);

    fn fetch_hook(&mut self, event: ButtonEvent) -> NativeEventOperation;
}

pub(crate) trait LayerHookFetcher: Send {
    fn fetch(
        &self,
        layer: LayerIndex,
        update: LayerAction,
        state: LayerState,
        facade: &LayerFacade,
    ) -> Vec<Arc<HookAction<ButtonEvent>>>;
}
