# hookmap

A rust library for Register hotkeys and emulate keyboard and mouse input.

## Supported OS

* Windows 10

## Example

```rust
use hookmap::*;

fn main() {
    let hook = Hook::new();

    hook.bind_key(Key::A)
        .on_press(|_| println!("The A key was pressed"));

    let mod_shift = hook.modifier_key(Key::Shift, EventBlock::Unblock);
    mod_shift
        .bind_key(Key::A)
        .on_release(|_| println!("The A key was released while the Shift key was pressed"));

    let mod_shift_ctrl = mod_shift.modifier_key(Key::Ctrl, EventBlock::Unblock);
    mod_shift_ctrl
        .bind_mouse(Mouse::LButton)
        .on_press(|_| println!("The left mouse button was pressed while the Shift key and the Control key were pressed"));

    hook.handle_input();
}
```

