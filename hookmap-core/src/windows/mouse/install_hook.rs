use super::{call_next_hook, set_button_state, MouseEventInfo, DW_EXTRA_INFO};
use crate::{
    common::{
        event::EventBlock,
        handler::{InputHandler, INPUT_HANDLER},
        mouse::{InstallMouseHook, MouseEvent},
    },
    EmulateButtonInput, EmulateMouseCursor, EmulateMouseWheel, MouseCursor, MouseWheel,
};
use once_cell::sync::Lazy;
use std::{
    sync::atomic::{AtomicPtr, Ordering},
    thread,
};
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

    thread::spawn(move || match event_info.unwrap() {
        MouseEventInfo::Button(target, action) => {
            let event = MouseEvent::new(target, action);
            let event_block = INPUT_HANDLER.mouse_button.read().unwrap().emit(event);
            match event_block {
                EventBlock::Block => set_button_state(target.into_vk_code(), action),
                EventBlock::Unblock => target.input(action),
            }
        }
        MouseEventInfo::Wheel(speed) => {
            let event_block = INPUT_HANDLER.mouse_wheel.read().unwrap().emit(speed);
            if event_block == EventBlock::Unblock {
                MouseWheel::rotate(speed);
            }
        }
        MouseEventInfo::Cursor(pos) => {
            let event_block = INPUT_HANDLER.mouse_cursor.read().unwrap().emit(pos);
            if event_block == EventBlock::Unblock {
                MouseCursor::move_abs(pos.0, pos.1);
            }
        }
    });
    1
}

impl InstallMouseHook for InputHandler {
    fn install() {
        let handler =
            unsafe { winuser::SetWindowsHookExW(WH_MOUSE_LL, Some(hook_proc), 0 as HINSTANCE, 0) };
        HHOOK_HANDLER.store(handler, Ordering::SeqCst);
    }
}
