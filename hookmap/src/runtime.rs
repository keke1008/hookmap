pub(crate) mod hook;

use hookmap_core::button::ButtonAction;
use hookmap_core::event::{Event, NativeEventHandler, NativeEventOperation};

use hook::{
    Hook, InputHook, InputHookStorage, LayerHookStrage, LayerIdentifier, LayerQuery, LayerState,
    LayerStateUpdate,
};

use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::thread;

fn handle_input_event<E, H>(hooks: Vec<H>, event: E, native_handler: NativeEventHandler)
where
    E: Send + Copy + 'static,
    H: InputHook<E> + 'static,
{
    let has_block_operation = hooks
        .iter()
        .map(|hook| hook.native_event_operation())
        .any(|operation| operation == NativeEventOperation::Block);
    let operation = if has_block_operation {
        NativeEventOperation::Block
    } else {
        NativeEventOperation::Dispatch
    };
    native_handler.handle(operation);
    std::thread::spawn(move || hooks.iter().for_each(move |hook| hook.run(event)));
}

#[derive(Debug)]
pub(crate) struct Runtime<ID, L, I, S>
where
    ID: LayerIdentifier + 'static,
    L: LayerHookStrage<S, LayerIdentifier = ID> + 'static,
    I: InputHookStorage<S, LayerIdentifier = ID> + 'static,
    S: LayerState<LayerIdentifier = ID> + 'static,
{
    layer_storage: L,
    input_storage: I,
    state: S,
}

impl<ID, L, I, S> Runtime<ID, L, I, S>
where
    ID: LayerIdentifier + 'static,
    L: LayerHookStrage<S, LayerIdentifier = ID> + 'static,
    I: InputHookStorage<S, LayerIdentifier = ID> + 'static,
    S: LayerState<LayerIdentifier = ID> + 'static,
{
    pub(crate) fn new(layer_storage: L, input_storage: I, state: S) -> Self {
        Self {
            layer_storage,
            input_storage,
            state,
        }
    }

    pub(crate) fn start(
        self,
        layer_rx: Receiver<LayerQuery<ID>>,
        input_rx: Receiver<(Event, NativeEventHandler)>,
    ) {
        let Runtime {
            layer_storage,
            input_storage,
            state,
        } = self;
        let state = Arc::new(state);
        let state_ = Arc::clone(&state);

        thread::spawn(move || {
            while let Ok((event, native_handler)) = input_rx.recv() {
                match event {
                    Event::Button(event) => {
                        if let Some(hook) = input_storage.fetch_remap_hook(event, &*state) {
                            handle_input_event(vec![hook], Some(event), native_handler);
                            continue;
                        }
                        match event.action {
                            ButtonAction::Press => {
                                let hooks = input_storage.fetch_on_press_hook(event, &*state);
                                handle_input_event(hooks, event, native_handler);
                            }
                            ButtonAction::Release => {
                                let hooks = input_storage.fetch_on_release_hook(event, &*state);
                                handle_input_event(hooks, Some(event), native_handler)
                            }
                        }
                    }
                    Event::Wheel(event) => {
                        let hooks = input_storage.fetch_mouse_wheel_hook(event, &*state);
                        handle_input_event(hooks, event, native_handler);
                    }
                    Event::Cursor(event) => {
                        let hooks = input_storage.fetch_mouse_cursor_hook(event, &*state);
                        handle_input_event(hooks, event, native_handler);
                    }
                }
            }
        });

        while let Ok(query) = layer_rx.recv() {
            let hooks = match query.update {
                LayerStateUpdate::Enabled => {
                    state_.update_enable(query.id);
                    layer_storage.fetch(&query, &*state_)
                }
                LayerStateUpdate::Disabled => {
                    let hooks = layer_storage.fetch(&query, &*state_);
                    state_.update_disable(query.id);
                    hooks
                }
            };
            thread::spawn(move || hooks.iter().for_each(|hook| hook.run(None)));
        }
    }
}
