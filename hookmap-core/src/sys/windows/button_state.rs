use std::{collections::HashMap, sync::RwLock};

use once_cell::sync::Lazy;

use crate::button::{Button, ButtonAction};

pub(super) static BUTTON_STATE: Lazy<SyncButtonState> = Lazy::new(SyncButtonState::default);

#[derive(Debug, Default)]
pub(super) struct SyncButtonState(RwLock<ButtonState>);

impl SyncButtonState {
    pub(super) fn reflect_input(&self, button: Button, action: ButtonAction) {
        self.0
            .write()
            .unwrap()
            .set(button, action == ButtonAction::Press);
    }

    pub(super) fn is_pressed(&self, button: Button) -> bool {
        self.0.read().unwrap().get(button)
    }
}

#[derive(Debug, Default)]
struct ButtonState(HashMap<Button, bool>);

impl ButtonState {
    fn set(&mut self, button: Button, state: bool) {
        self.0.insert(button, state);
    }

    fn get(&self, button: Button) -> bool {
        self.0.get(&button).copied().unwrap_or_default()
    }
}
