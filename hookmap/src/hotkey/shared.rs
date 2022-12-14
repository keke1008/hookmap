use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

#[derive(Debug)]
pub(super) struct Shared<T>(Rc<RefCell<Option<T>>>);

impl<T> Shared<T> {
    pub(super) fn get(&self) -> Ref<T> {
        Ref::map(self.0.borrow(), |t| {
            t.as_ref().expect("Shared is already unwrapped.")
        })
    }

    pub(super) fn get_mut(&self) -> RefMut<T> {
        RefMut::map(self.0.borrow_mut(), |t| {
            t.as_mut().expect("`Shared` is already unwrapped.")
        })
    }

    pub(super) fn try_unwrap(self) -> Option<T> {
        self.0.take()
    }
}

impl<T> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<T: Default> Default for Shared<T> {
    fn default() -> Self {
        Self(Rc::new(RefCell::new(Some(T::default()))))
    }
}
