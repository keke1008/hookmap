use hookmap::prelude::*;
use Button::*;

fn main() {
    let hotkey = Hotkey::new();

    // Remap H,J,K,L keys like vim.
    hotkey
        .remap(H, LeftArrow)
        .remap(J, DownArrow)
        .remap(K, UpArrow)
        .remap(L, RightArrow);

    let modified = hotkey.modifiers(modifiers![LShift, !RShift]);

    modified
        .disable(Space)
        .on_press(Space, |_| seq![with(Ctrl), A].send());

    hotkey.install();
}
