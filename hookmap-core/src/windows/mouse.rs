use super::{call_next_hook, set_button_state, DW_EXTRA_INFO};
use crate::common::{
    event::EventBlock,
    handler::{InputHandler, INPUT_HANDLER},
    mouse::{
        EmulateMouseCursor, EmulateMouseWheel, InstallMouseHook, Mouse, MouseCursor, MouseEvent,
        MouseWheel,
    },
    EmulateButtonInput,
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

mod conversion;
use conversion::{MouseEventInfo, MouseParameter};

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

impl EmulateButtonInput for Mouse {
    fn press(&self) {
        let MouseParameter {
            mouse_data,
            dw_flags,
        } = self.into_press();
        send_mouse_input(0, 0, mouse_data as u32, dw_flags as u32);
    }

    fn release(&self) {
        let MouseParameter {
            mouse_data,
            dw_flags,
        } = self.into_release();
        send_mouse_input(0, 0, mouse_data as u32, dw_flags as u32);
    }

    fn is_pressed(&self) -> bool {
        let vk_code = self.into_vk_code();
        unsafe { winuser::GetKeyState(vk_code as i32) & (1 << 15) != 0 }
    }

    fn is_toggled(&self) -> bool {
        let vk_code = self.into_vk_code();
        unsafe { winuser::GetKeyState(vk_code as i32) & 1 != 0 }
    }
}

impl EmulateMouseWheel for MouseWheel {
    fn rotate(speed: u32) {
        send_mouse_input(0, 0, speed * WHEEL_DELTA as u32, MOUSEEVENTF_WHEEL);
    }
}

impl EmulateMouseCursor for MouseCursor {
    fn move_rel(dx: i32, dy: i32) {
        send_mouse_input(dx, dy, 0, MOUSEEVENTF_MOVE);
    }

    fn move_abs(x: i32, y: i32) {
        send_mouse_input(x, y, 0, MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE);
    }

    fn get_pos() -> (i32, i32) {
        unsafe {
            let mut pos = MaybeUninit::zeroed().assume_init();
            winuser::GetCursorPos(&mut pos);
            (pos.x, pos.y)
        }
    }
}
