# hookmap

[![Crates.io](https://img.shields.io/crates/v/hookmap.svg)](https://crates.io/crates/hookmap)
[![API reference](https://docs.rs/hookmap/badge.svg)](https://docs.rs/hookmap)

A rust library for Register hotkeys and emulate keyboard and mouse input.

## Supported OS

* Windows 10

## Example

```rust
use hookmap::prelude::*;

fn main() {
    let mut hotkey = Hotkey::new();

    // Remap H,J,K,L keys as in vim.
    hotkey
        .register(Context::default())
        .remap(Button::H, Button::LeftArrow)
        .remap(Button::J, Button::DownArrow)
        .remap(Button::K, Button::UpArrow)
        .remap(Button::L, Button::RightArrow);

    // You can define hotkeys that work only when specific keys are pressed or released.
    hotkey
        .register(
            Context::new()
                .modifiers(buttons!(LCtrl, !RShift))
                .native_event_operation(NativeEventOperation::Block),
        )
        .on_press(Button::Space, |_| {
            seq!(with(LCtrl), A).send_ignore_modifiers();
        })
        .disable(buttons!(A, B))
        .on_release(buttons!(A, B), |event: ButtonEvent| {
            seq!(with(LShift), [event.target]).send_ignore_modifiers();
        });

    hotkey.install();
}
```
