use hookmap::prelude::*;
use hookmap::utils;

// Emulate Alt-tab with a-t
fn main() {
    let mut hotkey = Hotkey::new();
    utils::alt_tab(&mut hotkey, &Context::new(), Button::A, Button::T);
    hotkey.install();
}
