use super::event::{EventBlock, EventDetail};
use std::{
    any, fmt,
    sync::{mpsc, Mutex},
};

pub trait HookInstallable<T, A> {
    fn install_hook() -> Result<(), ()>;
    fn uninstall_hook() -> Result<(), ()>;
}

type EventCallback<T, A> = Box<dyn FnMut(EventDetail<T, A>) + Send>;

pub struct HookManager<T, A> {
    handler: Mutex<Option<EventCallback<T, A>>>,
}

impl<T, A> HookManager<T, A>
where
    Self: HookInstallable<T, A>,
{
    pub fn handle_input(
        &self,
        callback: impl FnMut(EventDetail<T, A>) + Send + 'static,
    ) -> Result<(), ()> {
        *self.handler.lock().unwrap() = Some(Box::new(callback));
        Self::install_hook()
    }

    pub fn stop_handle_input(&self) -> Result<(), ()> {
        Self::uninstall_hook()
    }

    pub fn emit(&self, target: T, action: A) -> EventBlock {
        let (tx, rx) = mpsc::channel();
        let event = EventDetail::new(target, action, tx);
        (self.handler.lock().unwrap().as_mut().unwrap())(event);
        rx.recv().unwrap()
    }
}

impl<T, A> Default for HookManager<T, A> {
    fn default() -> Self {
        Self {
            handler: Mutex::new(None),
        }
    }
}

impl<T, A> fmt::Debug for HookManager<T, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "EventHandler<{}, {}>",
            any::type_name::<T>(),
            any::type_name::<A>()
        )
    }
}
