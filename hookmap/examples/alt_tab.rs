use hookmap::*;

// Emulate Alt-tab with a-t
fn main() {
    let hook = Hook::new();
    hook.bind_alt_tab(Button::A, Button::T);
    hook.handle_input();
}
