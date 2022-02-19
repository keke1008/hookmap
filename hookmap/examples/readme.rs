use hookmap::prelude::*;

fn main() {
    let hotkey = Hotkey::new();

    // Remap H,J,K,L keys as in vim.
    hotkey.remap(Button::H, Button::LeftArrow);
    hotkey.remap(Button::J, Button::DownArrow);
    hotkey.remap(Button::K, Button::UpArrow);
    hotkey.remap(Button::L, Button::RightArrow);

    // You can define hotkeys that work only when specific keys are pressed or released.
    let modified = hotkey.add_modifier_keys(buttons!(LCtrl, !RShift));

    modified.on_press(Button::Space, |_| seq!(with(LCtrl), A).send());

    modified.on_release(buttons!(A, B), |event: ButtonEvent| {
        seq!(with(LShift), [event.target]).send()
    });

    hotkey.install();
}
