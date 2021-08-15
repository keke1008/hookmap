use hookmap::{button::EmulateButtonState, *};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

fn emulate_alt_tab<T, B>(hook: &T, alt: B, tab: B)
where
    T: SelectHandleTarget,
    B: EmulateButtonState + Clone,
{
    let is_working = Arc::new(AtomicBool::new(false));

    {
        hook.bind(alt.clone()).disable();
        let is_working = Arc::clone(&is_working);
        hook.bind(alt.clone()).block().on_release(move |_| {
            Button::Alt.release();
            is_working.store(false, Ordering::SeqCst);
        });
    }

    let mod_a = hook.cond(Cond::pressed(alt));

    mod_a.bind(tab.clone()).block().on_press(move |_| {
        if !is_working.load(Ordering::SeqCst) {
            Button::Alt.press();
            is_working.store(true, Ordering::SeqCst);
        }
    });
    mod_a.bind(tab).like(Button::Tab);
}

// Emulate Alt-tab with a-t
fn main() {
    let hook = Hook::new();

    emulate_alt_tab(&hook, Button::A, Button::T);

    hook.handle_input();
}
