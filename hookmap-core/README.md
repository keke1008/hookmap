# hookmap-core

[![Crates.io](https://img.shields.io/crates/v/hookmap-core.svg)](https://crates.io/crates/hookmap-core)
[![API reference](https://docs.rs/hookmap-core/badge.svg)](https://docs.rs/hookmap-core)

A core library of [hookmap](https://crates.io/crates/hookmap)

This library provides input simulation and global hooks for keyboard and mouse.

## Supported OS

* Windows 10

## Eample

```rust
use hookmap_core::{button::Button, event::Event, mouse};

fn main() {
    let rx = hookmap_core::install_hook().unwrap();

    while let Ok((event, native_handler)) = rx.recv() {
        match event {
            Event::Button(event) => {
                native_handler.dispatch();

                match event.target {
                    Button::RightArrow => println!("Left"),
                    Button::UpArrow => println!("Up"),
                    Button::LeftArrow => println!("Right"),
                    Button::DownArrow => println!("Down"),
                    _ => {}
                };
            }

            Event::Cursor(e) => {
                native_handler.block();

                // Reverses mouse cursor movement
                let (dx, dy) = e.delta;
                mouse::move_relative(-dx, -dy);
            }

            Event::Wheel(e) => {
                native_handler.dispatch();
                println!("delta: {}", e.delta);
            }
        }
    }
}
```
