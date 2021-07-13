use crate::{
    common::{
        event::EventBlock,
        handler::HookInstallable,
        mouse::{EmulateMouseInput, MouseHook},
    },
    mouse::{MouseAction, MouseInput, MOUSE_HOOK},
};
use once_cell::sync::Lazy;
use std::{
    mem::{self, MaybeUninit},
    sync::atomic::{AtomicPtr, Ordering},
};
use winapi::{
    ctypes::c_int,
    shared::{
        minwindef::{HINSTANCE, LPARAM, LRESULT, WPARAM},
        windef::{HHOOK__, HWND},
    },
    um::winuser::{
        self, INPUT, MOUSEEVENTF_ABSOLUTE, MOUSEEVENTF_MOVE, MOUSEEVENTF_WHEEL, MOUSEINPUT,
        MSLLHOOKSTRUCT, WHEEL_DELTA, WH_MOUSE_LL,
    },
};

static HHOOK_HANDLER: Lazy<AtomicPtr<HHOOK__>> = Lazy::new(AtomicPtr::default);

extern "system" fn hook_proc(code: c_int, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    let event_info = unsafe { *(l_param as *const MSLLHOOKSTRUCT) };
    let input: MouseInput = (w_param, event_info).into();
    let action: MouseAction = (w_param, event_info).into();
    match MOUSE_HOOK.emit(input, action) {
        EventBlock::Block => 1,
        EventBlock::Unblock => unsafe {
            winuser::CallNextHookEx(HHOOK_HANDLER.load(Ordering::SeqCst), code, w_param, l_param)
        },
    }
}

impl HookInstallable<MouseInput, MouseAction> for MouseHook {
    fn install_hook() -> Result<(), ()> {
        let handler =
            unsafe { winuser::SetWindowsHookExW(WH_MOUSE_LL, Some(hook_proc), 0 as HINSTANCE, 0) };
        if handler.is_null() {
            return Err(());
        }
        HHOOK_HANDLER.store(handler, Ordering::SeqCst);
        unsafe { winuser::GetMessageW(MaybeUninit::zeroed().assume_init(), 0 as HWND, 0, 0) };
        Ok(())
    }
}

fn send_mouse_input(dx: i32, dy: i32, mouse_data: u32, dw_flags: u32) {
    let mouse_input = MOUSEINPUT {
        dx,
        dy,
        mouseData: mouse_data,
        dwFlags: dw_flags,
        dwExtraInfo: 0,
        time: 0,
    };
    let mut input = INPUT {
        type_: 0,
        u: unsafe { mem::transmute(mouse_input) },
    };
    unsafe { winuser::SendInput(1, &mut input, mem::size_of::<INPUT>() as c_int) };
}

impl EmulateMouseInput for MouseInput {
    fn press(&self) {
        let (mouse_data, dw_flags) = self.into_press_parameter();
        send_mouse_input(0, 0, mouse_data as u32, dw_flags as u32);
    }

    fn release(&self) {
        let (mouse_data, dw_flags) = self.into_release_parameter();
        send_mouse_input(0, 0, mouse_data as u32, dw_flags as u32);
    }

    fn is_pressed(&self) -> bool {
        let vk_code = self.into_vk_code();
        unsafe { winuser::GetKeyState(vk_code) & (1 << 15) != 0 }
    }

    fn move_rel(dx: i32, dy: i32) {
        send_mouse_input(dx, dy, 0, MOUSEEVENTF_MOVE);
    }

    fn move_abs(x: i32, y: i32) {
        send_mouse_input(x, y, 0, MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE);
    }

    fn rotate_wheel(speed: u32) {
        send_mouse_input(0, 0, speed * WHEEL_DELTA as u32, MOUSEEVENTF_WHEEL);
    }

    fn get_cursor_pos() -> (i32, i32) {
        unsafe {
            let mut pos = MaybeUninit::zeroed().assume_init();
            winuser::GetCursorPos(&mut pos);
            (pos.x, pos.y)
        }
    }
}
