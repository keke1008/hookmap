use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub(super) struct Shared<T> {
    strong: Option<Rc<T>>,
    weak: Weak<T>,
}

impl<T: Default> Default for Shared<T> {
    fn default() -> Self {
        let strong = T::default().into();
        let weak = Rc::downgrade(&strong);

        Self {
            strong: Some(strong),
            weak,
        }
    }
}

impl<T> Shared<T> {
    pub(super) fn weak(&self) -> Self {
        Self {
            strong: None,
            weak: self.weak.clone(),
        }
    }

    pub(super) fn apply<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        f(&mut self.weak.upgrade().unwrap())
    }

    pub(super) fn into_inner(self) -> Option<T> {
        Rc::try_unwrap(self.strong?).ok()
    }
}

impl<T> Shared<RefCell<T>> {
    pub(super) fn apply_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        f(&mut self.weak.upgrade().unwrap().borrow_mut())
    }
}
