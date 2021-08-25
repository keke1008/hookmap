use super::{call_next_hook, IGNORED_DW_EXTRA_INFO};
use crate::common::{
    button::{Button, ButtonAction},
    event::{ButtonEvent, EventBlock},
    mouse::{EmulateMouseCursor, Mouse},
    INPUT_HANDLER,
};
use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicPtr, Ordering};
use winapi::{
    ctypes::c_int,
    shared::{
        minwindef::{self, HINSTANCE, LPARAM, LRESULT, WPARAM},
        windef::HHOOK__,
    },
    um::winuser,
};
// For many constants.
use winapi::um::winuser::*;

static HHOOK_HANDLER: Lazy<AtomicPtr<HHOOK__>> = Lazy::new(AtomicPtr::default);

fn to_hook_struct(l_param: LPARAM) -> MSLLHOOKSTRUCT {
    let ptr = l_param as *const MSLLHOOKSTRUCT;
    unsafe { *ptr }
}

enum MouseEventTarget {
    Button(Button),
    Cursor,
    Wheel,
}

fn to_event_target(w_param: WPARAM, hook_struct: &MSLLHOOKSTRUCT) -> Option<MouseEventTarget> {
    let mouse_data = minwindef::HIWORD(hook_struct.mouseData);
    let mouse_button = match w_param as u32 {
        WM_MOUSEWHEEL => return Some(MouseEventTarget::Wheel),
        WM_MOUSEMOVE => return Some(MouseEventTarget::Cursor),
        WM_LBUTTONDOWN | WM_LBUTTONUP => Button::LeftButton,
        WM_RBUTTONDOWN | WM_RBUTTONUP => Button::RightButton,
        WM_MBUTTONDOWN | WM_MBUTTONUP => Button::MiddleButton,
        WM_XBUTTONDOWN | WM_XBUTTONUP if mouse_data == XBUTTON1 => Button::SideButton1,
        WM_XBUTTONDOWN | WM_XBUTTONUP if mouse_data == XBUTTON2 => Button::SideButton2,
        _ => None?,
    };
    Some(MouseEventTarget::Button(mouse_button))
}

fn to_wheel_delta(w_param: WPARAM) -> i32 {
    let delta = winuser::GET_WHEEL_DELTA_WPARAM(w_param as usize) as i32;
    delta / (WHEEL_DELTA as i32)
}

fn to_button_action(w_param: WPARAM) -> Option<ButtonAction> {
    match w_param as u32 {
        WM_LBUTTONDOWN | WM_RBUTTONDOWN | WM_MBUTTONDOWN | WM_XBUTTONDOWN => {
            Some(ButtonAction::Press)
        }
        WM_LBUTTONUP | WM_RBUTTONUP | WM_MBUTTONUP | WM_XBUTTONUP => Some(ButtonAction::Release),
        _ => None,
    }
}

extern "system" fn hook_proc(code: c_int, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    let hook_struct = to_hook_struct(l_param);
    if hook_struct.dwExtraInfo & IGNORED_DW_EXTRA_INFO != 0 {
        return call_next_hook(code, w_param, l_param);
    }
    let event_target = to_event_target(w_param, &hook_struct);
    if event_target.is_none() {
        return call_next_hook(code, w_param, l_param);
    }
    let event_block = match event_target.unwrap() {
        MouseEventTarget::Button(target) => {
            let button_action = to_button_action(w_param).unwrap();
            let event = ButtonEvent::new(target, button_action);
            match button_action {
                ButtonAction::Press => target.assume_pressed(),
                ButtonAction::Release => target.assume_released(),
            }
            INPUT_HANDLER.button.emit(event)
        }
        MouseEventTarget::Wheel => INPUT_HANDLER.mouse_wheel.emit(to_wheel_delta(w_param)),
        MouseEventTarget::Cursor => INPUT_HANDLER.mouse_cursor.emit(Mouse::get_pos()),
    };
    match event_block {
        EventBlock::Unblock => call_next_hook(code, w_param, l_param),
        EventBlock::Block => 1,
    }
}

pub(in crate::windows) fn install_hook() {
    let handler =
        unsafe { winuser::SetWindowsHookExW(WH_MOUSE_LL, Some(hook_proc), 0 as HINSTANCE, 0) };
    HHOOK_HANDLER.store(handler, Ordering::SeqCst);
}
