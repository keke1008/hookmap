use hookmap::*;

const MODIFIERS: [Button; 8] = [
    Button::LShift,
    Button::RShift,
    Button::LCtrl,
    Button::RCtrl,
    Button::LAlt,
    Button::RAlt,
    Button::LMeta,
    Button::RMeta,
];

fn main() {
    let hook = Hook::new();
    hook.bind(Button::I).block().on_press(|_| {
        let pressed_modifiers = MODIFIERS
            .iter()
            .copied()
            .filter(ButtonState::is_pressed)
            .collect::<Vec<Button>>();

        pressed_modifiers.iter().for_each(ButtonInput::release);
        Button::I.click();
        pressed_modifiers.iter().for_each(ButtonInput::press);
    });

    hook.handle_input();
}
