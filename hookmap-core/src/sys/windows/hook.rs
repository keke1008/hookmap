use super::{vkcode, IGNORED_DW_EXTRA_INFO};
use crate::button::{Button, ButtonAction};
use crate::event::{ButtonEvent, Event, EventProvider, NativeEventOperation};

use std::mem::MaybeUninit;

use once_cell::sync::OnceCell;
use winapi::{
    ctypes::c_int,
    shared::minwindef::{self, HINSTANCE, LPARAM, LRESULT, WPARAM},
    um::winuser,
};
// For many constants.
use winapi::um::winuser::*;

static EVENT_PROVIDER: OnceCell<EventProvider> = OnceCell::new();

fn call_next_hook(n_code: c_int, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    unsafe {
        winuser::CallNextHookEx(
            // This parameter is ignored.
            MaybeUninit::zeroed().assume_init(),
            n_code,
            w_param,
            l_param,
        )
    }
}

pub(super) fn create_keyboard_event(hook: KBDLLHOOKSTRUCT) -> Option<ButtonEvent> {
    let target = vkcode::into_button(hook.vkCode)?;
    let action = if hook.flags >> 7 == 0 {
        ButtonAction::Press
    } else {
        ButtonAction::Release
    };
    Some(ButtonEvent::new(target, action))
}

extern "system" fn keyboard_hook_proc(n_code: c_int, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if n_code < 0 {
        return call_next_hook(n_code, w_param, l_param);
    }
    let hook = unsafe { *(l_param as *const KBDLLHOOKSTRUCT) };
    if hook.dwExtraInfo & IGNORED_DW_EXTRA_INFO != 0 {
        return call_next_hook(n_code, w_param, l_param);
    }
    let event = match create_keyboard_event(hook) {
        Some(event) => event,
        None => return call_next_hook(n_code, w_param, l_param),
    };
    let native = EVENT_PROVIDER
        .get()
        .expect("Hooks are not yet installed.")
        .send(Event::Button(event));
    if event.action == ButtonAction::Release {
        return call_next_hook(n_code, w_param, l_param);
    }
    match native {
        NativeEventOperation::Dispatch => call_next_hook(n_code, w_param, l_param),
        NativeEventOperation::Block => 1,
    }
}

enum MouseEventTarget {
    Button(Button),
    Cursor,
    Wheel,
}

fn into_mouse_event_target(
    w_param: WPARAM,
    hook_struct: &MSLLHOOKSTRUCT,
) -> Option<MouseEventTarget> {
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

fn into_mouse_button_action(w_param: WPARAM) -> Option<ButtonAction> {
    match w_param as u32 {
        WM_LBUTTONDOWN | WM_RBUTTONDOWN | WM_MBUTTONDOWN | WM_XBUTTONDOWN => {
            Some(ButtonAction::Press)
        }
        WM_LBUTTONUP | WM_RBUTTONUP | WM_MBUTTONUP | WM_XBUTTONUP => Some(ButtonAction::Release),
        _ => None,
    }
}

extern "system" fn mouse_hook_proc(n_code: c_int, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if n_code < 0 {
        return call_next_hook(n_code, w_param, l_param);
    }
    let hook = unsafe { *(l_param as *const MSLLHOOKSTRUCT) };
    if hook.dwExtraInfo & IGNORED_DW_EXTRA_INFO != 0 {
        return call_next_hook(n_code, w_param, l_param);
    }
    let target = match into_mouse_event_target(w_param, &hook) {
        Some(target) => target,
        None => return call_next_hook(n_code, w_param, l_param),
    };
    let event = match target {
        MouseEventTarget::Button(button) => {
            let action = match into_mouse_button_action(w_param) {
                Some(action) => action,
                None => return call_next_hook(n_code, w_param, l_param),
            };
            match action {
                ButtonAction::Press => button.assume_pressed(),
                ButtonAction::Release => button.assume_released(),
            };
            Event::Button(ButtonEvent::new(button, action))
        }
        MouseEventTarget::Cursor => Event::MouseCursor((hook.pt.x, hook.pt.y)),
        MouseEventTarget::Wheel => {
            let delta = winuser::GET_WHEEL_DELTA_WPARAM(hook.mouseData as usize);
            Event::MouseWheel(delta as i32 / WHEEL_DELTA as i32)
        }
    };
    match EVENT_PROVIDER
        .get()
        .expect("Hooks are not yet installed.")
        .send(event)
    {
        NativeEventOperation::Dispatch => call_next_hook(n_code, w_param, l_param),
        NativeEventOperation::Block => 1,
    }
}

pub(super) fn install(event_provider: EventProvider) {
    EVENT_PROVIDER
        .set(event_provider)
        .expect("Hooks are already installed.");
    unsafe {
        winuser::SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_hook_proc), 0 as HINSTANCE, 0);
        winuser::SetWindowsHookExW(WH_MOUSE_LL, Some(mouse_hook_proc), 0 as HINSTANCE, 0);
    }
}
