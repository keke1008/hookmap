use super::{call_next_hook, set_button_state, DW_EXTRA_INFO};
use crate::common::{
    event::EventBlock,
    handler::{InputHandler, INPUT_HANDLER},
    mouse::{EmulateMouseInput, InstallMouseHook, MouseEvent, MouseInput},
};
use once_cell::sync::Lazy;
use std::{
    mem::{self, MaybeUninit},
    sync::atomic::{AtomicPtr, Ordering},
    thread,
};
use winapi::{
    ctypes::c_int,
    shared::{
        minwindef::{HINSTANCE, LPARAM, LRESULT, WPARAM},
        windef::HHOOK__,
    },
    um::winuser::{
        self, INPUT, MOUSEEVENTF_ABSOLUTE, MOUSEEVENTF_MOVE, MOUSEEVENTF_WHEEL, MOUSEINPUT,
        MSLLHOOKSTRUCT, WHEEL_DELTA, WH_MOUSE_LL,
    },
};

use self::conversion::MouseEventInfo;
mod conversion;

static HHOOK_HANDLER: Lazy<AtomicPtr<HHOOK__>> = Lazy::new(AtomicPtr::default);

extern "system" fn hook_proc(code: c_int, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    let mouse_struct = unsafe { *(l_param as *const MSLLHOOKSTRUCT) };
    if mouse_struct.dwExtraInfo & DW_EXTRA_INFO != 0 {
        return call_next_hook(code, w_param, l_param);
    }
    let target = MouseEventInfo::new(w_param, mouse_struct).into_input();
    let action = MouseEventInfo::new(w_param, mouse_struct).into_action();

    if let (Some(target), Some(action)) = (target, action) {
        let event = MouseEvent::new(target, action);
        if INPUT_HANDLER.mouse.lock().unwrap().emit(event) == EventBlock::Block {
            if let Some(vk_code) = target.into_vk_code() {
                set_button_state(vk_code, action);
            }
            return 1;
        }
    }
    call_next_hook(code, w_param, l_param)
}

impl InstallMouseHook for InputHandler {
    fn install() {
        let handler =
            unsafe { winuser::SetWindowsHookExW(WH_MOUSE_LL, Some(hook_proc), 0 as HINSTANCE, 0) };
        HHOOK_HANDLER.store(handler, Ordering::SeqCst);
    }
}

fn send_mouse_input(dx: i32, dy: i32, mouse_data: u32, dw_flags: u32) {
    let mouse_input = MOUSEINPUT {
        dx,
        dy,
        mouseData: mouse_data,
        dwFlags: dw_flags,
        dwExtraInfo: DW_EXTRA_INFO,
        time: 0,
    };
    let mut input = INPUT {
        type_: 0,
        u: unsafe { mem::transmute(mouse_input) },
    };

    thread::spawn(move || unsafe {
        winuser::SendInput(1, &mut input, mem::size_of::<INPUT>() as c_int);
    });
}

impl EmulateMouseInput for MouseInput {
    fn press(&self) {
        if let Some(parameter) = self.into_press() {
            send_mouse_input(0, 0, parameter.mouse_data as u32, parameter.dw_flags as u32);
        }
    }

    fn release(&self) {
        if let Some(parameter) = self.into_release() {
            send_mouse_input(0, 0, parameter.mouse_data as u32, parameter.dw_flags as u32);
        }
    }

    fn is_pressed(&self) -> bool {
        match self.into_vk_code() {
            Some(vk_code) => unsafe { winuser::GetKeyState(vk_code as i32) & (1 << 15) != 0 },
            None => false,
        }
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
