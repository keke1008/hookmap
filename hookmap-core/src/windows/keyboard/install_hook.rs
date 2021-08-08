use super::{call_next_hook, conversion::VkCode, DW_EXTRA_INFO};
use crate::common::{
    button::ButtonAction,
    event::{ButtonEvent, EventBlock},
    BUTTON_EVENT_BLOCK, INPUT_HANDLER,
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

extern "system" fn hook_proc(code: c_int, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    let event_info = unsafe { *(l_param as *const KBDLLHOOKSTRUCT) };
    if event_info.dwExtraInfo & DW_EXTRA_INFO != 0 {
        return call_next_hook(code, w_param, l_param);
    }

    let target = VkCode(event_info.vkCode).into();
    let action = match event_info.flags >> 7 {
        0 => ButtonAction::Press,
        _ => ButtonAction::Release,
    };
    let event = ButtonEvent::new(target, action);
    INPUT_HANDLER.button.emit(event);
    match BUTTON_EVENT_BLOCK.get_or_default(target) {
        EventBlock::Unblock => return call_next_hook(code, w_param, l_param),
        EventBlock::Block => match action {
            ButtonAction::Press => target.assume_pressed(),
            ButtonAction::Release => target.assume_released(),
        },
    }
    1
}

pub(crate) fn install_hook() {
    let handler =
        unsafe { winuser::SetWindowsHookExW(WH_KEYBOARD_LL, Some(hook_proc), 0 as HINSTANCE, 0) };
    HHOOK_HANDLER.store(handler, Ordering::SeqCst);
}
