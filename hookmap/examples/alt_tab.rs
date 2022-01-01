use hookmap::{button_args, hotkey::Hotkey, utils::Utils};

// Emulate Alt-tab with a-t
fn main() {
    let hotkey = Hotkey::new();
    hotkey.bind_alt_tab(button_args!(A), button_args!(T));
    hotkey.install();
}
