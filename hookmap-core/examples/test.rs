use hookmap_core::{button::ButtonAction, event::Event};

fn main() {
    let rx = hookmap_core::install_hook();

    while let Ok((e, h)) = rx.recv() {
        match e {
            Event::Button(e) if e.action == ButtonAction::Press => {
                h.dispatch();
                println!("{:?}", e.target);
            }
            _ => {}
        }
    }
}
