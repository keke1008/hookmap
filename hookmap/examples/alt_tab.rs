use hookmap::*;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

fn emulate_alt_tab(hook: &Hook, alt: Key, tab: Key) {
    let is_working = Arc::new(AtomicBool::new(false));

    {
        let is_working = Arc::clone(&is_working);
        hook.bind_key(alt).on_release(move |_| {
            Key::Alt.release();
            is_working.store(false, Ordering::SeqCst);
        });
    }

    let mod_a = hook.modifier_key(alt, EventBlock::Block);

    mod_a.bind_key(tab).on_press(move |mut event| {
        if !is_working.load(Ordering::SeqCst) {
            Key::Alt.press();
            is_working.store(true, Ordering::SeqCst);
        }
        event.block_event();
    });
    mod_a.bind_key(tab).as_key(Key::Tab);
}

// Emulate Alt-tab with a-t
fn main() {
    let hook = Hook::new();

    emulate_alt_tab(&hook, Key::A, Key::T);

    hook.handle_input();
}
