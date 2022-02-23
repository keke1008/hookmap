use hookmap::prelude::*;
use std::collections::HashSet;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

fn emulate_sands<T: RegisterHotkey>(hotkey: &T, space: Button, ingored: HashSet<Button>) {
    let is_other_key_pressed = Arc::new(AtomicBool::default());

    let hotkey = hotkey.block_input_event();
    let on_press = {
        let is_other_key_pressed = Arc::clone(&is_other_key_pressed);
        move |_| {
            is_other_key_pressed.store(false, Ordering::SeqCst);
            seq!(LShift down).send();
        }
    };
    hotkey.on_press(space, on_press);

    let on_release = {
        let is_other_key_pressed = Arc::clone(&is_other_key_pressed);
        move |_| {
            seq!(LShift up);
            if !is_other_key_pressed.load(Ordering::SeqCst) {
                seq!([space]).send();
            }
        }
    };
    hotkey.on_release(space, on_release);

    let filter = Filter::new()
        .action(ButtonAction::Press)
        .callback(move |e| !ingored.contains(&e.target));
    Interceptor::unblock(filter).then_iter(move |iter| {
        iter.for_each(|_| is_other_key_pressed.store(true, Ordering::SeqCst))
    });
}

fn main() {
    let hotkey = Hotkey::new();
    let ignored = [
        Button::Space,
        Button::LShift,
        Button::RShift,
        Button::LCtrl,
        Button::RCtrl,
        Button::LAlt,
        Button::RAlt,
        Button::LMeta,
        Button::RMeta,
    ]
    .iter()
    .copied()
    .collect();

    emulate_sands(&hotkey, Button::Space, ignored);

    hotkey.install();
}
