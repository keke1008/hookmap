use super::{KeyboardRegister, MouseRegister};
use crate::{
    handler::Handler,
    modifier::{ModifierEventBlock, ModifierSet},
};
use hookmap_core::{EventBlock, Key, MouseInput};
use std::{cell::RefCell, rc::Weak, sync::Arc};

#[derive(Debug)]
pub struct Modifier {
    handler: Weak<RefCell<Handler>>,
    modifier: Arc<ModifierSet>,
    modifier_event_block: Weak<RefCell<ModifierEventBlock>>,
}

impl Modifier {
    pub(crate) fn new(
        handler: Weak<RefCell<Handler>>,
        modifier: Arc<ModifierSet>,
        modifier_event_block: Weak<RefCell<ModifierEventBlock>>,
    ) -> Self {
        Self {
            handler,
            modifier,
            modifier_event_block,
        }
    }

    pub fn bind_key(&self, key: Key) -> KeyboardRegister {
        KeyboardRegister::new(Weak::clone(&self.handler), Arc::clone(&self.modifier), key)
    }

    pub fn bind_mouse(&self, mouse: MouseInput) -> MouseRegister {
        MouseRegister::new(
            Weak::clone(&self.handler),
            Arc::clone(&self.modifier),
            mouse,
        )
    }

    pub fn modifier_key(&self, key: Key, event_block: EventBlock) -> Self {
        self.modifier_event_block
            .upgrade()
            .unwrap()
            .borrow_mut()
            .keyboard
            .insert(key, event_block);
        Self {
            handler: Weak::clone(&self.handler),
            modifier: Arc::new(self.modifier.added_key(key)),
            modifier_event_block: Weak::clone(&self.modifier_event_block),
        }
    }

    pub fn modifier_mouse_button(&self, mouse_button: MouseInput, event_block: EventBlock) -> Self {
        self.modifier_event_block
            .upgrade()
            .unwrap()
            .borrow_mut()
            .mouse
            .insert(mouse_button, event_block);
        Self {
            handler: Weak::clone(&self.handler),
            modifier: Arc::new(self.modifier.added_mouse_button(mouse_button)),
            modifier_event_block: Weak::clone(&self.modifier_event_block),
        }
    }
}
