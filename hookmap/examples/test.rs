use hookmap::*;

fn main() {
    let hotkey = Hotkey::new();
    hotkey!(hotkey => {
        modifier ([any!(A, B)]) {
            bind A => B;
        }
    });

    hotkey.handle_input();
}
