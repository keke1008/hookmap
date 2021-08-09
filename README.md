# hookmap

A rust library for Register hotkeys and emulate keyboard and mouse input.

## Supported OS

* Windows 10

## Example

```rust
use hookmap::*;

fn main() {
    let hook = Hook::new();

    hook.bind(Button::A)
        .on_press(|_| println!("The A key was pressed"));

    let mod_shift = hook.modifier(Button::Shift);
    mod_shift
        .bind(Button::A)
        .on_release(|_| println!("The A key was released while the Shift key was pressed"));

    let mod_shift_ctrl = mod_shift.modifier(Button::Ctrl);
    mod_shift_ctrl
        .bind(Button::LeftButton)
        .on_press(|_| println!("The left mouse button was pressed while the Shift key and the Control key were pressed"));

    hook.handle_input();
}
```

