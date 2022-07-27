use hookmap_core::event::{ButtonEvent, CursorEvent, NativeEventOperation, WheelEvent};

use std::sync::{
    mpsc::{self, Receiver, SyncSender},
    Arc,
};

pub(crate) trait LayerIdentifier: Send + Copy {}

pub(crate) trait LayerCollection: Send + Sync {
    type LayerIdentifier: LayerIdentifier;

    fn is_enabled(&self, id: Self::LayerIdentifier) -> bool;

    fn update_enable(&self, id: Self::LayerIdentifier);

    fn update_disable(&self, id: Self::LayerIdentifier);
}

pub(crate) trait Hook<E>: Send {
    fn run(&self, event: E);
}

impl<E, H: Hook<E> + Sync> Hook<E> for Arc<H> {
    fn run(&self, event: E) {
        (**self).run(event)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayerState {
    Enabled,
    Disabled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LayerQuery<ID: LayerIdentifier> {
    pub(crate) id: ID,
    pub(crate) update: LayerState,
}

#[derive(Debug, Clone)]
pub(crate) struct LayerQuerySender<ID: LayerIdentifier> {
    tx: SyncSender<LayerQuery<ID>>,
}

pub(crate) fn layer_query_channel<ID>() -> (LayerQuerySender<ID>, Receiver<LayerQuery<ID>>)
where
    ID: LayerIdentifier,
{
    const BOUND: usize = 8;
    let (tx, rx) = mpsc::sync_channel(BOUND);
    (LayerQuerySender { tx }, rx)
}

impl<ID: LayerIdentifier> LayerQuerySender<ID> {
    pub(crate) fn send(&self, update: LayerState, id: ID) {
        self.tx.send(LayerQuery { id, update }).unwrap();
    }
}

pub(crate) trait LayerHookStrage<S>: Send
where
    S: LayerCollection<LayerIdentifier = Self::LayerIdentifier>,
{
    type LayerIdentifier: LayerIdentifier;
    type Hook: Hook<Option<ButtonEvent>>;

    fn fetch(&self, query: &LayerQuery<Self::LayerIdentifier>, state: &S) -> Vec<Self::Hook>;
}

pub(crate) trait InputHook<E>: Hook<E> {
    fn native_event_operation(&self) -> NativeEventOperation;
}

impl<E, H: InputHook<E> + Sync> InputHook<E> for Arc<H> {
    fn native_event_operation(&self) -> NativeEventOperation {
        (**self).native_event_operation()
    }
}

pub(crate) trait InputHookStorage<S>: Send
where
    S: LayerCollection<LayerIdentifier = Self::LayerIdentifier>,
{
    type LayerIdentifier: LayerIdentifier;

    type RemapHook: InputHook<Option<ButtonEvent>>;
    type OnPressHook: InputHook<ButtonEvent>;
    type OnReleaseHook: InputHook<Option<ButtonEvent>>;
    type MouseCursorHook: InputHook<CursorEvent>;
    type MouseWheelHook: InputHook<WheelEvent>;

    fn fetch_remap_hook(&self, event: ButtonEvent, state: &S) -> Option<Self::RemapHook>;
    fn fetch_on_press_hook(&self, event: ButtonEvent, state: &S) -> Vec<Self::OnPressHook>;
    fn fetch_on_release_hook(&self, event: ButtonEvent, state: &S) -> Vec<Self::OnReleaseHook>;
    fn fetch_mouse_cursor_hook(&self, event: CursorEvent, state: &S) -> Vec<Self::MouseCursorHook>;
    fn fetch_mouse_wheel_hook(&self, event: WheelEvent, state: &S) -> Vec<Self::MouseWheelHook>;
}
