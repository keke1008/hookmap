use std::fmt::Debug;
use std::sync::mpsc::SyncSender;
use std::sync::{Arc, Mutex};

use hookmap_core::button::Button;
use hookmap_core::event::{ButtonEvent, NativeEventOperation};

use crate::condition::flag::FlagState;
use crate::condition::view::{View, ViewBuilder};
use crate::runtime::hook::{FlagEvent, HookAction, OptionalProcedure};

use super::flag::Flag;
use super::storage::FlagHookStorage;
use super::Hotkey;

#[derive(Debug)]
pub struct FlagHookRegistrar {
    storage: FlagHookStorage,
    state: Arc<Mutex<FlagState>>,
    flag_tx: SyncSender<FlagEvent>,
}

impl FlagHookRegistrar {
    fn new(flag_tx: SyncSender<FlagEvent>) -> Self {
        Self {
            storage: Default::default(),
            state: Default::default(),
            flag_tx,
        }
    }
}

impl FlagHookRegistrar {
    pub fn create_flag(&self, init_state: bool) -> Flag {
        let index = self.state.lock().unwrap().create_flag(init_state);
        Flag::new(index, Arc::clone(&self.state), self.flag_tx.clone())
    }

    pub fn on_enabled(
        &mut self,
        view: Arc<View>,
        procedure: impl Into<OptionalProcedure<ButtonEvent>>,
    ) {
        self.storage.register_on_enabled(
            view,
            Arc::new(HookAction::Procedure {
                procedure: procedure.into().into(),
                native: NativeEventOperation::Dispatch,
            }),
        );
    }

    pub fn on_disabled(
        &mut self,
        view: Arc<View>,
        procedure: impl Into<OptionalProcedure<ButtonEvent>>,
    ) {
        self.storage.register_on_disabled(
            view,
            Arc::new(HookAction::Procedure {
                procedure: procedure.into().into(),
                native: NativeEventOperation::Dispatch,
            }),
        )
    }
}

pub trait HotkeyCondition {
    fn view(&mut self, hotkey: &mut Hotkey, flag: &mut FlagHookRegistrar) -> Arc<View>;
}

impl HotkeyCondition for Button {
    fn view(&mut self, hotkey: &mut Hotkey, flag: &mut FlagHookRegistrar) -> Arc<View> {
        let f = flag.create_flag(false);
        let view = ViewBuilder::new().enabled(&f).build();

        let f = f.into();
        let f_ = Arc::clone(&f);
        hotkey
            .on_press(*self, move |_| f.enable())
            .on_release(*self, move |_| f_.disable());

        view.into()
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
    fn view(&mut self, hotkey: &mut Hotkey, flag: &mut FlagHookRegistrar) -> Arc<View> {
        self.condition.view(hotkey, flag).inversed().into()
    }
}

pub struct Multi<'a> {
    conditions: Vec<Box<dyn HotkeyCondition + 'a>>,
}

impl<'a> Multi<'a> {
    pub fn new(conditions: Vec<Box<dyn HotkeyCondition + 'a>>) -> Self {
        Self { conditions }
    }
}

impl<'a> HotkeyCondition for Multi<'a> {
    fn view(&mut self, hotkey: &mut Hotkey, flag: &mut FlagHookRegistrar) -> Arc<View> {
        self.conditions
            .iter_mut()
            .fold(ViewBuilder::new(), |builder, condition| {
                builder.merge(&condition.view(hotkey, flag))
            })
            .build()
            .into()
    }
}

#[macro_export]
macro_rules! multi {
    (@inner [ $( $acc:expr ),* ] !$arg:expr $(, $( $rest:tt )* )? ) => {
        modifiers!(
            @inner
            [
                $( $acc, )*
                $crate::hotkey::condition::Inversed(Box::new($arg))
            ]
            $( $( $rest )* )?
        )
    };

    (@inner [ $( $acc:expr ),* ] $arg:expr $(, $( $rest:tt )* )? ) => {
        modifiers!(
            @inner
            [
                $( $acc, )*
                Box::new($arg)
            ]
            $( $( $rest )* )?
        )
    };

    (@inner $acc:tt ) => {
        $crate::hotkey::conditions::Multi::new(vec!$acc)
    };

    [ $($args:tt)* ] => {
        $crate::modifiers!(@inner [] $($args)* )
    };
}
