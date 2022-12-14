use std::fmt::Debug;
use std::sync::mpsc::SyncSender;
use std::sync::{Arc, Mutex};

use hookmap_core::button::Button;
use hookmap_core::event::{ButtonEvent, CursorEvent, NativeEventOperation, WheelEvent};

use crate::condition::flag::FlagState;
use crate::condition::view::View;

use crate::storage::action::FlagEvent;
use crate::storage::procedure::{OptionalProcedure, RequiredProcedure};
use crate::storage::ViewHookStorage;

use super::flag::Flag;
use super::registrar::{self, InputHookRegistrar};
use super::shared::Shared;

#[derive(Debug)]
pub struct HookRegistrar {
    input_registrar: Shared<InputHookRegistrar>,
    view_storage: Shared<ViewHookStorage>,
    state: Arc<Mutex<FlagState>>,
    native: NativeEventOperation,
}

impl HookRegistrar {
    fn create_context(&self, view: Arc<View>) -> registrar::Context {
        registrar::Context {
            state: Arc::clone(&self.state),
            view,
            native: self.native,
        }
    }

    pub(super) fn new(
        input_registrar: Shared<InputHookRegistrar>,
        view_storage: Shared<ViewHookStorage>,
        state: Arc<Mutex<FlagState>>,
    ) -> Self {
        Self {
            input_registrar,
            view_storage,
            state,
            native: NativeEventOperation::Dispatch,
        }
    }

    pub fn remap(&self, view: impl Into<Arc<View>>, target: Button, behavior: Button) -> &Self {
        self.input_registrar.get_mut().remap(
            target,
            behavior,
            &self.create_context(view.into()),
            &mut self.view_storage.get_mut(),
        );
        self
    }

    pub fn on_press(
        &self,
        view: impl Into<Arc<View>>,
        target: Button,
        procedure: impl Into<RequiredProcedure<ButtonEvent>>,
    ) -> &Self {
        self.input_registrar.get_mut().on_press(
            target,
            procedure.into(),
            &self.create_context(view.into()),
        );
        self
    }

    pub fn on_release(
        &self,
        view: impl Into<Arc<View>>,
        target: Button,
        procedure: impl Into<RequiredProcedure<ButtonEvent>>,
    ) -> &Self {
        self.input_registrar.get_mut().on_release(
            target,
            procedure.into(),
            &self.create_context(view.into()),
        );
        self
    }

    pub fn on_release_certainly(
        &self,
        view: impl Into<Arc<View>>,
        target: Button,
        procedure: impl Into<OptionalProcedure<ButtonEvent>>,
    ) -> &Self {
        self.input_registrar.get_mut().on_release_certainly(
            target,
            procedure.into(),
            &self.create_context(view.into()),
            &mut self.view_storage.get_mut(),
        );
        self
    }

    pub fn mouse_cursor(
        &self,
        view: impl Into<Arc<View>>,
        procedure: impl Into<RequiredProcedure<CursorEvent>>,
    ) -> &Self {
        self.input_registrar
            .get_mut()
            .mouse_cursor(procedure.into(), &self.create_context(view.into()));
        self
    }

    pub fn mouse_wheel(
        &self,
        view: impl Into<Arc<View>>,
        procedure: impl Into<RequiredProcedure<WheelEvent>>,
    ) -> &Self {
        self.input_registrar
            .get_mut()
            .mouse_wheel(procedure.into(), &self.create_context(view.into()));
        self
    }

    pub fn disable(&self, view: impl Into<Arc<View>>, target: Button) -> &Self {
        self.input_registrar
            .get_mut()
            .disable(target, &self.create_context(view.into()));
        self
    }

    pub fn on_view_enabled(
        &self,
        view: impl Into<Arc<View>>,
        procedure: impl Into<OptionalProcedure<ButtonEvent>>,
    ) -> &Self {
        self.view_storage
            .get_mut()
            .add_procedure_on_enabled(view.into(), procedure.into());
        self
    }

    pub fn on_view_disabled(
        &self,
        view: impl Into<Arc<View>>,
        procedure: impl Into<OptionalProcedure<ButtonEvent>>,
    ) -> &Self {
        self.view_storage
            .get_mut()
            .add_procedure_on_disabled(view.into(), procedure.into());
        self
    }

    fn clone_with_native(&self, native: NativeEventOperation) -> Self {
        Self {
            input_registrar: self.input_registrar.clone(),
            view_storage: self.view_storage.clone(),
            state: Arc::clone(&self.state),
            native,
        }
    }

    pub fn block(&self) -> Self {
        self.clone_with_native(NativeEventOperation::Block)
    }

    pub fn dispatch(&self) -> Self {
        self.clone_with_native(NativeEventOperation::Dispatch)
    }
}

#[derive(Debug)]
pub struct ViewContext {
    state: Arc<Mutex<FlagState>>,
    flag_tx: SyncSender<FlagEvent>,
    root_view: Arc<View>,
    current_view: Arc<View>,
}

impl ViewContext {
    pub(super) fn new(
        state: Arc<Mutex<FlagState>>,
        flag_tx: SyncSender<FlagEvent>,
        root_view: Arc<View>,
        current_view: Arc<View>,
    ) -> Self {
        Self {
            state,
            flag_tx,
            root_view,
            current_view,
        }
    }

    fn replace_current_view(&self, current_view: Arc<View>) -> Self {
        Self {
            state: Arc::clone(&self.state),
            flag_tx: self.flag_tx.clone(),
            root_view: self.root_view(),
            current_view,
        }
    }

    pub fn root_view(&self) -> Arc<View> {
        Arc::clone(&self.root_view)
    }

    pub fn current_view(&self) -> Arc<View> {
        Arc::clone(&self.current_view)
    }

    pub fn create_flag(&self, init_state: bool) -> Flag {
        let index = self.state.lock().unwrap().create_flag(init_state);
        Flag::new(index, Arc::clone(&self.state), self.flag_tx.clone())
    }
}

pub trait HotkeyCondition {
    fn view(&mut self, hook: &mut HookRegistrar, context: &mut ViewContext) -> Arc<View>;
}

impl<T: HotkeyCondition> HotkeyCondition for Box<T> {
    fn view(&mut self, hook: &mut HookRegistrar, context: &mut ViewContext) -> Arc<View> {
        T::view(self, hook, context)
    }
}

impl<T: HotkeyCondition> HotkeyCondition for &mut T {
    fn view(&mut self, hook: &mut HookRegistrar, context: &mut ViewContext) -> Arc<View> {
        T::view(self, hook, context)
    }
}

impl HotkeyCondition for Button {
    fn view(&mut self, hook: &mut HookRegistrar, context: &mut ViewContext) -> Arc<View> {
        let f = context.create_flag(false);
        let v = View::new().enabled(&f);

        let f = f.into();
        let f_ = Arc::clone(&f);
        hook.on_press(context.root_view(), *self, move |_: ButtonEvent| {
            f.enable();
        })
        .on_release(context.root_view(), *self, move |_: ButtonEvent| {
            f_.disable();
        });

        v.into()
    }
}

#[derive(Debug)]
pub struct Inversed<T> {
    condition: T,
}

impl<T> Inversed<T> {
    pub fn new(condition: T) -> Self {
        Self { condition }
    }
}

impl<T: HotkeyCondition> HotkeyCondition for Inversed<T> {
    fn view(&mut self, hook: &mut HookRegistrar, context: &mut ViewContext) -> Arc<View> {
        self.condition.view(hook, context).inversed().into()
    }
}

pub struct Multi<'a> {
    conditions: Vec<&'a mut dyn HotkeyCondition>,
}

impl<'a> Multi<'a> {
    pub fn new(conditions: Vec<&'a mut dyn HotkeyCondition>) -> Self {
        Self { conditions }
    }
}

impl HotkeyCondition for Multi<'_> {
    fn view(&mut self, hook: &mut HookRegistrar, context: &mut ViewContext) -> Arc<View> {
        self.conditions
            .iter_mut()
            .fold(View::new(), |acc, condition| {
                let mut context = context.replace_current_view(Arc::new(acc.clone()));
                View::new()
                    .merge(&acc)
                    .merge(&condition.view(hook, &mut context))
            })
            .into()
    }
}

#[macro_export]
macro_rules! multi {
    (@inner [ $( $acc:expr ),* ] !$arg:expr $(, $( $rest:tt )* )? ) => {
        $crate::multi!(
            @inner
            [
                $( $acc, )*
                &mut $crate::hotkey::condition::Inversed::new($arg)
            ]
            $( $( $rest )* )?
        )
    };

    (@inner [ $( $acc:expr ),* ] $arg:expr $(, $( $rest:tt )* )? ) => {
        $crate::multi!(
            @inner
            [
                $( $acc, )*
                &mut $arg
            ]
            $( $( $rest )* )?
        )
    };

    (@inner $acc:tt ) => {
        $crate::hotkey::condition::Multi::new(vec!$acc)
    };

    [ $($args:tt)* ] => {
        $crate::multi!(@inner [] $($args)* )
    };
}
