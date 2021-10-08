use super::ButtonSet;
use crate::{
    hotkey::{ModifierKeys, RemapInfo},
    runtime::Register,
};
use std::{cell::RefCell, rc::Weak, sync::Arc};
use typed_builder::TypedBuilder;

/// Register remapping information.
#[derive(TypedBuilder)]
pub struct RemapEntry {
    register: Weak<RefCell<Register>>,
    trigger: ButtonSet,

    #[builder(default)]
    modifier_keys: Arc<ModifierKeys>,
}

impl RemapEntry {
    /// Determines which key to remap to.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::*;
    /// let hotkey = Hotkey::new();
    /// hotkey.remap(Button::A).to(Button::B);
    /// ```
    ///
    pub fn to(&self, button: impl Into<ButtonSet>) {
        let remap_info = RemapInfo {
            modifier_keys: Arc::clone(&self.modifier_keys),
            trigger: self.trigger.clone(),
            target: button.into(),
        };
        self.register
            .upgrade()
            .unwrap()
            .borrow_mut()
            .register_remap(remap_info)
    }
}
