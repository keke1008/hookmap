use crate::common::{
    event::{BlockInput, EventHandlerExt},
    keyboard::{Key, KeyboardAction, KeyboardEventHandler, KEYBOARD_HANDLER},
};
use once_cell::sync::Lazy;
use std::{
    mem::MaybeUninit,
    sync::atomic::{AtomicPtr, Ordering},
};
use winapi::{
    ctypes::c_int,
    shared::{
        minwindef::{HINSTANCE, LPARAM, LRESULT, WPARAM},
        windef::{HHOOK__, HWND},
    },
    um::winuser::{self, KBDLLHOOKSTRUCT, WH_KEYBOARD_LL},
};

static HHOOK_HANDLER: Lazy<AtomicPtr<HHOOK__>> = Lazy::new(AtomicPtr::default);

extern "system" fn hook_proc(code: c_int, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    let event_info = unsafe { *(l_param as *const KBDLLHOOKSTRUCT) };
    let kind = event_info.vkCode.into();
    let action = if event_info.flags >> 7 == 0 {
        KeyboardAction::Press
    } else {
        KeyboardAction::Release
    };
    match KEYBOARD_HANDLER.emit(kind, action) {
        BlockInput::Block => 0,
        BlockInput::Unblock => unsafe {
            winuser::CallNextHookEx(HHOOK_HANDLER.load(Ordering::SeqCst), code, w_param, l_param)
        },
    }
}

impl EventHandlerExt<Key, KeyboardAction> for KeyboardEventHandler {
    fn install_hook() -> Result<(), ()> {
        let handler = unsafe {
            winuser::SetWindowsHookExW(WH_KEYBOARD_LL, Some(hook_proc), 0 as HINSTANCE, 0)
        };
        if handler.is_null() {
            return Err(());
        }
        HHOOK_HANDLER.store(handler, Ordering::SeqCst);
        unsafe { winuser::GetMessageW(MaybeUninit::uninit().assume_init(), 0 as HWND, 0, 0) };
        Ok(())
    }

    fn uninstall_hook() -> Result<(), ()> {
        let result = unsafe {
            winuser::UnhookWindowsHookEx(HHOOK_HANDLER.swap(std::ptr::null_mut(), Ordering::SeqCst))
        };
        if result == 0 {
            Err(())
        } else {
            Ok(())
        }
    }
}
