use hookmap_core::*;

fn main() {
    let event_receiver = HookHandler::install_hook();

    while let Ok(mut event_message) = event_receiver.recv() {
        match event_message.event {
            Event::Button(event) => {
                match event.target {
                    Button::RightArrow => println!("Left"),
                    Button::UpArrow => println!("Up"),
                    Button::LeftArrow => println!("Right"),
                    Button::DownArrow => println!("Down"),
                    _ => {
                        event_message.send_event_block(EventBlock::Unblock);
                        continue;
                    }
                };
                event_message.send_event_block(EventBlock::Block);
            }
            Event::MouseCursor(cursor) => {
                println!("position: {:?}", cursor);
                event_message.send_event_block(EventBlock::Unblock);
            }
            Event::MouseWheel(speed) => {
                println!("speed: {}", speed);
                event_message.send_event_block(EventBlock::Unblock);
            }
        }
    }
}
