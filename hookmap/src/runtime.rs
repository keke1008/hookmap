mod worker;

use std::sync::mpsc::{Receiver, SyncSender};
use std::sync::{Arc, Mutex};
use std::thread;

use hookmap_core::button::ButtonAction;
use hookmap_core::event::{Event, NativeEventHandler};

use crate::condition::flag::FlagState;
use crate::storage::action::FlagEvent;
use crate::storage::{InputHookStorage, ViewHookStorage};

use worker::{Message, Worker};

use self::worker::{ActionMessage, ProcedureMessage};

pub(crate) struct Runtime {
    input_storage: InputHookStorage,
    view_storage: ViewHookStorage,
    flag_state: Arc<Mutex<FlagState>>,
}

impl Runtime {
    pub(crate) fn new(
        input_storage: InputHookStorage,
        view_storage: ViewHookStorage,
        flag_state: Arc<Mutex<FlagState>>,
    ) -> Self {
        Self {
            input_storage,
            view_storage,
            flag_state,
        }
    }
}

impl Runtime {
    pub(crate) fn start(
        self,
        input_rx: Receiver<(Event, NativeEventHandler)>,
        flag_tx: SyncSender<FlagEvent>,
        flag_rx: Receiver<FlagEvent>,
    ) {
        let Runtime {
            input_storage,
            view_storage,
            flag_state,
        } = self;

        let (worker_tx, worker) = Worker::new(Arc::clone(&flag_state), flag_tx);

        thread::scope(|scope| {
            scope.spawn(|| {
                let (input_rx, mut input_storage) = (input_rx, input_storage);

                for (event, native_handler) in input_rx.iter() {
                    let state = flag_state.lock().unwrap();

                    match event {
                        Event::Button(event) => {
                            let storage = match event.action {
                                ButtonAction::Press => &mut input_storage.remap_on_press,
                                ButtonAction::Release => &mut input_storage.remap_on_release,
                            };
                            let (action, procedure, native) =
                                storage.get(event.target).find(&state);

                            let has_remap_hook = action.is_some() || procedure.is_some();
                            if let Some(action) = action {
                                let msg = Message::Actions(ActionMessage {
                                    event: Some(event),
                                    actions: vec![action],
                                });
                                worker_tx.send(msg).unwrap();
                            }
                            if let Some(procedure) = procedure {
                                let msg = Message::Button(ProcedureMessage {
                                    event,
                                    procedures: vec![procedure],
                                });
                                worker_tx.send(msg).unwrap();
                            }
                            if has_remap_hook {
                                native_handler.handle(native);
                                continue;
                            }

                            let storage = match event.action {
                                ButtonAction::Press => &mut input_storage.on_press,
                                ButtonAction::Release => &mut input_storage.on_release,
                            };
                            let (actions, procedures, native_) =
                                storage.get(event.target).filter(&state);
                            native_handler.handle(native.or(native_));
                            let msg = Message::Actions(ActionMessage {
                                event: Some(event),
                                actions,
                            });
                            worker_tx.send(msg).unwrap();
                            let msg = Message::Button(ProcedureMessage { event, procedures });
                            worker_tx.send(msg).unwrap();
                        }

                        Event::Cursor(event) => {
                            let (actions, procedures, native) =
                                input_storage.mouse_cursor.filter(&state);
                            native_handler.handle(native);
                            let msg = Message::Actions(ActionMessage {
                                event: None,
                                actions,
                            });
                            worker_tx.send(msg).unwrap();
                            let msg = Message::Cursor(ProcedureMessage { event, procedures });
                            worker_tx.send(msg).unwrap();
                        }

                        Event::Wheel(event) => {
                            let (actions, procedures, native) =
                                input_storage.mouse_wheel.filter(&state);
                            native_handler.handle(native);
                            let msg = Message::Actions(ActionMessage {
                                event: None,
                                actions,
                            });
                            worker_tx.send(msg).unwrap();
                            let msg = Message::Wheel(ProcedureMessage { event, procedures });
                            worker_tx.send(msg).unwrap();
                        }
                    }
                }
            });

            scope.spawn(|| {
                let (flag_rx, view_storage) = (flag_rx, view_storage);

                for event in flag_rx.iter() {
                    let (actions, procedures) =
                        view_storage.fetch(event.snapshot, event.flag_index, event.change);
                    let msg = Message::Actions(ActionMessage {
                        event: event.inherited_event,
                        actions,
                    });
                    worker_tx.send(msg).unwrap();
                    let msg = Message::Optional(ProcedureMessage {
                        event: event.inherited_event,
                        procedures,
                    });
                    worker_tx.send(msg).unwrap();
                }
            });
        });

        worker.join();
    }
}
