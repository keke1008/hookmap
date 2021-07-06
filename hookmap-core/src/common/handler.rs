use super::event::{BlockInput, EventDetail};
use std::{
    any, fmt,
    sync::{mpsc, Mutex},
};

pub trait HookInstallable<T, A> {
    fn install_hook() -> Result<(), ()>;
    fn uninstall_hook() -> Result<(), ()>;
}

type EventCallback<T, A> = Box<dyn FnMut(EventDetail<T, A>) + Send>;

pub struct EventHandler<T, A> {
    callback: Mutex<Option<EventCallback<T, A>>>,
}

impl<T, A> EventHandler<T, A>
where
    Self: HookInstallable<T, A>,
{
    pub fn handle_input(
        &self,
        callback: impl FnMut(EventDetail<T, A>) + Send + 'static,
    ) -> Result<(), ()> {
        *self.callback.lock().unwrap() = Some(Box::new(callback));
        Self::install_hook()
    }

    pub fn stop_handle_input(&self) -> Result<(), ()> {
        Self::uninstall_hook()
    }

    pub fn emit(&self, target: T, action: A) -> BlockInput {
        let (tx, rx) = mpsc::channel();
        let event = EventDetail::new(target, action, tx);
        (self.callback.lock().unwrap().as_mut().unwrap())(event);
        rx.recv().unwrap()
    }
}

impl<T, A> Default for EventHandler<T, A> {
    fn default() -> Self {
        Self {
            callback: Mutex::new(None),
        }
    }
}

impl<T, A> fmt::Debug for EventHandler<T, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "EventHandler<{}, {}>",
            any::type_name::<T>(),
            any::type_name::<A>()
        )
    }
}
