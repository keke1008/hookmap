# hookmap-core

Input emulation and global hooks for keyboard and mouse.

## Supported OS

* Windows 10

## Eample

```rust
use hookmap_core::{ButtonAction, EventBlock, Key, INPUT_HANDLER};

fn main() {
    INPUT_HANDLER
        .keyboard
        .lock()
        .unwrap()
        .register_handler(|e| {
            match e.target {
                Key::RightArrow => println!("Left"),
                Key::UpArrow => println!("Up"),
                Key::LeftArrow => println!("Left"),
                Key::DownArrow => println!("Down"),
                _ => println!("Other"),
            };

            match e.action {
                ButtonAction::Press => println!("Pressed"),
                ButtonAction::Release => println!("Released"),
            }

            EventBlock::Unblock
        });

    INPUT_HANDLER.handle_input();
}
```
