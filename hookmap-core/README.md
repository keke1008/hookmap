# hookmap-core

Input emulation and global hooks for keyboard and mouse.

## Supported OS

* Windows 10

## Eample

```rust
use hookmap_core::*;

struct Handler {
    event: ButtonEvent,
}

impl Handler {
    fn new(event: ButtonEvent) -> Self {
        Self { event }
    }
}

impl EventCallback for Handler {
    fn call(&mut self) {
        match self.event.target {
            Button::RightArrow => println!("Left"),
            Button::UpArrow => println!("Up"),
            Button::LeftArrow => println!("Left"),
            Button::DownArrow => println!("Down"),
            _ => println!("Other"),
        };

        match self.event.action {
            ButtonAction::Press => println!("Pressed"),
            ButtonAction::Release => println!("Released"),
        }
    }

    fn get_event_block(&self) -> EventBlock {
        if Button::Shift.is_pressed() {
            EventBlock::Block
        } else {
            EventBlock::Unblock
        }
    }
}

fn main() {
    INPUT_HANDLER
        .button
        .register_handler(|e| Box::new(Handler::new(e)));

    INPUT_HANDLER.handle_input();
}
```
