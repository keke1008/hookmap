use super::DW_EXTRA_INFO;
use crate::common::{
    mouse::{EmulateMouseCursor, EmulateMouseWheel, Mouse, MouseCursor, MouseWheel},
    EmulateButtonInput,
};
use crate::windows::mouse::MouseParameter;
use std::{
    mem::{self, MaybeUninit},
    thread,
};
use winapi::{
    ctypes::c_int,
    um::winuser::{
        self, INPUT, MOUSEEVENTF_ABSOLUTE, MOUSEEVENTF_MOVE, MOUSEEVENTF_WHEEL, MOUSEINPUT,
        WHEEL_DELTA,
    },
};

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