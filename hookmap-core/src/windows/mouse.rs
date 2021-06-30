use crate::{
    common::{
        event::{BlockInput, EventHandlerExt},
        mouse::{EmulateMouseInput, MouseHandler},
    },
    mouse::{MouseAction, MouseInput, MOUSE_EVENT_HANDLER},
};
use once_cell::sync::Lazy;
use std::{
    mem::{self, MaybeUninit},
    sync::atomic::{AtomicPtr, Ordering},
};
use winapi::{
    ctypes::c_int,
    shared::{
        minwindef::{HINSTANCE, HIWORD, LPARAM, LRESULT, WPARAM},
        windef::{HHOOK__, HWND},
    },
    um::winuser::{
        self, INPUT, MOUSEEVENTF_ABSOLUTE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP,
        MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP, MOUSEEVENTF_MOVE, MOUSEEVENTF_RIGHTDOWN,
        MOUSEEVENTF_RIGHTUP, MOUSEEVENTF_WHEEL, MOUSEEVENTF_XDOWN, MOUSEEVENTF_XUP, MOUSEINPUT,
        MSLLHOOKSTRUCT, VK_LBUTTON, VK_MBUTTON, VK_RBUTTON, VK_XBUTTON1, VK_XBUTTON2, WHEEL_DELTA,
        WH_MOUSE_LL, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MBUTTONDOWN, WM_MBUTTONUP, WM_MOUSEMOVE,
        WM_MOUSEWHEEL, WM_RBUTTONDOWN, WM_RBUTTONUP, WM_XBUTTONDOWN, WM_XBUTTONUP, XBUTTON1,
        XBUTTON2,
    },
};

static HHOOK_HANDLER: Lazy<AtomicPtr<HHOOK__>> = Lazy::new(AtomicPtr::default);

extern "system" fn hook_proc(code: c_int, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    let event_info = unsafe { *(l_param as *const MSLLHOOKSTRUCT) };
    let input: MouseInput = (w_param, event_info).into();
    let action: MouseAction = (w_param, event_info).into();
    match MOUSE_EVENT_HANDLER.emit(input, action) {
        BlockInput::Block => 1,
        BlockInput::Unblock => unsafe {
            winuser::CallNextHookEx(HHOOK_HANDLER.load(Ordering::SeqCst), code, w_param, l_param)
        },
    }
}

impl EventHandlerExt<MouseInput, MouseAction> for MouseHandler {
    fn install_hook() -> Result<(), ()> {
        let handler =
            unsafe { winuser::SetWindowsHookExW(WH_MOUSE_LL, Some(hook_proc), 0 as HINSTANCE, 0) };
        if handler.is_null() {
            return Err(());
        }
        HHOOK_HANDLER.store(handler, Ordering::SeqCst);
        unsafe { winuser::GetMessageW(MaybeUninit::uninit().assume_init(), 0 as HWND, 0, 0) };
        Ok(())
    }

    fn uninstall_hook() -> Result<(), ()> {
        let handler = HHOOK_HANDLER.swap(std::ptr::null_mut(), Ordering::SeqCst);
        if handler.is_null() {
            return Err(());
        }
        let result =
            unsafe { winuser::UnhookWindowsHookEx(HHOOK_HANDLER.swap(handler, Ordering::SeqCst)) };
        if result == 0 {
            Err(())
        } else {
            Ok(())
        }
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
        let (mouse_data, dw_flags) = match self {
            MouseInput::LButton => (0, MOUSEEVENTF_LEFTDOWN),
            MouseInput::RButton => (0, MOUSEEVENTF_RIGHTDOWN),
            MouseInput::MButton => (0, MOUSEEVENTF_MIDDLEDOWN),
            MouseInput::SideButton1 => (XBUTTON1, MOUSEEVENTF_XDOWN),
            MouseInput::SideButton2 => (XBUTTON2, MOUSEEVENTF_XDOWN),
            _ => panic!("{:?} cannnot be pressed. It's not a button.", *self),
        };
        send_mouse_input(0, 0, mouse_data as u32, dw_flags as u32);
    }

    fn release(&self) {
        let (mouse_data, dw_flags) = match self {
            MouseInput::LButton => (0, MOUSEEVENTF_LEFTUP),
            MouseInput::RButton => (0, MOUSEEVENTF_RIGHTUP),
            MouseInput::MButton => (0, MOUSEEVENTF_MIDDLEUP),
            MouseInput::SideButton1 => (XBUTTON1, MOUSEEVENTF_XUP),
            MouseInput::SideButton2 => (XBUTTON2, MOUSEEVENTF_XUP),
            _ => panic!("{:?} cannnot be released. It's not a button.", *self),
        };
        send_mouse_input(0, 0, mouse_data as u32, dw_flags as u32);
    }

    fn is_pressed(&self) -> bool {
        let vk_code = match self {
            MouseInput::LButton => VK_LBUTTON,
            MouseInput::RButton => VK_RBUTTON,
            MouseInput::MButton => VK_MBUTTON,
            MouseInput::SideButton1 => VK_XBUTTON1,
            MouseInput::SideButton2 => VK_XBUTTON2,
            _ => panic!("Cannot check if {:?} is pressed. It's not a button.", *self),
        };
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
            let mut pos = MaybeUninit::uninit().assume_init();
            winuser::GetCursorPos(&mut pos);
            (pos.x, pos.y)
        }
    }
}

impl From<(WPARAM, MSLLHOOKSTRUCT)> for MouseInput {
    fn from((w_param, mouse_struct): (WPARAM, MSLLHOOKSTRUCT)) -> Self {
        match w_param as u32 {
            WM_MOUSEWHEEL => MouseInput::Wheel,
            WM_MOUSEMOVE => MouseInput::Move,
            WM_LBUTTONDOWN | WM_LBUTTONUP => MouseInput::LButton,
            WM_RBUTTONDOWN | WM_RBUTTONUP => MouseInput::RButton,
            WM_MBUTTONDOWN | WM_MBUTTONUP => MouseInput::MButton,
            WM_XBUTTONDOWN | WM_XBUTTONUP if HIWORD(mouse_struct.mouseData) == XBUTTON1 => {
                MouseInput::SideButton1
            }
            WM_XBUTTONDOWN | WM_XBUTTONUP if HIWORD(mouse_struct.mouseData) == XBUTTON2 => {
                MouseInput::SideButton2
            }
            _ => {
                println!("{}, {}", mouse_struct.mouseData, XBUTTON1);
                unreachable!();
            }
        }
    }
}

impl From<(WPARAM, MSLLHOOKSTRUCT)> for MouseAction {
    fn from((w_param, mouse_struct): (WPARAM, MSLLHOOKSTRUCT)) -> Self {
        match w_param as u32 {
            WM_MOUSEMOVE => MouseAction::Move(mouse_struct.pt.x, mouse_struct.pt.y),
            WM_MOUSEWHEEL => {
                let speed = winuser::GET_WHEEL_DELTA_WPARAM(mouse_struct.mouseData as usize);
                MouseAction::Wheel(speed as i32)
            }
            WM_LBUTTONDOWN | WM_RBUTTONDOWN | WM_MBUTTONDOWN | WM_XBUTTONDOWN => MouseAction::Press,
            WM_LBUTTONUP | WM_RBUTTONUP | WM_MBUTTONUP | WM_XBUTTONUP => MouseAction::Release,
            _ => unreachable!(),
        }
    }
}
