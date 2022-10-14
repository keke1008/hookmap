mod worker;

pub(crate) mod hook;
pub(crate) mod interruption;
pub(crate) mod storage;

use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

use hookmap_core::event::{Event, NativeEventHandler, NativeEventOperation};

use crate::layer::{LayerFacade, LayerState};

use hook::LayerEvent;
use storage::{InputHookFetcher, InterruptionFetcher, LayerHookFetcher};
use worker::{Action, Message, Worker};

use self::hook::HookAction;

pub(crate) struct Runtime<Input, Interruption, Layer> {
    input_fetcher: Input,
    interruption_storage: Interruption,
    layer_fetcher: Layer,
    layer_state: Arc<Mutex<LayerState>>,
    layer_facade: LayerFacade,
}

impl<Input, Interruption, Layer> Runtime<Input, Interruption, Layer> {
    pub(crate) fn new(
        input_fetcher: Input,
        interruption_storage: Interruption,
        layer_fetcher: Layer,
        layer_state: Arc<Mutex<LayerState>>,
        layer_facade: LayerFacade,
    ) -> Self {
        Self {
            input_fetcher,
            interruption_storage,
            layer_fetcher,
            layer_state,
            layer_facade,
        }
    }
}

fn calculate_native<E>(actions: &[Arc<HookAction<E>>]) -> NativeEventOperation {
    actions
        .iter()
        .map(|action| action.native_event_operation())
        .find(|native| *native == NativeEventOperation::Block)
        .unwrap_or(NativeEventOperation::Dispatch)
}

impl<Input, Interruption, Layer> Runtime<Input, Interruption, Layer>
where
    Input: InputHookFetcher,
    Interruption: InterruptionFetcher,
    Layer: LayerHookFetcher,
{
    pub(crate) fn start(
        self,
        input_rx: Receiver<(Event, NativeEventHandler)>,
        layer_tx: Sender<LayerEvent>,
        layer_rx: Receiver<LayerEvent>,
    ) {
        let Runtime {
            input_fetcher,
            interruption_storage,
            layer_fetcher,
            layer_state,
            layer_facade,
        } = self;

        let (worker_tx, worker) = Worker::new(Arc::clone(&layer_state), layer_tx);

        thread::scope(|scope| {
            let worker_tx_ = worker_tx.clone();
            scope.spawn(|| {
                let (input_rx, input_fetcher, worker_tx, mut interruption_storage) =
                    (input_rx, input_fetcher, worker_tx_, interruption_storage);

                for (event, native_handler) in input_rx.iter() {
                    let state = layer_state.lock().unwrap();

                    match event {
                        Event::Button(button_event) => {
                            let (found, mut native) =
                                interruption_storage.fetch_raw_hook(button_event);
                            if found {
                                native_handler.handle(native);
                                continue;
                            }

                            let action = input_fetcher.fetch_exclusive_button_hook(
                                button_event,
                                &state,
                                &layer_facade,
                            );
                            native = action
                                .as_ref()
                                .map_or(NativeEventOperation::Dispatch, |action| {
                                    action.native_event_operation()
                                })
                                .or(native);

                            if let Some(action) = action {
                                native_handler.handle(native);

                                let msg = Message::Button(Action::new(button_event, vec![action]));
                                worker_tx.send(msg).unwrap();
                                continue;
                            }

                            let native_ = interruption_storage.fetch_hook(button_event);
                            native = native.or(native_);

                            let actions = input_fetcher.fetch_button_hook(
                                button_event,
                                &state,
                                &layer_facade,
                            );
                            native = native.or(calculate_native(&actions));
                            native_handler.handle(native);

                            let msg = Message::Button(Action::new(button_event, actions));
                            worker_tx.send(msg).unwrap();
                        }

                        Event::Cursor(cursor_event) => {
                            let actions =
                                input_fetcher.fetch_mouse_cursor_hook(&state, &layer_facade);
                            native_handler.handle(calculate_native(&actions));

                            let msg = Message::Cursor(Action::new(cursor_event, actions));
                            worker_tx.send(msg).unwrap();
                        }

                        Event::Wheel(wheel_event) => {
                            let actions =
                                input_fetcher.fetch_mouse_wheel_hook(&state, &layer_facade);
                            native_handler.handle(calculate_native(&actions));

                            let msg = Message::Wheel(Action::new(wheel_event, actions));
                            worker_tx.send(msg).unwrap();
                        }
                    }
                }
            });

            scope.spawn(|| {
                let (layer_rx, layer_fetcher, worker_tx) = (layer_rx, layer_fetcher, worker_tx);

                for event in layer_rx.iter() {
                    let actions = layer_fetcher.fetch(
                        event.layer,
                        event.action,
                        event.snapshot,
                        &layer_facade,
                    );
                    let msg = Message::Optional(Action::new(event.inherited_event, actions));
                    worker_tx.send(msg).unwrap();
                }
            });
        });

        worker.join();
    }
}
