use downcast_rs::{impl_downcast, Downcast};
use hookmap_core::{Button, ButtonInput, ButtonState};
use std::{collections::HashSet, sync::Arc};

#[derive(Debug, Default, Clone)]
pub struct ButtonSet(Arc<HashSet<Button>>);

impl ButtonSet {
    pub fn new(buttons: &[Button]) -> Self {
        let set = buttons.iter().copied().collect();
        Self(Arc::new(set))
    }

    pub fn insert(&self, button: Button) -> Self {
        let mut set = (*self.0).clone();
        set.insert(button);
        Self(Arc::new(set))
    }

    pub fn remove(&self, button: &Button) -> Self {
        let mut set = (*self.0).clone();
        set.remove(button);
        Self(Arc::new(set))
    }

    pub fn any(&self) -> Any {
        Any(Arc::clone(&self.0))
    }

    pub fn all(&self) -> All {
        All(Arc::clone(&self.0))
    }
}

#[derive(Debug, Clone)]
pub struct Any(Arc<HashSet<Button>>);

impl ButtonState for Any {
    fn is_pressed(&self) -> bool {
        self.0.iter().any(Button::is_pressed)
    }
}

#[derive(Debug, Clone)]
pub struct All(Arc<HashSet<Button>>);

impl ButtonState for All {
    fn is_pressed(&self) -> bool {
        self.0.iter().all(Button::is_pressed)
    }
}

impl ButtonInput for All {
    fn press(&self) {
        self.0.iter().for_each(Button::press);
    }

    fn release(&self) {
        self.0.iter().for_each(Button::release);
    }
}

#[derive(Debug, Clone)]
pub enum ButtonWithState {
    Button(Button),
    Any(Any),
    All(All),
}

impl ButtonWithState {
    pub(crate) fn iter_buttons(&self) -> impl Iterator<Item = &Button> + '_ {
        match self {
            ButtonWithState::Button(button) => Iter::Once(Some(&button)),
            ButtonWithState::Any(any) => Iter::Set(any.0.iter()),
            ButtonWithState::All(all) => Iter::Set(all.0.iter()),
        }
    }
}

pub(crate) enum Iter<'a> {
    Once(Option<&'a Button>),
    Set(std::collections::hash_set::Iter<'a, Button>),
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Button;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Iter::Once(button) => button.take(),
            Iter::Set(iter) => iter.next(),
        }
    }
}

impl ButtonState for ButtonWithState {
    fn is_pressed(&self) -> bool {
        match self {
            ButtonWithState::Button(button) => button.is_pressed(),
            ButtonWithState::Any(any) => any.is_pressed(),
            ButtonWithState::All(all) => all.is_pressed(),
        }
    }
}

pub trait DownCastableButtonState: ButtonState + Downcast {
    fn into_button_with_state(self: Box<Self>) -> ButtonWithState {
        let button = self.into_any().downcast::<Button>();
        if let Ok(button) = button {
            return ButtonWithState::Button(*button);
        }
        let button = button.unwrap_err().downcast::<Any>();
        if let Ok(any) = button {
            return ButtonWithState::Any(*any);
        }
        let button = button.unwrap_err().downcast::<All>();
        match button {
            Ok(all) => ButtonWithState::All(*all),
            Err(_) => panic!(),
        }
    }
}
impl<T: ButtonState + Downcast> DownCastableButtonState for T {}
impl_downcast!(DownCastableButtonState);
