use std::sync::mpsc::{self, SyncSender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

use hookmap_core::event::{ButtonEvent, CursorEvent, WheelEvent};

use crate::condition::flag::FlagState;
use crate::storage::action::{FlagEvent, HookAction};
use crate::storage::procedure::Procedure;

#[derive(Debug)]
pub(super) struct ProcedureMessage<E, T> {
    pub(super) event: E,
    pub(super) procedures: Vec<Arc<Procedure<T>>>,
}

macro_rules! impl_procedure_message {
    ($event:ty) => {
        impl ProcedureMessage<$event, $event> {
            pub(super) fn run(&self) {
                for procedure in &self.procedures {
                    procedure.call(self.event)
                }
            }
        }
    };
}
impl_procedure_message!(ButtonEvent);
impl_procedure_message!(CursorEvent);
impl_procedure_message!(WheelEvent);

impl ProcedureMessage<Option<ButtonEvent>, ButtonEvent> {
    fn run(&self) {
        for procedure in &self.procedures {
            procedure.call_optional(self.event)
        }
    }
}

#[derive(Debug)]
pub(super) struct ActionMessage {
    pub(super) event: Option<ButtonEvent>,
    pub(super) actions: Vec<Arc<HookAction>>,
}

impl ActionMessage {
    fn run(&self, state: &mut FlagState, tx: &SyncSender<FlagEvent>) {
        for action in &self.actions {
            action.run(self.event, state, tx);
        }
    }
}

#[derive(Debug)]
pub(super) enum Message {
    Button(ProcedureMessage<ButtonEvent, ButtonEvent>),
    Optional(ProcedureMessage<Option<ButtonEvent>, ButtonEvent>),
    Cursor(ProcedureMessage<CursorEvent, CursorEvent>),
    Wheel(ProcedureMessage<WheelEvent, WheelEvent>),
    Actions(ActionMessage),
}

#[derive(Debug)]
pub(super) struct Worker {
    handle: JoinHandle<()>,
}

impl Worker {
    pub(super) fn new(
        state: Arc<Mutex<FlagState>>,
        flag_tx: SyncSender<FlagEvent>,
    ) -> (SyncSender<Message>, Self) {
        let (tx, rx) = mpsc::sync_channel(32);
        let handle = thread::spawn(move || {
            for msg in rx.iter() {
                let mut state = state.lock().unwrap().clone();
                match msg {
                    Message::Button(procedures) => procedures.run(),
                    Message::Optional(procedures) => procedures.run(),
                    Message::Cursor(procedures) => procedures.run(),
                    Message::Wheel(procedures) => procedures.run(),
                    Message::Actions(actions) => actions.run(&mut state, &flag_tx),
                }
            }
        });

        (tx, Worker { handle })
    }

    pub(super) fn join(self) {
        self.handle.join().unwrap();
    }
}
