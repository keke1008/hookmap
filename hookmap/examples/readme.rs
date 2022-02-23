use hookmap::prelude::*;

fn main() {
    let hotkey = Hotkey::new();

    // Remap H,J,K,L keys as in vim.
    hotkey
        .remap(Button::H, Button::LeftArrow)
        .remap(Button::J, Button::DownArrow)
        .remap(Button::K, Button::UpArrow)
        .remap(Button::L, Button::RightArrow);

    // You can define hotkeys that work only when specific keys are pressed or released.
    let modified = hotkey.add_modifiers(buttons!(LCtrl, !RShift));

    modified
        .block_input_event()
        .on_press(Button::Space, |_| {
            seq!(with(LCtrl), A).send_ignore_modifiers();
        })
        .disable(buttons!(A, B))
        .on_release(buttons!(A, B), |event: ButtonEvent| {
            seq!(with(LShift), [event.target]).send_ignore_modifiers();
        });

    hotkey.install();
}
