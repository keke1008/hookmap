pub mod structs;
pub mod traits;

pub use structs::{All, Any, ButtonSet, ButtonWithState, ALT, CTRL, META, SHIFT};
pub use traits::{
    ButtonInput, ButtonState, EmulateButtonInput, EmulateButtonState, ToButtonWithState,
};
