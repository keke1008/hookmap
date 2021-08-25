use super::{call_next_hook, DW_EXTRA_INFO};
use crate::common::{
    button::{Button, ButtonAction},
    event::{ButtonEvent, EventBlock},
    INPUT_HANDLER,
};
use once_cell::sync::Lazy;
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

pub(super) fn into_button_action(event_info: KBDLLHOOKSTRUCT) -> ButtonAction {
    match event_info.flags >> 7 {
        0 => ButtonAction::Press,
        _ => ButtonAction::Release,
    }
}

extern "system" fn hook_proc(code: c_int, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    let event_info = unsafe { *(l_param as *const KBDLLHOOKSTRUCT) };
    if event_info.dwExtraInfo & DW_EXTRA_INFO != 0 {
        return call_next_hook(code, w_param, l_param);
    }
    let target = Button::from_vkcode(event_info.vkCode);
    let action = into_button_action(event_info);
    match action {
        ButtonAction::Press => target.assume_pressed(),
        ButtonAction::Release => target.assume_released(),
    };
    let event = ButtonEvent::new(target, action);
    let event_block = INPUT_HANDLER.button.emit(event);
    match event_block {
        EventBlock::Unblock => call_next_hook(code, w_param, l_param),
        EventBlock::Block => 1,
    }
}

pub(crate) fn install_hook() {
    let handler =
        unsafe { winuser::SetWindowsHookExW(WH_KEYBOARD_LL, Some(hook_proc), 0 as HINSTANCE, 0) };
    HHOOK_HANDLER.store(handler, Ordering::SeqCst);
}
