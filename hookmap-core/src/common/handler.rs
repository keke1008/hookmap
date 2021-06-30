use super::event::{BlockInput, EventDetail};
use std::{
    any, fmt,
    sync::{mpsc, Mutex},
};

pub trait HookInstallable<K, A> {
    fn install_hook() -> Result<(), ()>;
    fn uninstall_hook() -> Result<(), ()>;
}

type EventCallback<K, A> = Box<dyn FnMut(EventDetail<K, A>) + Send>;

pub struct EventHandler<K, A> {
    callback: Mutex<Option<EventCallback<K, A>>>,
}

impl<K, A> EventHandler<K, A>
where
    Self: HookInstallable<K, A>,
{
    pub fn handle_input(
        &self,
        callback: impl FnMut(EventDetail<K, A>) + Send + 'static,
    ) -> Result<(), ()> {
        *self.callback.lock().unwrap() = Some(Box::new(callback));
        Self::install_hook()
    }

    pub fn stop_handle_input(&self) -> Result<(), ()> {
        Self::uninstall_hook()
    }

    pub fn emit(&self, kind: K, action: A) -> BlockInput {
        let (tx, rx) = mpsc::channel();
        let event = EventDetail::new(kind, action, tx);
        (self.callback.lock().unwrap().as_mut().unwrap())(event);
        rx.recv().unwrap()
    }
}

impl<K, A> Default for EventHandler<K, A> {
    fn default() -> Self {
        Self {
            callback: Mutex::new(None),
        }
    }
}

impl<K, A> fmt::Debug for EventHandler<K, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "EventHandler<{}, {}>",
            any::type_name::<K>(),
            any::type_name::<A>()
        )
    }
}
