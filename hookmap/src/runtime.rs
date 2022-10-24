mod worker;

pub(crate) mod hook;
pub(crate) mod storage;

use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

use hookmap_core::event::{Event, NativeEventHandler, NativeEventOperation};

use crate::condition::flag::FlagState;

use hook::{FlagEvent, HookAction};
use storage::{FlagHookFetcher, InputHookFetcher};
use worker::{Action, Message, Worker};

pub(crate) struct Runtime<Input, Flag> {
    input_fetcher: Input,
    flag_fetcher: Flag,
    flag_state: Arc<Mutex<FlagState>>,
}

impl<Input, Flag> Runtime<Input, Flag> {
    pub(crate) fn new(
        input_fetcher: Input,
        flag_fetcher: Flag,
        flag_state: Arc<Mutex<FlagState>>,
    ) -> Self {
        Self {
            input_fetcher,
            flag_fetcher,
            flag_state,
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

impl<Input, Flag> Runtime<Input, Flag>
where
    Input: InputHookFetcher,
    Flag: FlagHookFetcher,
{
    pub(crate) fn start(
        self,
        input_rx: Receiver<(Event, NativeEventHandler)>,
        flag_tx: Sender<FlagEvent>,
        flag_rx: Receiver<FlagEvent>,
    ) {
        let Runtime {
            input_fetcher,
            flag_fetcher,
            flag_state,
        } = self;

        let (worker_tx, worker) = Worker::new(Arc::clone(&flag_state), flag_tx);

        thread::scope(|scope| {
            let worker_tx_ = worker_tx.clone();
            scope.spawn(|| {
                let (input_rx, input_fetcher, worker_tx) = (input_rx, input_fetcher, worker_tx_);

                for (event, native_handler) in input_rx.iter() {
                    let state = flag_state.lock().unwrap();

                    match event {
                        Event::Button(button_event) => {
                            let action =
                                input_fetcher.fetch_exclusive_button_hook(button_event, &*state);
                            let mut native = action
                                .as_ref()
                                .map_or(NativeEventOperation::Dispatch, |action| {
                                    action.native_event_operation()
                                });

                            if let Some(action) = action {
                                native_handler.handle(native);

                                let msg = Message::Button(Action::new(button_event, vec![action]));
                                worker_tx.send(msg).unwrap();
                                continue;
                            }

                            let actions = input_fetcher.fetch_button_hook(button_event, &state);
                            native = native.or(calculate_native(&actions));
                            native_handler.handle(native);

                            let msg = Message::Button(Action::new(button_event, actions));
                            worker_tx.send(msg).unwrap();
                        }

                        Event::Cursor(cursor_event) => {
                            let actions = input_fetcher.fetch_mouse_cursor_hook(&state);
                            native_handler.handle(calculate_native(&actions));

                            let msg = Message::Cursor(Action::new(cursor_event, actions));
                            worker_tx.send(msg).unwrap();
                        }

                        Event::Wheel(wheel_event) => {
                            let actions = input_fetcher.fetch_mouse_wheel_hook(&state);
                            native_handler.handle(calculate_native(&actions));

                            let msg = Message::Wheel(Action::new(wheel_event, actions));
                            worker_tx.send(msg).unwrap();
                        }
                    }
                }
            });

            scope.spawn(|| {
                let (flag_rx, flag_fetcher, worker_tx) = (flag_rx, flag_fetcher, worker_tx);

                for event in flag_rx.iter() {
                    let inherited_event = event.inherited_event;
                    let actions = flag_fetcher.fetch(event);
                    let msg = Message::Optional(Action::new(inherited_event, actions));
                    worker_tx.send(msg).unwrap();
                }
            });
        });

        worker.join();
    }
}
