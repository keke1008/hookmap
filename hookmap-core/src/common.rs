pub mod button;
pub mod event;
pub mod handler;
pub mod mouse;

use event::ButtonEventBlockMap;
use handler::InputHandler;

use once_cell::sync::Lazy;

pub static BUTTON_EVENT_BLOCK: Lazy<ButtonEventBlockMap> = Lazy::new(Default::default);

pub static INPUT_HANDLER: Lazy<InputHandler> = Lazy::new(InputHandler::default);
