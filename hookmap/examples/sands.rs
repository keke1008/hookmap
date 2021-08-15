use hookmap::*;
use std::collections::HashSet;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;

fn emulate_sands<T, B>(hook: &T, space: B, ignore: HashSet<Button>)
where
    T: SelectHandleTarget,
    B: ButtonInput + ButtonState + Clone + Send + Sync,
{
    hook.bind(space.clone()).like(Button::Shift);

    let is_alone = Arc::new(AtomicBool::new(true));
    {
        let is_alone = Arc::clone(&is_alone);
        hook.bind(space.clone())
            .on_press(move |_| is_alone.store(true, Ordering::SeqCst));
    }
    {
        let is_alone = Arc::clone(&is_alone);
        hook.cond(Cond::callback(move || is_alone.load(Ordering::SeqCst)))
            .bind(space.clone())
            .on_release(move |_| space.click());
    }

    thread::spawn(move || loop {
        let event = interruption::keyboard_event();
        if event.action == ButtonAction::Press && !ignore.contains(&event.target) {
            is_alone.store(false, Ordering::SeqCst);
        }
    });
}

fn main() {
    let hook = Hook::new();
    let ignore = [Button::Space, Button::Shift, Button::Ctrl, Button::Alt]
        .iter()
        .map(|&b| b)
        .collect();

    emulate_sands(&hook, Button::Space, ignore);

    hook.handle_input();
}
