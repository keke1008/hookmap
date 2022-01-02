# hookmap

[![Crates.io](https://img.shields.io/crates/v/hookmap.svg)](https://crates.io/crates/hookmap)
[![API reference](https://docs.rs/hookmap/badge.svg)](https://docs.rs/hookmap)

Register hotkeys and simulate keyboard and mosue input.

## Supported OS

* Windows 10

## Example

```rust
use hookmap::prelude::*;

fn main() {
    let hotkey = Hotkey::new();

    hotkey!(hotkey => {

        // Remap H,J,K,L keys as in vim.
        remap H => LeftArrow;
        remap J => DownArrow;
        remap K => UpArrow;
        remap L => RightArrow;


        // if left ctrl is pressed and right shift is not pressed.
        modifier LCtrl, !RShift {

            // Disable the event so that it does not reach other processes.
            block {

                // Send Ctrl+A when the Shift and the Space key are pressed.
                on_press Space => |_| send!(with(LCtrl), A);

                // Sends an uppercase A or B when the A or B key is pressed.
                on_release A, B => |event| {
                    send!(with(LShift, [event.target]));
                };
            }
        }
    });

    hotkey.install();
}
```
