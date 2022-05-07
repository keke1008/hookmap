use super::{vkcode, INJECTED_FLAG, INPUT, SHOULD_BE_IGNORED_FLAG};
use crate::button::{Button, ButtonAction};
use crate::event::{
    ButtonEvent, CursorEvent, Event, EventSender, NativeEventOperation, WheelEvent,
};

use once_cell::sync::OnceCell;
use windows::Win32::Foundation::{HINSTANCE, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY;
use windows::Win32::UI::WindowsAndMessaging;

// For many constants.
use windows::Win32::UI::WindowsAndMessaging::*;

static EVENT_SENDER: OnceCell<EventSender> = OnceCell::new();

#[inline]
fn call_next_hook(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    unsafe {
        WindowsAndMessaging::CallNextHookEx(
            // This parameter is ignored.
            HHOOK(0),
            n_code,
            w_param,
            l_param,
        )
    }
}

pub(super) fn create_keyboard_event(hook: KBDLLHOOKSTRUCT) -> Option<ButtonEvent> {
    let action = if hook.flags.0 >> 7 == 0 {
        ButtonAction::Press
    } else {
        ButtonAction::Release
    };
    Some(ButtonEvent {
        target: vkcode::into_button(VIRTUAL_KEY(hook.vkCode as u16))?,
        injected: hook.dwExtraInfo & INJECTED_FLAG != 0,
        action,
    })
}

extern "system" fn keyboard_hook_proc(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if n_code < 0 {
        return call_next_hook(n_code, w_param, l_param);
    }
    let hook = unsafe { *(l_param.0 as *const KBDLLHOOKSTRUCT) };
    if hook.dwExtraInfo & SHOULD_BE_IGNORED_FLAG != 0 {
        return call_next_hook(n_code, w_param, l_param);
    }
    let event = match create_keyboard_event(hook) {
        Some(event) => event,
        None => return call_next_hook(n_code, w_param, l_param),
    };
    let native = EVENT_SENDER
        .get()
        .expect("Hooks are not yet installed.")
        .send(Event::Button(event));
    if event.action == ButtonAction::Release {
        return call_next_hook(n_code, w_param, l_param);
    }
    match native {
        NativeEventOperation::Dispatch => call_next_hook(n_code, w_param, l_param),
        NativeEventOperation::Block => LRESULT(1),
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
    let mouse_button = match w_param.0 as u32 {
        WM_MOUSEWHEEL => return Some(MouseEventTarget::Wheel),
        WM_MOUSEMOVE => return Some(MouseEventTarget::Cursor),
        WM_LBUTTONDOWN | WM_LBUTTONUP => Button::LeftButton,
        WM_RBUTTONDOWN | WM_RBUTTONUP => Button::RightButton,
        WM_MBUTTONDOWN | WM_MBUTTONUP => Button::MiddleButton,
        WM_XBUTTONDOWN | WM_XBUTTONUP if hook_struct.mouseData == XBUTTON1 => Button::SideButton1,
        WM_XBUTTONDOWN | WM_XBUTTONUP if hook_struct.mouseData == XBUTTON2 => Button::SideButton2,
        _ => None?,
    };
    Some(MouseEventTarget::Button(mouse_button))
}

fn into_mouse_button_action(w_param: WPARAM) -> Option<ButtonAction> {
    match w_param.0 as u32 {
        WM_LBUTTONDOWN | WM_RBUTTONDOWN | WM_MBUTTONDOWN | WM_XBUTTONDOWN => {
            Some(ButtonAction::Press)
        }
        WM_LBUTTONUP | WM_RBUTTONUP | WM_MBUTTONUP | WM_XBUTTONUP => Some(ButtonAction::Release),
        _ => None,
    }
}

extern "system" fn mouse_hook_proc(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if n_code < 0 {
        return call_next_hook(n_code, w_param, l_param);
    }
    let hook = unsafe { *(l_param.0 as *const MSLLHOOKSTRUCT) };
    if hook.dwExtraInfo & SHOULD_BE_IGNORED_FLAG != 0 {
        return call_next_hook(n_code, w_param, l_param);
    }
    let target = match into_mouse_event_target(w_param, &hook) {
        Some(target) => target,
        None => return call_next_hook(n_code, w_param, l_param),
    };
    let injected = hook.dwExtraInfo & INJECTED_FLAG != 0;
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
            Event::Button(ButtonEvent {
                target: button,
                injected,
                action,
            })
        }
        MouseEventTarget::Cursor => {
            let prev = INPUT.cursor_position();
            let current = hook.pt;
            let delta = (current.x - prev.0, current.y - prev.1);

            Event::Cursor(CursorEvent { delta, injected })
        }
        MouseEventTarget::Wheel => {
            let delta = (hook.mouseData.0 as i32 >> 16) / WHEEL_DELTA as i32;
            Event::Wheel(WheelEvent { delta, injected })
        }
    };
    match EVENT_SENDER
        .get()
        .expect("Hooks are not yet installed.")
        .send(event)
    {
        NativeEventOperation::Dispatch => call_next_hook(n_code, w_param, l_param),
        NativeEventOperation::Block => LRESULT(1),
    }
}

pub(super) fn install(event_provider: EventSender) {
    EVENT_SENDER
        .set(event_provider)
        .expect("Hooks are already installed.");

    unsafe {
        WindowsAndMessaging::SetWindowsHookExW(
            WH_KEYBOARD_LL,
            Some(keyboard_hook_proc),
            HINSTANCE(0),
            0,
        )
        .expect("Failed to install keyboard hook.");

        WindowsAndMessaging::SetWindowsHookExW(WH_MOUSE_LL, Some(mouse_hook_proc), HINSTANCE(0), 0)
            .expect("Failed to install mouse hook.");
    }
}
