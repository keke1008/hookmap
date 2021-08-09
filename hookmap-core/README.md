# hookmap-core

Input emulation and global hooks for keyboard and mouse.

## Supported OS

* Windows 10

## Eample

```rust
use hookmap_core::{Button, ButtonAction, INPUT_HANDLER};

fn main() {
    INPUT_HANDLER.button.register_handler(|e| {
        match e.target {
            Button::RightArrow => println!("Left"),
            Button::UpArrow => println!("Up"),
            Button::LeftArrow => println!("Left"),
            Button::DownArrow => println!("Down"),
            _ => println!("Other"),
        };

        match e.action {
            ButtonAction::Press => println!("Pressed"),
            ButtonAction::Release => println!("Released"),
        }
    });

    INPUT_HANDLER.handle_input();
}
```
