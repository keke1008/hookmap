use super::{vkcode, HOOK, INJECTED_FLAG, INPUT, SHOULD_BE_IGNORED_FLAG};
use crate::button::{Button, ButtonAction};
use crate::event::{
    ButtonEvent, CursorEvent, Event, EventSender, NativeEventOperation, WheelEvent,
};

use std::mem::MaybeUninit;
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Mutex};
use std::thread::{self, JoinHandle};

use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::Threading;
use windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY;
use windows::Win32::UI::WindowsAndMessaging;

// For many constants.
use windows::Win32::UI::WindowsAndMessaging::*;

#[derive(Debug)]
struct Inner {
    keyboard_hook_handler: HHOOK,
    mouse_hook_handler: HHOOK,
    event_sender: EventSender,
    join_handle: JoinHandle<()>,
    thread_id: u32,
}

impl Inner {
    fn spawn_thread(tx: Sender<(HHOOK, HHOOK, u32)>) -> JoinHandle<()> {
        thread::spawn(move || unsafe {
            let keyboard_hook_handler = WindowsAndMessaging::SetWindowsHookExW(
                WH_KEYBOARD_LL,
                Some(keyboard_hook_proc),
                HINSTANCE(0),
                0,
            )
            .expect("Failed to install keyboard hook.");

            let mouse_hook_handler = WindowsAndMessaging::SetWindowsHookExW(
                WH_MOUSE_LL,
                Some(mouse_hook_proc),
                HINSTANCE(0),
                0,
            )
            .expect("Failed to install mouse hook.");

            let thread_id = Threading::GetCurrentThreadId();

            tx.send((keyboard_hook_handler, mouse_hook_handler, thread_id))
                .unwrap();

            let mut msg = MaybeUninit::zeroed().assume_init();
            WindowsAndMessaging::GetMessageW(&mut msg, HWND(0), 0, 0);
            dbg!(msg);
        })
    }

    fn new(event_sender: EventSender) -> Self {
        let (tx, rx) = mpsc::channel();

        let join_handle = Self::spawn_thread(tx);
        let (keyboard_hook_handler, mouse_hook_handler, thread_id) = rx.recv().unwrap();

        Inner {
            keyboard_hook_handler,
            mouse_hook_handler,
            event_sender,
            join_handle,
            thread_id,
        }
    }
}

#[derive(Debug, Default)]
pub(super) struct Hook {
    inner: Mutex<Option<Inner>>,
}

impl Hook {
    pub(super) fn new() -> Self {
        Self::default()
    }

    pub(super) fn install(&self, event_sender: EventSender) {
        let mut hook = self.inner.lock().unwrap();
        assert!(hook.is_none(), "Hooks are already installed.");

        *hook = Some(Inner::new(event_sender));
    }

    fn send_event(&self, event: Event) -> NativeEventOperation {
        self.inner
            .lock()
            .unwrap()
            .as_ref()
            .expect("Hooks are not installed.")
            .event_sender
            .send(event)
    }
}

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

pub(super) fn create_keyboard_event(hook: &KBDLLHOOKSTRUCT) -> Option<ButtonEvent> {
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

fn hook_proc_inner(event: Event, hook_handler: &Hook) -> NativeEventOperation {
    if let Event::Button(ButtonEvent { target, action, .. }) = event {
        match action {
            ButtonAction::Press => target.assume_pressed(),
            ButtonAction::Release => target.assume_released(),
        }
    }
    hook_handler.send_event(event)
}

extern "system" fn keyboard_hook_proc(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if n_code < 0 {
        return call_next_hook(n_code, w_param, l_param);
    }
    let hook = unsafe { *(l_param.0 as *const KBDLLHOOKSTRUCT) };
    let event = match create_keyboard_event(&hook) {
        None => return call_next_hook(n_code, w_param, l_param),
        Some(event) => event,
    };

    let native_operation = hook_proc_inner(Event::Button(event), &HOOK);
    if event.action == ButtonAction::Release {
        return call_next_hook(n_code, w_param, l_param);
    }
    match native_operation {
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
        _ => return None,
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

fn create_mouse_event(w_param: WPARAM, hook: MSLLHOOKSTRUCT) -> Option<Event> {
    if hook.dwExtraInfo & SHOULD_BE_IGNORED_FLAG != 0 {
        return None;
    }
    let injected = hook.dwExtraInfo & INJECTED_FLAG != 0;
    let event = match into_mouse_event_target(w_param, &hook)? {
        MouseEventTarget::Wheel => {
            let delta = (hook.mouseData.0 as i32 >> 16) / WHEEL_DELTA as i32;
            Event::Wheel(WheelEvent { delta, injected })
        }
        MouseEventTarget::Cursor => {
            let prev = INPUT.cursor_position();
            let current = hook.pt;
            let delta = (current.x - prev.0, current.y - prev.1);
            Event::Cursor(CursorEvent { delta, injected })
        }
        MouseEventTarget::Button(button) => {
            let action = into_mouse_button_action(w_param)?;
            Event::Button(ButtonEvent {
                target: button,
                action,
                injected,
            })
        }
    };
    Some(event)
}

extern "system" fn mouse_hook_proc(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if n_code < 0 {
        return call_next_hook(n_code, w_param, l_param);
    }
    let hook = unsafe { *(l_param.0 as *const MSLLHOOKSTRUCT) };
    let event = match create_mouse_event(w_param, hook) {
        None => return call_next_hook(n_code, w_param, l_param),
        Some(event) => event,
    };
    match hook_proc_inner(event, &HOOK) {
        NativeEventOperation::Dispatch => call_next_hook(n_code, w_param, l_param),
        NativeEventOperation::Block => LRESULT(1),
    }
}
