mod hook;
mod runner;

use crossbeam_utils::thread;

use hookmap_core::button::ButtonAction;
use hookmap_core::event::{Event, NativeEventHandler, NativeEventOperation};

use std::sync::mpsc::Receiver;

use runner::{HookRunner, Message, Task};

pub use hook::LayerState;
pub(crate) use hook::{
    layer_query_channel, Hook, InputHook, InputHookStorage, LayerHookStrage, LayerIdentifier,
    LayerQuery, LayerQuerySender, LayerStateCollection,
};

fn handle_native_event<'env, E, H>(hooks: &[&'env H], native_handler: NativeEventHandler)
where
    E: Send + Copy + 'static,
    H: InputHook<E>,
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
}

#[derive(Debug)]
pub(crate) struct Runtime<ID, L, I, S>
where
    ID: LayerIdentifier,
    L: LayerHookStrage<S, LayerIdentifier = ID>,
    I: InputHookStorage<S, LayerIdentifier = ID>,
    S: LayerStateCollection<LayerIdentifier = ID>,
{
    layer_storage: L,
    input_storage: I,
    state: S,
}

impl<ID, L, I, S> Runtime<ID, L, I, S>
where
    ID: LayerIdentifier,
    L: LayerHookStrage<S, LayerIdentifier = ID>,
    I: InputHookStorage<S, LayerIdentifier = ID>,
    S: LayerStateCollection<LayerIdentifier = ID>,
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

        thread::scope(|scope| {
            let runner = HookRunner::new(scope);

            let queue = runner.queue();
            let input_thread = scope.spawn(|_| {
                let (input_rx, queue) = (input_rx, queue);

                while let Ok((event, native_handler)) = input_rx.recv() {
                    match event {
                        Event::Button(event) => {
                            if let Some(hook) = input_storage.fetch_remap_hook(event, &state) {
                                let hooks = vec![hook];
                                handle_native_event(&hooks, native_handler);
                                queue.enqueue(Message::Remap(Task::new(Some(event), hooks)));
                                continue;
                            }
                            match event.action {
                                ButtonAction::Press => {
                                    let hooks = input_storage.fetch_on_press_hook(event, &state);
                                    handle_native_event(&hooks, native_handler);
                                    queue.enqueue(Message::OnPress(Task::new(event, hooks)));
                                }
                                ButtonAction::Release => {
                                    let hooks = input_storage.fetch_on_release_hook(event, &state);
                                    handle_native_event(&hooks, native_handler);
                                    queue
                                        .enqueue(Message::OnRelease(Task::new(Some(event), hooks)));
                                }
                            }
                        }
                        Event::Wheel(event) => {
                            let hooks = input_storage.fetch_mouse_wheel_hook(event, &state);
                            handle_native_event(&hooks, native_handler);
                            queue.enqueue(Message::Wheel(Task::new(event, hooks)));
                        }
                        Event::Cursor(event) => {
                            let hooks = input_storage.fetch_mouse_cursor_hook(event, &state);
                            handle_native_event(&hooks, native_handler);
                            queue.enqueue(Message::Cursor(Task::new(event, hooks)));
                        }
                    }
                }
            });

            let queue = runner.queue();
            let layer_thread = scope.spawn(|_| {
                let (layer_rx, queue) = (layer_rx, queue);

                while let Ok(query) = layer_rx.recv() {
                    let hooks = match query.update {
                        LayerState::Enabled => {
                            state.update_enable(query.id);
                            layer_storage.fetch(&query, &state)
                        }
                        LayerState::Disabled => {
                            let hooks = layer_storage.fetch(&query, &state);
                            state.update_disable(query.id);
                            hooks
                        }
                    };
                    queue.enqueue(Message::Layer(Task::new(None, hooks)));
                }
            });

            input_thread.join().unwrap();
            layer_thread.join().unwrap();
            runner.terminate();
        })
        .unwrap();
    }
}
