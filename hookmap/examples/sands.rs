use hookmap::{
    button::{EmulateButtonInput, ToButtonWithState},
    *,
};
use std::collections::HashSet;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;

fn emulate_sands<T, U>(hook: &T, space: U, ignore: HashSet<Button>)
where
    T: SelectHandleTarget,
    U: EmulateButtonInput + ToButtonWithState,
{
    let is_alone = Arc::new(AtomicBool::new(true));

    hotkey!(hook => {
        bind [&space] => LShift;
        on_press [&space] => {
            let is_alone = Arc::clone(&is_alone);
            move |_| is_alone.store(true, Ordering::SeqCst)
        };
    });

    {
        let is_alone = Arc::clone(&is_alone);
        let is_shift_alone = hook.cond(Cond::callback(move || is_alone.load(Ordering::SeqCst)));
        hotkey!(is_shift_alone => {
            on_release [&space] => move |_| space.click();
        });
    }

    thread::spawn(move || loop {
        Interruption::unblock()
            .keyboard()
            .iter()
            .filter(|e| e.action == ButtonAction::Press && !ignore.contains(&e.target))
            .for_each(|_| is_alone.store(false, Ordering::SeqCst));
    });
}

fn main() {
    let hook = Hook::new();
    let ignore = [
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

    emulate_sands(&hook, Button::Space, ignore);

    hook.handle_input();
}
