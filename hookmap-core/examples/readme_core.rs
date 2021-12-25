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
