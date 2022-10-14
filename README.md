# hookmap

[![Crates.io](https://img.shields.io/crates/v/hookmap.svg)](https://crates.io/crates/hookmap)
[![API reference](https://docs.rs/hookmap/badge.svg)](https://docs.rs/hookmap)

A rust library for Register hotkeys and emulate keyboard and mouse input.

## Supported OS

* Windows 10

## Example

```rust
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
```
