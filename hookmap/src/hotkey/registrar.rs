use std::sync::{Arc, Mutex};

use hookmap_core::button::Button;
use hookmap_core::event::{ButtonEvent, CursorEvent, NativeEventOperation, WheelEvent};

use crate::condition::flag::FlagState;
use crate::condition::view::View;
use crate::storage::action::HookAction;
use crate::storage::procedure::{OptionalProcedure, Procedure, ProcedureHook, RequiredProcedure};
use crate::storage::{InputHookStorage, ViewHookStorage};

#[derive(Debug, Clone)]
pub(super) struct Context {
    pub(super) state: Arc<Mutex<FlagState>>,
    pub(super) view: Arc<View>,
    pub(super) native: NativeEventOperation,
}

impl Context {
    pub(super) fn replace_view(&self, view: Arc<View>) -> Self {
        Self {
            view,
            ..self.clone()
        }
    }

    pub(super) fn replace_native(&self, native: NativeEventOperation) -> Self {
        Self {
            native,
            ..self.clone()
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            view: Arc::default(),
            state: Arc::default(),
            native: NativeEventOperation::Dispatch,
        }
    }
}

#[derive(Debug, Default)]
pub(super) struct InputHookRegistrar {
    storage: InputHookStorage,
}

impl InputHookRegistrar {
    pub(super) fn into_inner(self) -> InputHookStorage {
        self.storage
    }

    pub(super) fn remap(
        &mut self,
        target: Button,
        behavior: Button,
        context: &Context,
        view_storage: &mut ViewHookStorage,
    ) {
        let flag = context.state.lock().unwrap().create_flag(false);
        let view = View::new().merge(&*context.view).enabled(flag).into();

        self.storage.remap_on_press.get(target).add_action(
            Arc::clone(&context.view),
            HookAction::RemapPress {
                button: behavior,
                flag_index: flag,
            },
        );

        self.storage
            .remap_on_press
            .get(target)
            .add_action(Arc::clone(&view), HookAction::DisableFlag(flag));

        view_storage.add_action_on_disabled(
            view,
            HookAction::RemapRelease {
                button: target,
                flag_index: flag,
            },
        );
    }

    pub(super) fn on_press(
        &mut self,
        target: Button,
        procedure: RequiredProcedure<ButtonEvent>,
        context: &Context,
    ) {
        self.storage.on_press.get(target).add_procedure(
            Arc::clone(&context.view),
            ProcedureHook::new(Procedure::Required(procedure), context.native),
        )
    }

    pub(super) fn on_release(
        &mut self,
        target: Button,
        procedure: RequiredProcedure<ButtonEvent>,
        context: &Context,
    ) {
        self.storage.on_release.get(target).add_procedure(
            Arc::clone(&context.view),
            ProcedureHook::new(Procedure::Required(procedure), context.native),
        );
    }

    pub(super) fn on_release_certainly(
        &mut self,
        target: Button,
        procedure: OptionalProcedure<ButtonEvent>,
        context: &Context,
        view_storage: &mut ViewHookStorage,
    ) {
        let flag = context.state.lock().unwrap().create_flag(false);
        let view = View::new().merge(&*context.view).enabled(flag).into();

        self.storage
            .on_press
            .get(target)
            .add_action(Arc::clone(&context.view), HookAction::EnableFlag(flag));

        self.storage
            .on_release
            .get(target)
            .add_action(Arc::clone(&context.view), HookAction::DisableFlag(flag));

        view_storage.add_procedure_on_disabled(view, procedure);
    }

    pub(super) fn disable(&mut self, target: Button, context: &Context) {
        self.storage
            .on_press
            .get(target)
            .add_action(Arc::clone(&context.view), HookAction::Block);
        self.storage
            .on_release
            .get(target)
            .add_action(Arc::clone(&context.view), HookAction::Block);
    }

    pub(super) fn mouse_cursor(
        &mut self,
        procedure: RequiredProcedure<CursorEvent>,
        context: &Context,
    ) {
        self.storage.mouse_cursor.add_procedure(
            Arc::clone(&context.view),
            ProcedureHook::new(Procedure::Required(procedure), context.native),
        );
    }

    pub(super) fn mouse_wheel(
        &mut self,
        procedure: RequiredProcedure<WheelEvent>,
        context: &Context,
    ) {
        self.storage.mouse_wheel.add_procedure(
            Arc::clone(&context.view),
            ProcedureHook::new(Procedure::Required(procedure), context.native),
        );
    }
}
