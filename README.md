# hookmap

[![Crates.io](https://img.shields.io/crates/v/hookmap.svg)](https://crates.io/crates/hookmap)
[![API reference](https://docs.rs/hookmap/badge.svg)](https://docs.rs/hookmap)

A rust library for Register hotkeys and emulate keyboard and mouse input.

## Supported OS

* Windows 10

## Example

```rust
use hookmap::*;

fn main() {
    let hotkey = Hotkey::new();

    hotkey!(hotkey => {

        // Binds the H,J,K,L keys as in vim.
        bind H => LeftArrow;
        bind J => DownArrow;
        bind K => UpArrow;
        bind L => RightArrow;


        // if left ctrl is pressed and right shift is not pressed.
        modifier(LCtrl, !RShift) {

            // Disables the Mouse cursor movement.
            disable MouseMove;

            // Disable the event so that it does not reach other processes.
            block_event {

                // Send Ctrl+A when the Shift and the Space key are pressed.
                on_press Space => |_| send!(with(LCtrl), A);

                // Sends an uppercase A or B when the A or B key is pressed.
                on_release [any!(A, B)] => |event| {
                    send!(with(LShift, [event.target]));
                };
            }
        }
    });

    hotkey.handle_input();
}
```
