use std::sync::mpsc::{self, Sender};

use crossbeam_utils::thread::{Scope, ScopedJoinHandle};
use hookmap_core::event::{ButtonEvent, CursorEvent, WheelEvent};

use super::Hook;

type OptionalButtonEvent = Option<ButtonEvent>;

#[derive(Debug)]
pub(super) struct Task<'env, E, H>
where
    E: Send + Copy + 'static,
    H: Hook<E>,
{
    pub(super) event: E,
    pub(super) hooks: Vec<&'env H>,
}

impl<'env, E, H> Task<'env, E, H>
where
    E: Send + Copy + 'static,
    H: Hook<E>,
{
    pub(super) fn new(event: E, hooks: Vec<&'env H>) -> Self {
        Self { event, hooks }
    }
}

pub(super) trait HookBound {
    type Layer: Hook<OptionalButtonEvent>;
    type Remap: Hook<OptionalButtonEvent>;
    type OnPress: Hook<ButtonEvent>;
    type OnRelease: Hook<OptionalButtonEvent>;
    type Cursor: Hook<CursorEvent>;
    type Wheel: Hook<WheelEvent>;
}

impl<Layer, Remap, OnPress, OnRelease, Cursor, Wheel> HookBound
    for (Layer, Remap, OnPress, OnRelease, Cursor, Wheel)
where
    Layer: Hook<OptionalButtonEvent>,
    Remap: Hook<OptionalButtonEvent>,
    OnPress: Hook<ButtonEvent>,
    OnRelease: Hook<OptionalButtonEvent>,
    Cursor: Hook<CursorEvent>,
    Wheel: Hook<WheelEvent>,
{
    type Layer = Layer;
    type Remap = Remap;
    type OnPress = OnPress;
    type OnRelease = OnRelease;
    type Cursor = Cursor;
    type Wheel = Wheel;
}

#[derive(Debug)]
pub(super) enum Message<'env, T: HookBound> {
    Layer(Task<'env, OptionalButtonEvent, T::Layer>),
    Remap(Task<'env, OptionalButtonEvent, T::Remap>),
    OnPress(Task<'env, ButtonEvent, T::OnPress>),
    OnRelease(Task<'env, OptionalButtonEvent, T::OnRelease>),
    Cursor(Task<'env, CursorEvent, T::Cursor>),
    Wheel(Task<'env, WheelEvent, T::Wheel>),
}

#[derive(Debug)]
pub(super) struct MessageQueue<'env, T: HookBound> {
    tx: Sender<Message<'env, T>>,
}

impl<'env, T: HookBound> MessageQueue<'env, T> {
    fn new(tx: Sender<Message<'env, T>>) -> Self {
        Self { tx }
    }

    pub(super) fn enqueue(&self, msg: Message<'env, T>) {
        self.tx.send(msg).unwrap();
    }
}

#[derive(Debug)]
pub(super) struct HookRunner<'env, 'scope, T: HookBound> {
    tx: Sender<Message<'env, T>>,
    handle: ScopedJoinHandle<'scope, ()>,
}

fn handle_task<E, H>(task: Task<E, H>)
where
    E: Send + Copy + 'static,
    H: Hook<E>,
{
    task.hooks.iter().for_each(|hook| hook.run(task.event));
}

impl<'env, 'scope, L, R, OP, OR, C, W> HookRunner<'env, 'scope, (L, R, OP, OR, C, W)>
where
    L: Hook<OptionalButtonEvent>,
    R: Hook<OptionalButtonEvent>,
    OP: Hook<ButtonEvent>,
    OR: Hook<OptionalButtonEvent>,
    C: Hook<CursorEvent>,
    W: Hook<WheelEvent>,
{
    pub(super) fn new(scope: &'scope Scope<'env>) -> Self {
        let (tx, rx) = mpsc::channel();
        let handle = scope.spawn(move |_| {
            while let Ok(msg) = rx.recv() {
                match msg {
                    Message::Layer(task) => handle_task(task),
                    Message::Remap(task) => handle_task(task),
                    Message::OnPress(task) => handle_task(task),
                    Message::OnRelease(task) => handle_task(task),
                    Message::Cursor(task) => handle_task(task),
                    Message::Wheel(task) => handle_task(task),
                }
            }
        });

        Self { tx, handle }
    }
}

impl<'env, 'scope, T: HookBound> HookRunner<'env, 'scope, T> {
    pub(super) fn queue(&self) -> MessageQueue<'env, T> {
        MessageQueue::new(self.tx.clone())
    }

    pub(super) fn terminate(self) {
        drop(self.tx);
        self.handle.join().unwrap();
    }
}
