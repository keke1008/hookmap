# hookmap

A rust library for Register hotkeys and emulate keyboard and mouse input.

## Supported OS

* Windows 10

## Example

```rust
use hookmap::*;

fn main() {
    let hook = Hook::new();

    hotkey!(hook => {
        // Binds the H,J,K,L keys as in vim.
        bind H => LeftArrow;
        bind J => DownArrow;
        bind K => UpArrow;
        bind L => RightArrow;

        if (pressed [&CTRL]) {
            // Disables the Mouse cursor movement while the Shift key is held down.
            disable MouseMove;


            // Blocks default button/mouse event;
            block_event {

                // Send Ctrl+A when the Shift and the Space key are pressed.
                on_press Space => |_| send!(LCtrl down, A, LCtrl up);

                // Sends an uppercase A or B when the A or B key is pressed.
                on_press [button_set!(A, B).any()] => |event| {
                    send!(LShift down, [event.target], LShift up)
                };
            }
        }
    });

    hook.handle_input();
}
```
