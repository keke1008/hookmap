use hookmap::*;

// Emulate Alt-tab with a-t
fn main() {
    let hotkey = Hotkey::new();
    hotkey.bind_alt_tab(Button::A, Button::T);
    hotkey.handle_input();
}
