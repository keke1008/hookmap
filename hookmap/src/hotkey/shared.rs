use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

#[derive(Debug, Default)]
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
        Rc::try_unwrap(self.0).ok()?.into_inner()
    }
}

impl<T> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}
