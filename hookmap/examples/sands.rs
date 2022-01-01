use hookmap::{
    button::{Button, ButtonAction},
    hotkey,
    hotkey::{Hotkey, RegisterHotkey},
    interceptor::{Filter, Interceptor},
    seq,
};
use std::collections::HashSet;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

fn emulate_sands(hotkey: &Hotkey, space: Button, ingored: HashSet<Button>) {
    let is_alone = Arc::new(AtomicBool::new(true));

    let on_press_space = {
        let is_alone = Arc::clone(&is_alone);
        move |_| {
            is_alone.store(true, Ordering::SeqCst);
            seq!(LShift down);
        }
    };

    let on_release_space = {
        let is_alone = Arc::clone(&is_alone);
        move |_| {
            seq!(LShift up);
            if is_alone.load(Ordering::SeqCst) {
                seq!([space]);
            }
        }
    };

    hotkey!(hotkey => {
        block {
            on_press [space] => on_press_space;
            on_release [space] => on_release_space;
        }
    });

    let filter = Filter::new()
        .action(ButtonAction::Press)
        .callback(move |e| !ingored.contains(&e.target));
    Interceptor::unblock(filter)
        .then_iter(move |iter| iter.for_each(|_| is_alone.store(false, Ordering::SeqCst)));
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
