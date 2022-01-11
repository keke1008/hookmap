use hookmap::prelude::*;

// Emulate Alt-tab with a-t
fn main() {
    let hotkey = Hotkey::new();
    hotkey.bind_alt_tab(arg!(A), arg!(T));
    hotkey.install();
}
