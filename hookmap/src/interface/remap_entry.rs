use std::{cell::RefCell, rc::Weak, sync::Arc};

use crate::{
    hotkey::{ModifierKeys, RemapInfo},
    runtime::Register,
};

use super::ButtonSet;

/// Register remapping information.
pub struct RemapEntry {
    register: Weak<RefCell<Register>>,
    trigger: ButtonSet,
    modifier_keys: Arc<ModifierKeys>,
}

impl RemapEntry {
    pub(super) fn new(
        register: Weak<RefCell<Register>>,
        target: ButtonSet,
        modifier_keys: Arc<ModifierKeys>,
    ) -> Self {
        Self {
            register,
            trigger: target,
            modifier_keys,
        }
    }

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
