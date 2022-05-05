use hookmap_core::{button::Button, event::Event};

fn main() {
    let rx = hookmap_core::install_hook();

    while let Ok((event, native_handler)) = rx.recv() {
        match event {
            Event::Button(event) => {
                match event.target {
                    Button::RightArrow => println!("Left"),
                    Button::UpArrow => println!("Up"),
                    Button::LeftArrow => println!("Right"),
                    Button::DownArrow => println!("Down"),
                    _ => {
                        native_handler.dispatch();
                        continue;
                    }
                };
                native_handler.block();
            }

            Event::MouseCursor(cursor) => {
                println!("position: {:?}", cursor);
                native_handler.dispatch();
            }

            Event::MouseWheel(speed) => {
                println!("speed: {}", speed);
                native_handler.dispatch()
            }
        }
    }
}
