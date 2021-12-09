use crate::button::ButtonSet;
use crate::hotkey::{Action, ModifierKeys, MouseEventHandler};
use hookmap_core::{
    Button, ButtonAction, ButtonEvent, MouseCursorEvent, MouseWheelEvent, NativeEventOperation,
};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub(super) enum HookKind {
    Independet {
        modifier_keys: Arc<ModifierKeys>,
    },
    LinkedOnPress {
        modifier_keys: Arc<ModifierKeys>,
        activated: Arc<AtomicBool>,
    },
    LinkedOnRelease {
        activated: Arc<AtomicBool>,
    },
}

#[derive(Debug, Clone)]
pub(crate) struct HookInfo {
    pub(super) kind: HookKind,
    pub(super) action: Action<ButtonEvent>,
    pub(super) event_block: NativeEventOperation,
}

impl HookInfo {
    pub(super) fn new(
        kind: HookKind,
        action: Action<ButtonEvent>,
        event_block: NativeEventOperation,
    ) -> Self {
        Self {
            kind,
            action,
            event_block,
        }
    }
}

impl HookInfo {
    pub(super) fn satisfies_condition(&self) -> bool {
        match &self.kind {
            HookKind::Independet { modifier_keys } => modifier_keys.satisfies_condition(),
            HookKind::LinkedOnRelease { activated } => activated.swap(false, Ordering::SeqCst),
            HookKind::LinkedOnPress {
                modifier_keys,
                activated,
            } => {
                modifier_keys.satisfies_condition() && {
                    activated.store(true, Ordering::SeqCst);
                    true
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub(super) struct Remap {
    pub(super) modifier_keys: Arc<ModifierKeys>,
    pub(super) target: ButtonSet,
    pub(super) activated: Arc<AtomicBool>,
}

impl Remap {
    pub(super) fn remappable(&self, action: ButtonAction) -> bool {
        match action {
            ButtonAction::Press => {
                self.modifier_keys.satisfies_condition() && {
                    self.activated.store(true, Ordering::SeqCst);
                    true
                }
            }
            ButtonAction::Release => self.activated.swap(false, Ordering::SeqCst),
        }
    }
}

#[derive(Debug)]
pub struct MouseHook<E> {
    pub(super) modifier_keys: Arc<ModifierKeys>,
    pub(super) action: Action<E>,
    pub(super) event_block: NativeEventOperation,
}

#[derive(Default, Debug)]
pub(super) struct ButtonStorage {
    pub(super) just: HashMap<Button, Vec<HookInfo>>,
    pub(super) all: Vec<HookInfo>,
}
pub(super) type MouseStorage<E> = Vec<MouseEventHandler<E>>;
pub(super) type MouseCursorStorage = MouseStorage<MouseCursorEvent>;
pub(super) type MouseWheelStorage = MouseStorage<MouseWheelEvent>;
pub(super) type RemapStorage = HashMap<Button, Vec<Remap>>;

#[derive(Default, Debug)]
pub(super) struct Storage {
    pub(super) remap: HashMap<Button, Vec<Remap>>,
    pub(super) on_press: ButtonStorage,
    pub(super) on_release: ButtonStorage,
    pub(super) mouse_cursor: MouseCursorStorage,
    pub(super) mouse_wheel: MouseWheelStorage,
}
