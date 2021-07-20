pub mod event;
pub mod handler;
pub mod hook;
pub mod modifier;
pub mod register;

pub use event::{Button, EventInfo};
pub use hook::Hook;
pub use hookmap_core::{
    keyboard::{EmulateKeyboardInput, Key},
    mouse::{EmulateMouseInput, MouseInput as Mouse},
};
