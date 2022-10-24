use std::sync::Arc;

use hookmap_core::event::{ButtonEvent, CursorEvent, WheelEvent};

use crate::condition::flag::FlagState;

use super::hook::{FlagEvent, HookAction};

pub(crate) trait InputHookFetcher: Send {
    fn fetch_exclusive_button_hook(
        &self,
        event: ButtonEvent,
        state: &FlagState,
    ) -> Option<Arc<HookAction<ButtonEvent>>>;

    fn fetch_button_hook(
        &self,
        event: ButtonEvent,
        state: &FlagState,
    ) -> Vec<Arc<HookAction<ButtonEvent>>>;

    fn fetch_mouse_cursor_hook(&self, state: &FlagState) -> Vec<Arc<HookAction<CursorEvent>>>;

    fn fetch_mouse_wheel_hook(&self, state: &FlagState) -> Vec<Arc<HookAction<WheelEvent>>>;
}

pub(crate) trait FlagHookFetcher: Send {
    fn fetch(&self, event: FlagEvent) -> Vec<Arc<HookAction<ButtonEvent>>>;
}
