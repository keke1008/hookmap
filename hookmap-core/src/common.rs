pub mod button;
pub mod event;
pub mod handler;
pub mod mouse;

use handler::InputHandler;

use once_cell::sync::Lazy;

pub static INPUT_HANDLER: Lazy<InputHandler> = Lazy::new(InputHandler::default);
