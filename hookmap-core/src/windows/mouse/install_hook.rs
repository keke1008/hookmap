use super::{call_next_hook, set_button_state, MouseEventInfo, DW_EXTRA_INFO};
use crate::common::{
    event::EventBlock,
    handler::{InputHandler, INPUT_HANDLER},
    mouse::{InstallMouseHook, MouseEvent},
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

    let event_block = match event_info {
        Some(MouseEventInfo::Button(target, action)) => {
            let event = MouseEvent::new(target, action);
            let event_block = INPUT_HANDLER.mouse_button.lock().unwrap().emit(event);
            if event_block == EventBlock::Block {
                set_button_state(target.into_vk_code(), action);
            }
            event_block
        }
        Some(MouseEventInfo::Wheel(speed)) => INPUT_HANDLER.mouse_wheel.lock().unwrap().emit(speed),
        Some(MouseEventInfo::Cursor(pos)) => INPUT_HANDLER.mouse_cursor.lock().unwrap().emit(pos),
        _ => EventBlock::Unblock,
    };
    match event_block {
        EventBlock::Block => 1,
        EventBlock::Unblock => call_next_hook(code, w_param, l_param),
    }
}

impl InstallMouseHook for InputHandler {
    fn install() {
        let handler =
            unsafe { winuser::SetWindowsHookExW(WH_MOUSE_LL, Some(hook_proc), 0 as HINSTANCE, 0) };
        HHOOK_HANDLER.store(handler, Ordering::SeqCst);
    }
}
