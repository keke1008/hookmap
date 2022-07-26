pub(crate) mod hook;
pub(crate) mod layer;
pub(crate) mod storage;

use hookmap_core::button::Button;
use hookmap_core::event::{ButtonEvent, CursorEvent, NativeEventOperation, WheelEvent};

use std::sync::Arc;

pub(crate) trait Hook<E> {
    fn native_event_operation(&self) -> NativeEventOperation;
    fn run(&self, event: E);
}

impl<E, T: Hook<E>> Hook<E> for Arc<T> {
    fn native_event_operation(&self) -> NativeEventOperation {
        (**self).native_event_operation()
    }

    fn run(&self, event: E) {
        (**self).run(event);
    }
}

pub(crate) trait ButtonState {
    fn is_pressed(&self, button: Button) -> bool;
    fn is_released(&self, button: Button) -> bool;
}

pub(crate) trait HookStorage {
    type ButtonHook: Hook<ButtonEvent>;
    type MouseCursorHook: Hook<CursorEvent>;
    type MouseWheelHook: Hook<WheelEvent>;

    fn fetch_button_hook<S: ButtonState>(
        &self,
        event: ButtonEvent,
        state: &S,
    ) -> Vec<Self::ButtonHook>;

    fn fetch_mouse_cursor_hook<S: ButtonState>(
        &self,
        event: CursorEvent,
        state: &S,
    ) -> Vec<Self::MouseCursorHook>;

    fn fetch_mouse_wheel_hook<S: ButtonState>(
        &self,
        event: WheelEvent,
        state: &S,
    ) -> Vec<Self::MouseWheelHook>;
}

pub(crate) trait LayerState {
    type Layer;

    fn is_enabled(&self, layer: &Self::Layer) -> bool;

    /// Returns true if there are hooks to execute.
    fn update_enable(&mut self, layer: &Self::Layer) -> bool;

    /// Returns true if there are hooks to execute.
    fn update_disable(&mut self, layer: &Self::Layer) -> bool;
}

pub(crate) trait LayerHook<E> {
    fn run(&self, event: Option<E>);
}

impl<E, H: LayerHook<E>> LayerHook<E> for Arc<H> {
    fn run(&self, event: Option<E>) {
        (**self).run(event)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) struct FetchedLayerHooks<B, C, W>
where
    B: LayerHook<ButtonEvent>,
    C: LayerHook<CursorEvent>,
    W: LayerHook<WheelEvent>,
{
    button: Vec<B>,
    cursor: Vec<C>,
    wheel: Vec<W>,
}

pub(crate) trait LayerHookStorage {
    type Layer;

    type ButtonHook: LayerHook<ButtonEvent>;
    type MouseCursorHook: LayerHook<CursorEvent>;
    type MouseWheelHook: LayerHook<WheelEvent>;

    fn fetch_button_hook<S>(
        &self,
        layer: &Self::Layer,
        state: S,
    ) -> FetchedLayerHooks<Self::ButtonHook, Self::MouseCursorHook, Self::MouseWheelHook>
    where
        S: LayerState<Layer = Self::Layer>;
}
