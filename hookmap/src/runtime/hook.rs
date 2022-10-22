use std::fmt::{Debug, Formatter};
use std::sync::mpsc::Sender;
use std::sync::Arc;

use hookmap_core::button::Button;
use hookmap_core::event::{ButtonEvent, CursorEvent, NativeEventOperation, WheelEvent};

use crate::condition::view::View;
use crate::condition::{
    detector::FlagChange,
    flag::{FlagIndex, FlagState},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FlagEvent {
    pub(crate) flag_index: FlagIndex,
    pub(crate) change: FlagChange,
    pub(crate) snapshot: FlagState,
    pub(crate) inherited_event: Option<ButtonEvent>,
}

#[derive(Clone)]
pub struct RequiredProcedure<E>(pub Arc<dyn Fn(E) + Send + Sync>);
#[derive(Clone)]
pub struct OptionalProcedure<E>(pub Arc<dyn Fn(Option<E>) + Send + Sync>);

impl<E> Debug for RequiredProcedure<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RequiredProcedure").finish_non_exhaustive()
    }
}
impl<E> Debug for OptionalProcedure<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OptionalProcedure").finish_non_exhaustive()
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Procedure<E> {
    Required(RequiredProcedure<E>),
    Optional(OptionalProcedure<E>),
}

impl<E> Procedure<E> {
    fn call(&self, event: E) {
        use Procedure::*;

        match self {
            Required(RequiredProcedure(procedure)) => procedure(event),
            Optional(OptionalProcedure(procedure)) => procedure(Some(event)),
        }
    }

    fn call_optional(&self, event: Option<E>) {
        use Procedure::*;

        match self {
            Required(_) => {
                panic!("Cannot call `Procedure::Required` with optional event.");
            }
            Optional(OptionalProcedure(procedure)) => procedure(event),
        }
    }
}

impl<E> From<RequiredProcedure<E>> for Procedure<E> {
    fn from(procedure: RequiredProcedure<E>) -> Self {
        Self::Required(procedure)
    }
}
impl<E> From<OptionalProcedure<E>> for Procedure<E> {
    fn from(procedure: OptionalProcedure<E>) -> Self {
        Self::Optional(procedure)
    }
}

#[derive(Debug, Clone)]
pub(crate) enum HookAction<E> {
    Procedure {
        procedure: Procedure<E>,
        native: NativeEventOperation,
    },
    RemapPress {
        button: Button,
        flag_index: FlagIndex,
    },
    RemapRelease {
        button: Button,
        flag_index: FlagIndex,
    },
    EnableFlag(FlagIndex),
    DisableFlag(FlagIndex),
    Block,
}

pub(crate) trait IntoInheritedEvent: Sized {
    fn into(self) -> Option<ButtonEvent> {
        None
    }
}
impl IntoInheritedEvent for ButtonEvent {
    fn into(self) -> Option<ButtonEvent> {
        Some(self)
    }
}
impl IntoInheritedEvent for Option<ButtonEvent> {
    fn into(self) -> Option<ButtonEvent> {
        self
    }
}
impl IntoInheritedEvent for CursorEvent {}
impl IntoInheritedEvent for WheelEvent {}

fn run_action<E>(
    action: &HookAction<E>,
    event: impl IntoInheritedEvent,
    state: &FlagState,
    tx: &Sender<FlagEvent>,
) {
    use HookAction::*;

    match action {
        Procedure { .. } => unreachable!(),
        RemapPress { button, flag_index } => {
            let event = FlagEvent {
                flag_index: *flag_index,
                change: FlagChange::Enabled,
                snapshot: state.clone(),
                inherited_event: event.into(),
            };
            tx.send(event).unwrap();
            button.press_recursive();
        }
        RemapRelease { button, flag_index } => {
            button.release_recursive();
            let event = FlagEvent {
                flag_index: *flag_index,
                change: FlagChange::Disabled,
                snapshot: state.clone(),
                inherited_event: event.into(),
            };
            tx.send(event).unwrap();
        }
        EnableFlag(flag_index) => {
            let event = FlagEvent {
                flag_index: *flag_index,
                change: FlagChange::Enabled,
                snapshot: state.clone(),
                inherited_event: event.into(),
            };
            tx.send(event).unwrap();
        }
        DisableFlag(flag_index) => {
            let event = FlagEvent {
                flag_index: *flag_index,
                change: FlagChange::Disabled,
                snapshot: state.clone(),
                inherited_event: event.into(),
            };
            tx.send(event).unwrap();
        }
        Block => {}
    }
}

impl<E: IntoInheritedEvent> HookAction<E> {
    pub(super) fn run(&self, event: E, state: &FlagState, tx: &Sender<FlagEvent>) {
        match self {
            HookAction::Procedure { procedure, .. } => procedure.call(event),
            _ => run_action(self, event, state, tx),
        }
    }
}

impl HookAction<ButtonEvent> {
    pub(super) fn run_optional(
        &self,
        event: Option<ButtonEvent>,
        state: &FlagState,
        tx: &Sender<FlagEvent>,
    ) {
        match self {
            HookAction::Procedure { procedure, .. } => procedure.call_optional(event),
            _ => run_action(self, event, state, tx),
        }
    }
}

impl<E> HookAction<E> {
    pub(super) fn native_event_operation(&self) -> NativeEventOperation {
        use HookAction::*;

        match self {
            Procedure { native, .. } => *native,
            RemapPress { .. } | RemapRelease { .. } => NativeEventOperation::Block,
            EnableFlag(..) | DisableFlag(..) => NativeEventOperation::Dispatch,
            Block => NativeEventOperation::Block,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Hook<E> {
    view: Arc<View>,
    action: Arc<HookAction<E>>,
}

impl<E> Hook<E> {
    pub(crate) fn new(view: Arc<View>, action: Arc<HookAction<E>>) -> Self {
        Self { view, action }
    }

    pub(crate) fn action(&self) -> Arc<HookAction<E>> {
        Arc::clone(&self.action)
    }

    pub(crate) fn is_runnable(&self, state: &FlagState) -> bool {
        self.view.is_enabled(state)
    }
}
