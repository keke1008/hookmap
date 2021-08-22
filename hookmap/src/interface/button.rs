use hookmap_core::{Button, ButtonInput, ButtonState};
use std::{borrow::Borrow, collections::HashSet, fmt::Debug, sync::Arc};

/// A struct for operating multiple buttons.
///
/// # Example
///
/// ```
/// use hookmap::*;
/// let hook = Hook::new();
/// let set1 = ButtonSet::new([Button::A, Button::B, Button::C]);
///
/// hook.bind(set1.any())
///     .on_press(|e| println!("{:?}", e));
///
/// let set2 = ButtonSet::new([Button::D, Button::E]);
/// hook.cond(Cond::pressed(set1.all()))
///     .cond(Cond::released(set2.any()))
///     .bind(Button::Q)
///     .on_release(|e| println!("{:?}", e));
/// ```
///
#[derive(Debug, Default, Clone)]
pub struct ButtonSet(Arc<HashSet<Button>>);

impl ButtonSet {
    /// Creates a new instance of `ButtonSet`.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::*;
    /// let set = ButtonSet::new([Button::A, Button::B]);
    /// ```
    ///
    pub fn new(buttons: impl Borrow<[Button]>) -> Self {
        let set = buttons.borrow().iter().copied().collect();
        Self(Arc::new(set))
    }

    /// Creates a clone and inserts the button.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::*;
    /// let set1 = ButtonSet::new([Button::A, Button::B]);
    /// let set2 = set1.insert(Button::C);
    /// ```
    ///
    pub fn insert(&self, button: Button) -> Self {
        let mut set = (*self.0).clone();
        set.insert(button);
        Self(Arc::new(set))
    }

    /// Creates a clone and remove the button.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::*;
    /// let set1 = ButtonSet::new([Button::A, Button::B]);
    /// let set2 = set1.remove(Button::C);
    /// ```
    ///
    pub fn remove(&self, button: Button) -> Self {
        let mut set = (*self.0).clone();
        set.remove(&button);
        Self(Arc::new(set))
    }

    /// Creates a new [`Any`] to operate any button.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::*;
    /// let set = ButtonSet::new([Button::A, Button::B]);
    /// let any = set.any();
    /// ```
    ///
    pub fn any(&self) -> Any {
        Any(Arc::clone(&self.0))
    }

    /// Creates a new [`All`] to operate all buttons.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::*;
    /// let set = ButtonSet::new([Button::A, Button::B]);
    /// let any = set.any();
    /// ```
    ///
    pub fn all(&self) -> All {
        All(Arc::clone(&self.0))
    }
}

/// A struct foe operating any buttons.
///
/// # Example
///
/// ```
/// use hookmap::*;
/// let hook = Hook::new();
/// let any = ButtonSet::new([Button::A, Button::B]).any();
/// hook.bind(any)
///     .on_press(|e| {
///         assert!(e.target == Button::A || e.target == Button::B);
///     });
/// ```
///
#[derive(Debug, Clone)]
pub struct Any(Arc<HashSet<Button>>);

impl ButtonState for Any {
    fn is_pressed(&self) -> bool {
        self.0.iter().any(Button::is_pressed)
    }

    fn is_released(&self) -> bool {
        self.0.iter().any(Button::is_released)
    }
}

/// A struct for operating all buttons.
///
/// # Example
///
/// ```
/// use hookmap::*;
/// let hook = Hook::new();
/// let all = ButtonSet::new([Button::A, Button::B]).all();
/// hook.bind(all)
///     .on_press(|e| {
///         assert!(e.target == Button::A || e.target == Button::B);
///         assert!(Button::A.is_pressed() && Button::B.is_pressed());
///     });
/// ```
///
#[derive(Debug, Clone)]
pub struct All(Arc<HashSet<Button>>);

impl ButtonState for All {
    fn is_pressed(&self) -> bool {
        self.0.iter().all(Button::is_pressed)
    }

    fn is_released(&self) -> bool {
        self.0.iter().all(Button::is_released)
    }
}

impl ButtonInput for All {
    fn press(&self) {
        self.0.iter().for_each(Button::press);
    }

    fn release(&self) {
        self.0.iter().for_each(Button::release);
    }

    fn press_recursive(&self) {
        self.0.iter().for_each(Button::press_recursive);
    }

    fn release_recursive(&self) {
        self.0.iter().for_each(Button::release_recursive);
    }
}

#[derive(Clone)]
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

impl Debug for ButtonWithState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ButtonWithState::Button(button) => f.write_fmt(format_args!("{:?}", button)),
            ButtonWithState::Any(any) => f.write_fmt(format_args!("{:?}", any)),
            ButtonWithState::All(all) => f.write_fmt(format_args!("{:?}", all)),
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

    fn is_released(&self) -> bool {
        match self {
            ButtonWithState::Button(button) => button.is_released(),
            ButtonWithState::Any(any) => any.is_released(),
            ButtonWithState::All(all) => all.is_released(),
        }
    }
}

pub trait EmulateButtonInput: ButtonInput + Send + Sync + Clone + 'static {}
pub trait EmulateButtonState: ToButtonWithState + Send + Sync + Clone + 'static {}

impl<T: ButtonInput + Send + Sync + Clone + 'static> EmulateButtonInput for T {}
impl<T: ToButtonWithState + Send + Sync + Clone + 'static> EmulateButtonState for T {}

/// A trait that allows references to the types that impl [`ButtonInput`] to be given as arguments.
pub trait BorrowedEmulateButtonInput<T: EmulateButtonInput>: Borrow<T> {
    fn clone_static(&self) -> T {
        T::clone(self.borrow())
    }
}
impl<T: EmulateButtonInput> BorrowedEmulateButtonInput<T> for T {}
impl<T: EmulateButtonInput> BorrowedEmulateButtonInput<T> for &T {}

pub trait ToButtonWithState: Send + Sync {
    fn to_button_with_state(&self) -> ButtonWithState;
}

impl<T: ToButtonWithState> ToButtonWithState for &T {
    fn to_button_with_state(&self) -> ButtonWithState {
        (*self).to_button_with_state()
    }
}

impl ToButtonWithState for Button {
    fn to_button_with_state(&self) -> ButtonWithState {
        ButtonWithState::Button(*self)
    }
}

impl ToButtonWithState for Any {
    fn to_button_with_state(&self) -> ButtonWithState {
        ButtonWithState::Any(self.clone())
    }
}

impl ToButtonWithState for All {
    fn to_button_with_state(&self) -> ButtonWithState {
        ButtonWithState::All(self.clone())
    }
}
