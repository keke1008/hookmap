use std::sync::{Arc, Mutex};

use hookmap_core::button::Button;
use hookmap_core::event::{ButtonEvent, CursorEvent, WheelEvent};

use crate::condition::flag::FlagState;
use crate::condition::view::{View, ViewBuilder};
use crate::runtime::{hook::HookAction, interruption::InterruptionStorage};

use super::storage::{FlagHookStorage, InputHookStorage};

#[derive(Debug, Default)]
pub(super) struct HotkeyStorage {
    input_storage: InputHookStorage,
    interruption_storage: Arc<Mutex<InterruptionStorage>>,
    flag_storage: FlagHookStorage,
}

impl HotkeyStorage {
    pub(super) fn destruct(
        self,
    ) -> (
        InputHookStorage,
        Arc<Mutex<InterruptionStorage>>,
        FlagHookStorage,
    ) {
        (
            self.input_storage,
            self.interruption_storage,
            self.flag_storage,
        )
    }

    pub(super) fn remap(
        &mut self,
        view: Arc<View>,
        target: Button,
        behavior: Button,
        state: &mut FlagState,
    ) {
        let flag = state.create_flag(false);
        let enabled_view = Arc::new(ViewBuilder::new().merge(&*view).enabled(flag).build());

        self.input_storage.on_press_exclusive.register_specific(
            view,
            target,
            Arc::new(HookAction::RemapPress {
                button: behavior,
                flag_index: flag,
            }),
        );

        self.input_storage.on_release.register_specific(
            enabled_view,
            target,
            Arc::new(HookAction::RemapRelease {
                button: behavior,
                flag_index: flag,
            }),
        );
    }

    pub(super) fn on_press(
        &mut self,
        view: Arc<View>,
        target: Button,
        action: HookAction<ButtonEvent>,
    ) {
        self.input_storage
            .on_press
            .register_specific(view, target, Arc::new(action));
    }

    pub(super) fn on_release(
        &mut self,
        view: Arc<View>,
        target: Button,
        action: HookAction<ButtonEvent>,
        state: &mut FlagState,
    ) {
        let flag = state.create_flag(false);
        let enabled_view = Arc::new(ViewBuilder::new().merge(&*view).enabled(flag).build());

        self.input_storage.on_press.register_specific(
            view,
            target,
            Arc::new(HookAction::EnableFlag(flag)),
        );

        self.input_storage.on_release.register_specific(
            Arc::clone(&enabled_view),
            target,
            Arc::new(HookAction::DisableFlag(flag)),
        );

        self.flag_storage
            .register_on_inactivated(enabled_view, Arc::new(action));
    }

    pub(super) fn disable(&mut self, view: Arc<View>, target: Button) {
        self.input_storage.on_press.register_specific(
            Arc::clone(&view),
            target,
            Arc::new(HookAction::Block),
        );

        self.input_storage.on_release.register_specific(
            Arc::clone(&view),
            target,
            Arc::new(HookAction::Block),
        );
    }

    pub(super) fn mouse_cursor(&mut self, view: Arc<View>, action: HookAction<CursorEvent>) {
        self.input_storage
            .mouse_cursor
            .register(view, Arc::new(action));
    }

    pub(super) fn mouse_wheel(&mut self, view: Arc<View>, action: HookAction<WheelEvent>) {
        self.input_storage
            .mouse_wheel
            .register(view, Arc::new(action));
    }
}
