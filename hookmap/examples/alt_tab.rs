use hookmap::prelude::*;

// Emulate Alt-tab with a-t
fn main() {
    let hotkey = Hotkey::new();

    hotkey
        .disable(Button::A)
        .modifiers(Button::A)
        .alt_tab(Button::T)
        .shift_alt_tab(Button::R);

    hotkey.install();
}
