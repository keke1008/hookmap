# hookmap-core

[![Crates.io](https://img.shields.io/crates/v/hookmap-core.svg)](https://crates.io/crates/hookmap-core)
[![API reference](https://docs.rs/hookmap-core/badge.svg)](https://docs.rs/hookmap-core)

Core crate of [hookmap](https://crates.io/crates/hookmap)

Input emulation and global hooks for keyboard and mouse.

## Supported OS

* Windows 10

## Eample

```rust
use hookmap_core::*;

fn main() {
    let event_receiver = HookHandler::install_hook();

    loop {
        let undispatched_event = event_receiver.recv();
        match undispatched_event.event {
            Event::Button(event) => {
                match event.target {
                    Button::RightArrow => println!("Left"),
                    Button::UpArrow => println!("Up"),
                    Button::LeftArrow => println!("Right"),
                    Button::DownArrow => println!("Down"),
                    _ => {
                        undispatched_event.dispatch();
                        continue;
                    }
                };
                undispatched_event.block();
            }
            Event::MouseCursor(cursor) => {
                println!("position: {:?}", cursor);
                undispatched_event.dispatch();
            }
            Event::MouseWheel(speed) => {
                println!("speed: {}", speed);
                undispatched_event.dispatch()
            }
        }
    }
}
```
