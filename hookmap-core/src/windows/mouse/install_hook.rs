use super::{call_next_hook, MouseEventInfo, DW_EXTRA_INFO};
use crate::common::{
    button::ButtonAction,
    event::{ButtonEvent, EventBlock, BUTTON_EVENT_BLOCK},
    handler::INPUT_HANDLER,
};
use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicPtr, Ordering};
use winapi::{
    ctypes::c_int,
    shared::{
        minwindef::{HINSTANCE, LPARAM, LRESULT, WPARAM},
        windef::HHOOK__,
    },
    um::winuser::{self, MSLLHOOKSTRUCT, WH_MOUSE_LL},
};

static HHOOK_HANDLER: Lazy<AtomicPtr<HHOOK__>> = Lazy::new(AtomicPtr::default);

extern "system" fn hook_proc(code: c_int, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    let mouse_struct = unsafe { *(l_param as *const MSLLHOOKSTRUCT) };
    if mouse_struct.dwExtraInfo & DW_EXTRA_INFO != 0 {
        return call_next_hook(code, w_param, l_param);
    }
    let event_info = MouseEventInfo::new(w_param, mouse_struct);
    if event_info.is_none() {
        return call_next_hook(code, w_param, l_param);
    }

    match event_info.unwrap() {
        MouseEventInfo::Button(target, action) => {
            let event = ButtonEvent::new(target, action);
            INPUT_HANDLER.button.read().unwrap().emit(event);
            match BUTTON_EVENT_BLOCK.get_or_default(target) {
                EventBlock::Unblock => call_next_hook(code, w_param, l_param),
                EventBlock::Block => {
                    match action {
                        ButtonAction::Press => target.assume_pressed(),
                        ButtonAction::Release => target.assume_released(),
                    }
                    1
                }
            }
        }
        MouseEventInfo::Wheel(speed) => {
            INPUT_HANDLER.mouse_wheel.read().unwrap().emit(speed);
            call_next_hook(code, w_param, l_param)
        }
        MouseEventInfo::Cursor(pos) => {
            INPUT_HANDLER.mouse_cursor.read().unwrap().emit(pos);
            call_next_hook(code, w_param, l_param)
        }
    }
}

pub(in crate::windows) fn install_hook() {
    let handler =
        unsafe { winuser::SetWindowsHookExW(WH_MOUSE_LL, Some(hook_proc), 0 as HINSTANCE, 0) };
    HHOOK_HANDLER.store(handler, Ordering::SeqCst);
}
