use hookmap_core::button::Button;

use crate::hook::ButtonState;

#[derive(Debug, Default)]
pub(crate) struct RealButtonState;

impl ButtonState for RealButtonState {
    fn is_pressed(&self, button: Button) -> bool {
        button.is_pressed()
    }

    fn is_released(&self, button: Button) -> bool {
        button.is_released()
    }
}
