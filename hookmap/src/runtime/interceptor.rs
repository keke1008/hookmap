//! Gets keyboard events dynamically.

use hookmap_core::button::{Button, ButtonAction};
use hookmap_core::event::{ButtonEvent, NativeEventOperation};
use std::{collections::HashSet, fmt::Debug, sync::mpsc, sync::Arc};

pub(super) mod event_sender {
    use super::Filter;
    use hookmap_core::event::{ButtonEvent, NativeEventOperation};
    use once_cell::sync::Lazy;
    use std::{sync::mpsc::SyncSender, sync::Mutex};

    #[derive(Default)]
    struct EventSender {
        block: Vec<(SyncSender<ButtonEvent>, Filter)>,
        unblock: Vec<(SyncSender<ButtonEvent>, Filter)>,
    }

    impl EventSender {
        fn push(
            &mut self,
            tx: SyncSender<ButtonEvent>,
            filter: Filter,
            native_event_operation: NativeEventOperation,
        ) {
            match native_event_operation {
                NativeEventOperation::Block => self.block.push((tx, filter)),
                NativeEventOperation::Dispatch => self.unblock.push((tx, filter)),
            }
        }

        fn send(&mut self, event: ButtonEvent) -> NativeEventOperation {
            if !self.block.is_empty() {
                let satisfied_index = self
                    .block
                    .iter()
                    .rposition(|(_, filter)| filter.filter(&event));
                if let Some(index) = satisfied_index {
                    let (tx, _) = self.block.remove(index);
                    tx.send(event).unwrap();
                    return NativeEventOperation::Block;
                }
            }

            // drain_filter (https://doc.rust-lang.org/std/vec/struct.Vec.html#method.drain_filter)
            let mut i = 0;
            while i < self.unblock.len() {
                if self.unblock[i].1.filter(&event) {
                    let (tx, _) = self.unblock.remove(i);
                    tx.send(event).unwrap();
                } else {
                    i += 1;
                }
            }
            NativeEventOperation::Dispatch
        }
    }

    static EVENT_SENDERS: Lazy<Mutex<EventSender>> = Lazy::new(Mutex::default);

    pub(super) fn push(
        tx: SyncSender<ButtonEvent>,
        filter: Filter,
        native_event_operation: NativeEventOperation,
    ) {
        EVENT_SENDERS
            .lock()
            .unwrap()
            .push(tx, filter, native_event_operation);
    }

    pub(in super::super) fn send(event: ButtonEvent) -> NativeEventOperation {
        EVENT_SENDERS.lock().unwrap().send(event)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use hookmap_core::button::{Button, ButtonAction};
        use std::sync::mpsc;

        #[test]
        fn event_sender_sends_block_events() {
            let mut event_sender = EventSender::default();
            let (tx, rx) = mpsc::sync_channel(1);
            let filter = Filter::new();
            event_sender.push(tx, filter, NativeEventOperation::Block);

            let event = ButtonEvent::new(Button::A, ButtonAction::Press);
            event_sender.send(event);
            assert_eq!(event, rx.recv().unwrap());
        }

        #[test]
        fn event_sender_does_not_send_block_events() {
            let mut event_sender = EventSender::default();
            let (tx, rx) = mpsc::sync_channel(1);
            let filter = Filter::new().target(Button::A);
            event_sender.push(tx, filter, NativeEventOperation::Block);

            let event = ButtonEvent::new(Button::B, ButtonAction::Press);
            event_sender.send(event);
            assert!(rx.try_recv().is_err());
        }

        #[test]
        fn event_sender_does_not_send_unblock_events() {
            let mut event_sender = EventSender::default();

            let (tx_unblock, rx_unblock) = mpsc::sync_channel(1);
            let filter = Filter::new();
            event_sender.push(tx_unblock, filter.clone(), NativeEventOperation::Dispatch);

            let (tx_block, rx_block) = mpsc::sync_channel(1);
            event_sender.push(tx_block, filter, NativeEventOperation::Block);

            let event = ButtonEvent::new(Button::A, ButtonAction::Press);
            event_sender.send(event);
            assert!(rx_unblock.try_recv().is_err());
            assert_eq!(rx_block.recv().unwrap(), event);

            let event = ButtonEvent::new(Button::B, ButtonAction::Press);
            event_sender.send(event);
            assert_eq!(rx_unblock.recv().unwrap(), event);
        }

        #[test]
        fn event_sender_sends_the_same_event_to_all_unblock_event_receiver() {
            let mut event_sender = EventSender::default();

            let (tx1, rx1) = mpsc::sync_channel(1);
            let filter = Filter::new();
            event_sender.push(tx1, filter.clone(), NativeEventOperation::Dispatch);

            let (tx2, rx2) = mpsc::sync_channel(1);
            let filter = Filter::new();
            event_sender.push(tx2, filter, NativeEventOperation::Dispatch);

            let event = ButtonEvent::new(Button::C, ButtonAction::Release);
            event_sender.send(event);
            assert_eq!(rx1.recv().unwrap(), event);
            assert_eq!(rx2.recv().unwrap(), event);
        }
    }
}

#[derive(Debug, Clone)]
enum Target {
    Single(Button),
    Multiple(Arc<HashSet<Button>>),
}

#[derive(Clone)]
struct Callback(Arc<dyn Fn(&ButtonEvent) -> bool + Send + Sync>);

impl Callback {
    fn new<F>(callback: F) -> Self
    where
        F: Fn(&ButtonEvent) -> bool + Send + Sync + 'static,
    {
        Self(Arc::new(callback))
    }
}

impl Debug for Callback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Callback")
    }
}

/// Filters input events.
///
/// # Examples
///
/// ```
/// use hookmap::prelude::*;
///
/// let filter = Filter::new()
///     .target(Button::A)
///     .action(ButtonAction::Press);
/// ```
///
#[derive(Debug, Default, Clone)]
pub struct Filter {
    target: Option<Target>,
    action: Option<ButtonAction>,
    callback: Vec<Callback>,
}

impl Filter {
    /// Creates a new instance of [`Filter`]
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::interceptor::Filter;
    ///
    /// let filter = Filter::new();
    /// ```
    ///
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the target of events.
    /// This setting will be overridden by [`Filter::targets`].
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::prelude::*;
    ///
    /// let filter = Filter::new().target(Button::A);
    /// ```
    ///
    pub fn target(mut self, target: Button) -> Self {
        self.target = Some(Target::Single(target));
        self
    }

    /// Set multiple targets of events.
    /// This setting will be overridden by [`Filter::target`].
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::prelude::*;
    ///
    /// let targets = [Button::A, Button::B].iter().copied().collect();
    /// let filter = Filter::new().targets(targets);
    /// ```
    ///
    pub fn targets(mut self, targets: HashSet<Button>) -> Self {
        self.target = Some(Target::Multiple(Arc::new(targets)));
        self
    }

    /// Set the action of events.
    ///
    /// # Examples
    /// ```
    /// use hookmap::prelude::*;
    ///
    /// let filter = Filter::new().action(ButtonAction::Press);
    /// ```
    ///
    pub fn action(mut self, action: ButtonAction) -> Self {
        self.action = Some(action);
        self
    }

    pub fn callback<F>(mut self, callback: F) -> Self
    where
        F: Fn(&ButtonEvent) -> bool + Send + Sync + 'static,
    {
        self.callback.push(Callback::new(callback));
        self
    }

    fn filter(&self, event: &ButtonEvent) -> bool {
        self.action
            .map(|action| action == event.action)
            .unwrap_or(true)
            && match self.target {
                Some(Target::Single(button)) => event.target == button,
                Some(Target::Multiple(ref buttons)) => buttons.contains(&event.target),
                None => true,
            }
            && self.callback.iter().all(|callback| callback.0(event))
    }
}

pub struct Iter {
    filter: Filter,
    native_event_operation: NativeEventOperation,
}

impl Iter {
    fn new(filter: Filter, native_event_operation: NativeEventOperation) -> Self {
        Iter {
            filter,
            native_event_operation,
        }
    }
}

impl Iterator for Iter {
    type Item = ButtonEvent;

    fn next(&mut self) -> Option<Self::Item> {
        let (tx, rx) = mpsc::sync_channel(0);
        event_sender::push(tx, self.filter.clone(), self.native_event_operation);
        Some(rx.recv().unwrap())
    }
}

/// Set the hook that receives input events;
///
/// # Examples
///
/// ```no_run
/// use hookmap::prelude::*;
///
/// let filter = Filter::new().action(ButtonAction::Press);
/// Interceptor::unblock(filter).then(|event| {
///     println!("{:?}, {:?}", event.target, event.action);
/// });
///
/// ```
///
pub struct Interceptor {
    filter: Filter,
    native_event_operation: NativeEventOperation,
}

impl Interceptor {
    /// Creates a new instance of [`Interceptor`].
    /// This hook disables keyboard events.
    ///
    /// ```no_run
    /// use hookmap::prelude::*;
    ///
    /// let filter = Filter::new();
    /// Interceptor::block(filter).then(|event| {
    ///     println!("This event was disabled: {:?}", event);
    /// });
    /// ```
    ///
    pub fn block(filter: Filter) -> Self {
        Self {
            filter,
            native_event_operation: NativeEventOperation::Block,
        }
    }

    /// Creates a new instance of [`Interceptor`].
    /// This hook doen not disable keyboard events.
    ///
    /// ```no_run
    /// use hookmap::prelude::*;
    ///
    /// let filter = Filter::new();
    /// Interceptor::unblock(filter).then(|event| {
    ///     println!("This event was disabled: {:?}", event);
    /// });
    /// ```
    ///
    pub fn unblock(filter: Filter) -> Self {
        Self {
            filter,
            native_event_operation: NativeEventOperation::Dispatch,
        }
    }

    /// Receives a single event.
    ///
    /// ```no_run
    /// use hookmap::prelude::*;
    ///
    /// let filter = Filter::new();
    /// Interceptor::block(filter).then(|event| {
    ///     println!("{:?}", event);
    /// });
    /// ```
    ///
    pub fn then<F>(&self, f: F)
    where
        F: Fn(ButtonEvent) + Send + Sync + 'static,
    {
        let mut iter = Iter::new(self.filter.clone(), self.native_event_operation);
        std::thread::spawn(move || f(iter.next().unwrap()));
    }

    /// Receives multimple hooks as iterator.
    ///
    /// ```no_run
    /// use hookmap::prelude::*;
    ///
    /// let filter = Filter::new();
    /// Interceptor::block(filter).then_iter(|mut iter| {
    ///     while let Some(event) = iter.next() {
    ///         match event.target {
    ///             Button::A => break,
    ///             Button::B => iter.next().unwrap().target.press(),
    ///             Button::C => println!("C"),
    ///             _ => {}
    ///         }
    ///     }
    /// });
    /// ```
    pub fn then_iter<F>(&self, f: F)
    where
        F: Fn(Iter) + Send + Sync + 'static,
    {
        let iter = Iter::new(self.filter.clone(), self.native_event_operation);
        std::thread::spawn(move || f(iter));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filtering_events_by_target_matching_conditions() {
        let filter = Filter::new().target(Button::A);

        let mut event = ButtonEvent::new(Button::A, ButtonAction::Press);
        assert!(filter.filter(&event));

        event.action = ButtonAction::Release;
        assert!(filter.filter(&event));
    }

    #[test]
    fn filtering_events_by_target_not_matching_conditions() {
        let filter = Filter::new().target(Button::A);

        let mut event = ButtonEvent::new(Button::B, ButtonAction::Press);
        assert!(!filter.filter(&event));

        event.action = ButtonAction::Press;
        assert!(!filter.filter(&event));
    }

    #[test]
    fn filtering_events_by_targets_matching_conditions() {
        let targets = [Button::A, Button::B].iter().cloned().collect();
        let filter = Filter::new().targets(targets);

        let mut event = ButtonEvent::new(Button::A, ButtonAction::Press);
        assert!(filter.filter(&event));

        event.action = ButtonAction::Release;
        assert!(filter.filter(&event));

        event.target = Button::B;
        assert!(filter.filter(&event));
    }

    #[test]
    fn filtering_events_by_targets_not_matching_conditions() {
        let targets = [Button::A, Button::B].iter().cloned().collect();
        let filter = Filter::new().targets(targets);

        let mut event = ButtonEvent::new(Button::C, ButtonAction::Press);
        assert!(!filter.filter(&event));

        event.action = ButtonAction::Release;
        assert!(!filter.filter(&event));
    }

    #[test]
    fn filtering_events_by_action() {
        let filter = Filter::new().action(ButtonAction::Press);

        let mut event = ButtonEvent::new(Button::A, ButtonAction::Press);
        assert!(filter.filter(&event));

        event.action = ButtonAction::Release;
        assert!(!filter.filter(&event));

        let filter = Filter::new().action(ButtonAction::Release);

        assert!(filter.filter(&event));

        event.action = ButtonAction::Press;
        assert!(!filter.filter(&event));
    }

    #[test]
    fn filtering_events_by_target_and_action() {
        let filter = Filter::new().target(Button::A).action(ButtonAction::Press);

        let mut event = ButtonEvent::new(Button::A, ButtonAction::Press);
        assert!(filter.filter(&event));

        event.action = ButtonAction::Release;
        assert!(!filter.filter(&event));

        event.target = Button::B;
        assert!(!filter.filter(&event));

        event.action = ButtonAction::Press;
        assert!(!filter.filter(&event));
    }

    #[test]
    fn filtering_events_by_targets_and_action() {
        let targets = [Button::A, Button::B].iter().cloned().collect();
        let filter = Filter::new().targets(targets).action(ButtonAction::Press);

        let mut event = ButtonEvent::new(Button::A, ButtonAction::Press);
        assert!(filter.filter(&event));

        event.target = Button::B;
        assert!(filter.filter(&event));

        event.target = Button::C;
        assert!(!filter.filter(&event));

        event.target = Button::B;
        event.action = ButtonAction::Release;
        assert!(!filter.filter(&event));
    }

    #[test]
    fn filtering_events_by_callback() {
        let filter = Filter::new().callback(|e| e.action == ButtonAction::Press);

        let mut event = ButtonEvent::new(Button::A, ButtonAction::Press);
        assert!(filter.filter(&event));

        event.action = ButtonAction::Release;
        assert!(!filter.filter(&event));
    }
}
