use std::sync::{
    mpsc::{self, Sender},
    Mutex,
};

#[derive(Debug, Clone, Copy)]
pub enum BlockInput {
    Block,
    Unblock,
}

#[derive(Debug)]
pub struct Event<K, A> {
    pub kind: K,
    pub action: A,
    pub block_input: BlockInput,
    block_input_tx: Sender<BlockInput>,
}

impl<K, A> Event<K, A> {
    pub fn new(kind: K, action: A, block_input_tx: Sender<BlockInput>) -> Self {
        Self {
            kind,
            action,
            block_input_tx,
            block_input: BlockInput::Unblock,
        }
    }
}

impl<K, A> Drop for Event<K, A> {
    fn drop(&mut self) {
        self.block_input_tx.send(self.block_input).unwrap();
    }
}

pub type EventCallback<K, A> = Box<dyn FnMut(Event<K, A>) + Send>;

pub trait EventHandlerExt<K, A> {
    fn install_hook() -> Result<(), ()>;
    fn uninstall_hook() -> Result<(), ()>;
}

pub struct EventHandler<K, A> {
    callback: Mutex<Option<EventCallback<K, A>>>,
}

impl<K, A> EventHandler<K, A>
where
    Self: EventHandlerExt<K, A>,
{
    pub fn handle_input(
        &self,
        callback: impl FnMut(Event<K, A>) + Send + 'static,
    ) -> Result<(), ()> {
        *self.callback.lock().unwrap() = Some(Box::new(callback));
        Self::install_hook()
    }

    pub fn stop_handle_input(&self) -> Result<(), ()> {
        Self::uninstall_hook()
    }

    pub fn call(&self, kind: K, action: A) -> BlockInput {
        let (tx, rx) = mpsc::channel();
        let event = Event::new(kind, action, tx);
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
