use super::{button_state::BUTTON_STATE, convert};
use crate::button::{Button, ButtonAction};

use std::{
    mem::MaybeUninit,
    sync::mpsc::{self, SyncSender},
};

use once_cell::sync::Lazy;
use windows::Win32::UI::{
    Input::KeyboardAndMouse::{SendInput, INPUT},
    WindowsAndMessaging,
};

fn send_input(input: INPUT) {
    unsafe {
        SendInput(
            std::slice::from_ref(&input),
            std::mem::size_of::<INPUT>() as i32,
        );
    }
}

static INPUT_THREAD: Lazy<SyncSender<INPUT>> = Lazy::new(|| {
    let (tx, rx) = mpsc::sync_channel(256);
    std::thread::spawn(move || rx.into_iter().for_each(send_input));
    tx
});

fn invoke_send_input(input: INPUT) {
    INPUT_THREAD.send(input).unwrap();
}

#[inline]
pub(super) fn get_cursor_position() -> (i32, i32) {
    unsafe {
        let mut pos = MaybeUninit::zeroed().assume_init();
        WindowsAndMessaging::GetCursorPos(&mut pos);
        (pos.x, pos.y)
    }
}

pub(super) fn send_button_input(button: Button, action: ButtonAction, recursive: bool) {
    BUTTON_STATE.reflect_input(button, action);
    let input = convert::to_button_input(button, action, recursive);
    invoke_send_input(input);
}

pub(super) fn is_pressed(button: Button) -> bool {
    BUTTON_STATE.is_pressed(button)
}

pub(super) fn move_cursor(x: i32, y: i32, absolute: bool, recursive: bool) {
    let input = convert::to_mouse_cursor_input(x, y, absolute, recursive);
    invoke_send_input(input);
}

pub(super) fn rotate_wheel(speed: i32, recursive: bool) {
    let input = convert::to_mouse_wheel_input(speed, recursive);
    invoke_send_input(input);
}
