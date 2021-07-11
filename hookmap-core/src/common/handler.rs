use super::event::{EventBlock, EventDetail};
use std::{any, fmt, sync::Mutex};

pub trait HookInstallable<T, A> {
    fn install_hook() -> Result<(), ()>;
    fn uninstall_hook() -> Result<(), ()>;
}

type EventHandler<T, A> = dyn FnMut(EventDetail<T, A>) -> EventBlock + Send;

pub struct HookManager<T, A> {
    handler: Mutex<Option<Box<EventHandler<T, A>>>>,
}

impl<T, A> HookManager<T, A>
where
    Self: HookInstallable<T, A>,
{
    pub fn handle_input(
        &self,
        handler: impl FnMut(EventDetail<T, A>) -> EventBlock + Send + 'static,
    ) -> Result<(), ()> {
        *self.handler.lock().unwrap() = Some(Box::new(handler));
        Self::install_hook()
    }

    pub fn stop_handle_input(&self) -> Result<(), ()> {
        Self::uninstall_hook()
    }

    pub fn emit(&self, target: T, action: A) -> EventBlock {
        let event = EventDetail::new(target, action);
        (self.handler.lock().unwrap().as_mut().unwrap())(event)
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
