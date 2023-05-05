use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::Threading;
use windows::Win32::UI::HiDpi::{
    SetProcessDpiAwarenessContext, DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE,
};
use windows::Win32::UI::WindowsAndMessaging::*;

use std::mem::{self, MaybeUninit};
use std::sync::{mpsc, Mutex};
use std::thread::JoinHandle;

use crate::event::{CursorEvent, Event};
use crate::hook::{EventSender, NativeEventOperation};

use super::button_state::BUTTON_STATE;
use super::convert::{self, MouseEvent, WindowsCursorEvent};
use super::input;

static HOOK_HANDLER: Mutex<Option<HookHandler>> = Mutex::new(None);
static EVENT_SENDER: Mutex<Option<EventSender>> = Mutex::new(None);

pub(crate) fn install(tx: EventSender) {
    let mut hook_handler = HOOK_HANDLER.lock().unwrap();
    if hook_handler.is_some() {
        panic!("Hooks are already installed.");
    }

    *EVENT_SENDER.lock().unwrap() = Some(tx);
    *hook_handler = Some(HookHandler::install());
}

pub(crate) fn uninstall() {
    let mut hook_handler = HOOK_HANDLER.lock().unwrap();
    let Some(hook_handler) = hook_handler.take() else {
        panic!("Hooks are not installed.");
    };

    hook_handler.uninstall();
    *EVENT_SENDER.lock().unwrap() = None;
}

fn call_next_hook(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe { CallNextHookEx(HHOOK(0), code, wparam, lparam) }
}

fn common_hook_proc(event: Event, code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let rx = {
        let event_sender = EVENT_SENDER.lock().unwrap();
        event_sender.as_ref().unwrap().send(event)
    };
    match rx.recv() {
        NativeEventOperation::Block => LRESULT(1),
        NativeEventOperation::Dispatch => call_next_hook(code, wparam, lparam),
    }
}

extern "system" fn keyboard_hook_procedure(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code < 0 {
        return call_next_hook(code, wparam, lparam);
    }

    let input: &KBDLLHOOKSTRUCT = unsafe { mem::transmute(lparam.0) };
    let Some(event) = convert::to_button_event(input) else {
        return call_next_hook(code, wparam, lparam);
    };

    BUTTON_STATE.reflect_input(event.target, event.action);

    common_hook_proc(Event::Button(event), code, wparam, lparam)
}

extern "system" fn mouse_hook_procedure(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code < 0 {
        return call_next_hook(code, wparam, lparam);
    }

    let input: &MSLLHOOKSTRUCT = unsafe { mem::transmute(lparam.0) };
    let Some(event) = convert::to_mouse_event(wparam, input) else {
        return call_next_hook(code, wparam, lparam);
    };

    let event = match event {
        MouseEvent::Button(event) => {
            BUTTON_STATE.reflect_input(event.target, event.action);
            Event::Button(event)
        }
        MouseEvent::Wheel(event) => Event::Wheel(event),
        MouseEvent::Cursor(WindowsCursorEvent { position, injected }) => {
            let prev = input::get_cursor_position();
            let delta = (position.0 - prev.0, position.1 - prev.1);
            Event::Cursor(CursorEvent { delta, injected })
        }
    };

    common_hook_proc(event, code, wparam, lparam)
}

fn set_windows_hook_ex(
    idhook: WINDOWS_HOOK_ID,
    lpfn: unsafe extern "system" fn(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT,
) -> Result<HHOOK, windows::core::Error> {
    unsafe { SetWindowsHookExW(idhook, Some(lpfn), HINSTANCE(0), 0) }
}

#[derive(Debug)]
struct HookHandler {
    keyboard_hook: HHOOK,
    mouse_hook: HHOOK,
    thread_id: u32,
    join_handle: JoinHandle<()>,
}

impl HookHandler {
    fn install() -> Self {
        let (tx, rx) = mpsc::channel();

        let join_handle = std::thread::spawn(move || {
            // Adjust DPI for multiple monitors
            unsafe { SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE) };

            let keyboard_hook =
                set_windows_hook_ex(WH_KEYBOARD_LL, keyboard_hook_procedure).unwrap();
            let mouse_hook = set_windows_hook_ex(WH_MOUSE_LL, mouse_hook_procedure).unwrap();

            let thread_id = unsafe { Threading::GetCurrentThreadId() };
            tx.send((keyboard_hook, mouse_hook, thread_id)).unwrap();

            // Message loop
            unsafe {
                let mut msg = MaybeUninit::zeroed().assume_init();
                while GetMessageW(&mut msg, HWND(0), 0, 0).as_bool() {}
            }
        });

        let (keyboard_hook, mouse_hook, thread_id) = rx.recv().unwrap();
        Self {
            keyboard_hook,
            mouse_hook,
            thread_id,
            join_handle,
        }
    }

    fn uninstall(self) {
        unsafe {
            UnhookWindowsHookEx(self.keyboard_hook);
            UnhookWindowsHookEx(self.mouse_hook);
            PostThreadMessageW(self.thread_id, WM_QUIT, WPARAM(0), LPARAM(0));
        }

        self.join_handle.join().unwrap();
    }
}
