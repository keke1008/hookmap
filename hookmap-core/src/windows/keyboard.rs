use crate::common::{
    event::EventBlock,
    handler::HookInstallable,
    keyboard::{EmulateKeyboardInput, Key, KeyboardAction, KeyboardEventHandler, KEYBOARD_HANDLER},
};
use once_cell::sync::Lazy;
use std::{
    mem::{self, MaybeUninit},
    sync::atomic::{AtomicPtr, Ordering},
};
use winapi::{
    ctypes::c_int,
    shared::{
        minwindef::{HINSTANCE, LPARAM, LRESULT, UINT, WPARAM},
        windef::{HHOOK__, HWND},
    },
    um::winuser::{
        self, INPUT, INPUT_KEYBOARD, KBDLLHOOKSTRUCT, KEYBDINPUT, KEYEVENTF_KEYUP, LPINPUT,
        WH_KEYBOARD_LL,
    },
};

static HHOOK_HANDLER: Lazy<AtomicPtr<HHOOK__>> = Lazy::new(AtomicPtr::default);

extern "system" fn hook_proc(code: c_int, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    let event_info = unsafe { *(l_param as *const KBDLLHOOKSTRUCT) };
    let target = event_info.vkCode.into();
    let action = if event_info.flags >> 7 == 0 {
        KeyboardAction::Press
    } else {
        KeyboardAction::Release
    };
    match KEYBOARD_HANDLER.emit(target, action) {
        EventBlock::Block => 0,
        EventBlock::Unblock => unsafe {
            winuser::CallNextHookEx(HHOOK_HANDLER.load(Ordering::SeqCst), code, w_param, l_param)
        },
    }
}

impl HookInstallable<Key, KeyboardAction> for KeyboardEventHandler {
    fn install_hook() -> Result<(), ()> {
        let handler = unsafe {
            winuser::SetWindowsHookExW(WH_KEYBOARD_LL, Some(hook_proc), 0 as HINSTANCE, 0)
        };
        if handler.is_null() {
            return Err(());
        }
        HHOOK_HANDLER.store(handler, Ordering::SeqCst);
        unsafe { winuser::GetMessageW(MaybeUninit::zeroed().assume_init(), 0 as HWND, 0, 0) };
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

impl EmulateKeyboardInput for Key {
    fn press(&self) {
        send_key_input(self, 0);
    }
    fn release(&self) {
        send_key_input(self, KEYEVENTF_KEYUP);
    }

    fn is_pressed(&self) -> bool {
        get_key_state(self) & (1 << 15) != 0
    }
    fn is_toggled(&self) -> bool {
        get_key_state(self) & 1 != 0
    }
}

fn send_key_input(key: &Key, flags: u32) {
    let keybd_input = KEYBDINPUT {
        wVk: <Key as Into<u32>>::into(*key) as u16,
        wScan: 0,
        dwFlags: flags,
        time: 0,
        dwExtraInfo: 0,
    };
    let input = &mut INPUT {
        type_: INPUT_KEYBOARD,
        u: unsafe { mem::transmute_copy(&keybd_input) },
    } as LPINPUT;
    unsafe { winuser::SendInput(1 as UINT, input, mem::size_of::<INPUT>() as c_int) };
}

fn get_key_state(key: &Key) -> i16 {
    unsafe { winuser::GetKeyState(<Key as Into<u32>>::into(*key) as i32) }
}
