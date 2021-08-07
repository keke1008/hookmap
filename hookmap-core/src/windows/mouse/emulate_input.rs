use super::DW_EXTRA_INFO;
use crate::common::{
    mouse::{EmulateMouseCursor, EmulateMouseWheel, Mouse},
    EmulateButtonInput,
};
use crate::windows::mouse::MouseParameter;
use once_cell::sync::Lazy;
use std::{
    collections::HashMap,
    mem::{self, MaybeUninit},
    sync::Mutex,
};
use winapi::{
    ctypes::c_int,
    um::winuser::{self, INPUT, MOUSEEVENTF_WHEEL, MOUSEINPUT, WHEEL_DELTA},
};

static MOUSE_STATE: Lazy<Mutex<HashMap<Mouse, bool>>> = Lazy::new(Default::default);

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

    unsafe {
        winuser::SendInput(1, &mut input, mem::size_of::<INPUT>() as c_int);
    }
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
}

impl Mouse {
    pub(super) fn assume_pressed(&self) {
        MOUSE_STATE.lock().unwrap().insert(*self, true);
    }

    pub(super) fn assume_released(&self) {
        MOUSE_STATE.lock().unwrap().insert(*self, false);
    }
}

impl EmulateMouseWheel for Mouse {
    fn rotate(speed: i32) {
        let speed = speed * WHEEL_DELTA as i32;
        send_mouse_input(0, 0, speed as u32, MOUSEEVENTF_WHEEL);
    }
}

impl EmulateMouseCursor for Mouse {
    fn move_rel(dx: i32, dy: i32) {
        let (x, y) = Self::get_pos();
        unsafe { winuser::SetCursorPos(x + dx, y + dy) };
    }

    fn move_abs(x: i32, y: i32) {
        unsafe { winuser::SetCursorPos(x, y) };
    }

    fn get_pos() -> (i32, i32) {
        unsafe {
            let mut pos = MaybeUninit::zeroed().assume_init();
            winuser::GetPhysicalCursorPos(&mut pos);
            (pos.x, pos.y)
        }
    }
}
