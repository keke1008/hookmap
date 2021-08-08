use once_cell::sync::Lazy;

use crate::{ButtonAction, Key, Mouse};
use std::{collections::HashMap, hash::Hash, sync::Mutex};

/// Indicates whether to pass the generated event to the next program or not .
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventBlock {
    /// Do not pass the generated event to the next program.
    Block,

    /// Pass the generated event to the next program.
    Unblock,
}

impl Default for &EventBlock {
    fn default() -> Self {
        &EventBlock::Unblock
    }
}

#[derive(Debug)]
pub struct EventBlockMap<K: Hash + Eq>(Mutex<HashMap<K, EventBlock>>);

impl<K: Hash + Eq> EventBlockMap<K> {
    pub fn get_or_default(&self, k: K) -> EventBlock {
        *self.0.lock().unwrap().get(&k).unwrap_or_default()
    }

    pub fn insert(&mut self, k: K, v: EventBlock) -> Option<EventBlock> {
        self.0.lock().unwrap().insert(k, v)
    }
}

impl<K: Hash + Eq> Default for EventBlockMap<K> {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[derive(Debug, Default)]
pub struct ButtonEventBlock {
    pub keyboard: EventBlockMap<Key>,
    pub mouse: EventBlockMap<Mouse>,
}

pub static BUTTON_EVENT_BLOCK: Lazy<ButtonEventBlock> = Lazy::new(Default::default);

/// Information about the generated event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ButtonEvent<T> {
    /// Target of the generated event.
    pub target: T,

    /// Action of the generated event.
    pub action: ButtonAction,
}

impl<T> ButtonEvent<T> {
    /// Creates a new `ButtonEvent<T, A>`.
    pub fn new(target: T, action: ButtonAction) -> Self {
        Self { target, action }
    }
}
