use std::sync::mpsc::SyncSender;

use crate::condition::detector::FlagChange;
use crate::condition::flag::{FlagIndex, FlagState};

use hookmap_core::event::NativeEventOperation;
use hookmap_core::{button::Button, event::ButtonEvent};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FlagEvent {
    pub(crate) flag_index: FlagIndex,
    pub(crate) change: FlagChange,
    pub(crate) snapshot: FlagState,
    pub(crate) inherited_event: Option<ButtonEvent>,
}

#[derive(Debug)]
pub(crate) enum HookAction {
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

fn send_event(
    tx: &SyncSender<FlagEvent>,
    flag_index: FlagIndex,
    change: FlagChange,
    state: &mut FlagState,
    event: Option<ButtonEvent>,
) {
    let event = FlagEvent {
        flag_index,
        change,
        snapshot: state.clone(),
        inherited_event: event,
    };
    tx.send(event).unwrap();
}

impl HookAction {
    pub(crate) fn run(
        &self,
        event: Option<ButtonEvent>,
        state: &mut FlagState,
        tx: &SyncSender<FlagEvent>,
    ) {
        use HookAction::*;

        match *self {
            RemapPress { button, flag_index } => {
                send_event(tx, flag_index, FlagChange::Enabled, state, event);
                button.press_recursive();
            }
            RemapRelease { button, flag_index } => {
                send_event(tx, flag_index, FlagChange::Disabled, state, event);
                button.release_recursive();
            }
            EnableFlag(flag_index) => {
                state.enable(flag_index);
                send_event(tx, flag_index, FlagChange::Enabled, state, event);
            }
            DisableFlag(flag_index) => {
                state.disable(flag_index);
                send_event(tx, flag_index, FlagChange::Disabled, state, event);
            }
            Block => {}
        }
    }

    pub(crate) fn native(&self) -> NativeEventOperation {
        use HookAction::*;
        match self {
            RemapPress { .. } | RemapRelease { .. } | Block => NativeEventOperation::Block,
            EnableFlag(..) | DisableFlag(..) => NativeEventOperation::Dispatch,
        }
    }
}