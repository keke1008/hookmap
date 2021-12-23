use super::{call_next_hook, IGNORED_DW_EXTRA_INFO};
use crate::common::{
    button::{Button, ButtonAction},
    event::{ButtonEvent, Event, EventProvider, NativeEventOperation},
};
use once_cell::sync::{Lazy, OnceCell};
use std::sync::atomic::{AtomicPtr, Ordering};
use winapi::{
    ctypes::c_int,
    shared::{
        minwindef::{HINSTANCE, LPARAM, LRESULT, WPARAM},
        windef::HHOOK__,
    },
    um::winuser::{self, KBDLLHOOKSTRUCT, WH_KEYBOARD_LL},
};

static HHOOK_HANDLER: Lazy<AtomicPtr<HHOOK__>> = Lazy::new(AtomicPtr::default);

static EVENT_PROVIDER: OnceCell<EventProvider> = OnceCell::new();

pub(super) fn into_button_action(event_info: KBDLLHOOKSTRUCT) -> ButtonAction {
    match event_info.flags >> 7 {
        0 => ButtonAction::Press,
        _ => ButtonAction::Release,
    }
}

extern "system" fn hook_proc(code: c_int, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    let event_info = unsafe { *(l_param as *const KBDLLHOOKSTRUCT) };
    if event_info.dwExtraInfo & IGNORED_DW_EXTRA_INFO != 0 {
        return call_next_hook(code, w_param, l_param);
    }
    let target = if let Some(button) = Button::from_hook_struct(&event_info) {
        button
    } else {
        return call_next_hook(code, w_param, l_param);
    };
    let action = into_button_action(event_info);
    match action {
        ButtonAction::Press => target.assume_pressed(),
        ButtonAction::Release => target.assume_released(),
    };
    let event = ButtonEvent::new(target, action);
    let operation = EVENT_PROVIDER.get().unwrap().send(Event::Button(event));
    match operation {
        NativeEventOperation::Dispatch => call_next_hook(code, w_param, l_param),
        NativeEventOperation::Block => 1,
    }
}

pub(in crate::sys::windows) fn install_hook(event_provider: EventProvider) {
    EVENT_PROVIDER.set(event_provider).unwrap();
    let handler =
        unsafe { winuser::SetWindowsHookExW(WH_KEYBOARD_LL, Some(hook_proc), 0 as HINSTANCE, 0) };
    HHOOK_HANDLER.store(handler, Ordering::SeqCst);
}