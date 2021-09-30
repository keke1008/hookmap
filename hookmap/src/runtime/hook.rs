//! Gets keyboard events dynamically.

use hookmap_core::{Button, ButtonAction, ButtonEvent, EventBlock};
use std::{collections::HashSet, sync::mpsc, sync::Arc};

pub(super) mod event_sender {
    use super::Filter;
    use hookmap_core::{ButtonEvent, EventBlock};
    use once_cell::sync::Lazy;
    use std::{sync::mpsc::SyncSender, sync::Mutex};

    #[derive(Default)]
    struct EventSender {
        block: Vec<(SyncSender<ButtonEvent>, Filter)>,
        unblock: Vec<(SyncSender<ButtonEvent>, Filter)>,
    }

    impl EventSender {
        fn push(&mut self, tx: SyncSender<ButtonEvent>, filter: Filter, event_block: EventBlock) {
            match event_block {
                EventBlock::Block => self.block.push((tx, filter)),
                EventBlock::Unblock => self.unblock.push((tx, filter)),
            }
        }

        fn send(&mut self, event: ButtonEvent) -> EventBlock {
            if !self.block.is_empty() {
                let satisfied_index = self
                    .block
                    .iter()
                    .rposition(|(_, filter)| filter.filter(&event));
                if let Some(index) = satisfied_index {
                    let (tx, _) = self.block.remove(index);
                    tx.send(event).unwrap();
                    return EventBlock::Block;
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
            EventBlock::Unblock
        }
    }

    static EVENT_SENDERS: Lazy<Mutex<EventSender>> = Lazy::new(Mutex::default);

    pub(super) fn push(tx: SyncSender<ButtonEvent>, filter: Filter, event_block: EventBlock) {
        EVENT_SENDERS.lock().unwrap().push(tx, filter, event_block);
    }

    pub(in super::super) fn send(event: ButtonEvent) -> EventBlock {
        EVENT_SENDERS.lock().unwrap().send(event)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use hookmap_core::{Button, ButtonAction};
        use std::sync::mpsc;

        #[test]
        fn event_sender_sends_block_events() {
            let mut event_sender = EventSender::default();
            let (tx, rx) = mpsc::sync_channel(1);
            let filter = Filter::new();
            event_sender.push(tx, filter, EventBlock::Block);

            let event = ButtonEvent::new(Button::A, ButtonAction::Press);
            event_sender.send(event);
            assert_eq!(event, rx.recv().unwrap());
        }

        #[test]
        fn event_sender_does_not_send_block_events() {
            let mut event_sender = EventSender::default();
            let (tx, rx) = mpsc::sync_channel(1);
            let filter = Filter::new().target(Button::A);
            event_sender.push(tx, filter, EventBlock::Block);

            let event = ButtonEvent::new(Button::B, ButtonAction::Press);
            event_sender.send(event);
            assert!(rx.try_recv().is_err());
        }

        #[test]
        fn event_sender_does_not_send_unblock_events() {
            let mut event_sender = EventSender::default();

            let (tx_unblock, rx_unblock) = mpsc::sync_channel(1);
            let filter = Filter::new();
            event_sender.push(tx_unblock, filter.clone(), EventBlock::Unblock);

            let (tx_block, rx_block) = mpsc::sync_channel(1);
            event_sender.push(tx_block, filter, EventBlock::Block);

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
            event_sender.push(tx1, filter.clone(), EventBlock::Unblock);

            let (tx2, rx2) = mpsc::sync_channel(1);
            let filter = Filter::new();
            event_sender.push(tx2, filter, EventBlock::Unblock);

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

/// Filters input events.
///
/// # Examples
///
/// ```
/// use hookmap::{*, hook::Filter};
/// let filter = Filter::new()
///     .target(Button::A)
///     .action(ButtonAction::Press);
/// ```
///
#[derive(Debug, Default, Clone)]
pub struct Filter {
    target: Option<Target>,
    action: Option<ButtonAction>,
}

impl Filter {
    /// Creates a new instance of [`Filter`]
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::hook::Filter;
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
    /// use hookmap::{*, hook::Filter};
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
    /// use hookmap::{*, hook::Filter};
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
    /// use hookmap::{*, hook::Filter};
    /// let filter = Filter::new().action(ButtonAction::Press);
    /// ```
    ///
    pub fn action(mut self, action: ButtonAction) -> Self {
        self.action = Some(action);
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
    }
}

pub struct Iter {
    filter: Filter,
    event_block: EventBlock,
}

impl Iter {
    fn new(filter: Filter, event_block: EventBlock) -> Self {
        Iter {
            filter,
            event_block,
        }
    }
}

impl Iterator for Iter {
    type Item = ButtonEvent;

    fn next(&mut self) -> Option<Self::Item> {
        let (tx, rx) = mpsc::sync_channel(0);
        event_sender::push(tx, self.filter.clone(), self.event_block);
        Some(rx.recv().unwrap())
    }
}

/// Set the hook that receives input events;
///
/// # Examples
///
/// ```no_run
/// use hookmap::{*, hook::{Hook, Filter}};
/// let filter = Filter::new().action(ButtonAction::Press);
/// Hook::unblock(filter).then(|event| {
///     println!("{:?}, {:?}", event.target, event.action);
/// });
///
/// ```
///
pub struct Hook {
    filter: Filter,
    event_block: EventBlock,
}

impl Hook {
    /// Creates a new instance of [`Hook`].
    /// This hook disables keyboard events.
    ///
    /// ```no_run
    /// use hookmap::{*, hook::{Hook, Filter}};
    /// let filter = Filter::new();
    /// Hook::block(filter).then(|event| {
    ///     println!("This event was disabled: {:?}", event);
    /// });
    /// ```
    ///
    pub fn block(filter: Filter) -> Self {
        Self {
            filter,
            event_block: EventBlock::Block,
        }
    }

    /// Creates a new instance of [`Hook`].
    /// This hook doen not disable keyboard events.
    ///
    /// ```no_run
    /// use hookmap::{*, hook::{Hook, Filter}};
    /// let filter = Filter::new();
    /// Hook::unblock(filter).then(|event| {
    ///     println!("This event was disabled: {:?}", event);
    /// });
    /// ```
    ///
    pub fn unblock(filter: Filter) -> Self {
        Self {
            filter,
            event_block: EventBlock::Unblock,
        }
    }

    /// Receives a single event.
    ///
    /// ```no_run
    /// use hookmap::{*, hook::{Hook, Filter}};
    /// let filter = Filter::new();
    /// Hook::block(filter).then(|event| {
    ///     println!("{:?}", event);
    /// });
    /// ```
    ///
    pub fn then<F>(&self, f: F)
    where
        F: Fn(ButtonEvent) + Send + Sync + 'static,
    {
        let mut iter = Iter::new(self.filter.clone(), self.event_block);
        std::thread::spawn(move || f(iter.next().unwrap()));
    }

    /// Receives multimple hooks as iterator.
    ///
    /// ```no_run
    /// use hookmap::{*, hook::{Hook, Filter}};
    /// let filter = Filter::new();
    /// Hook::block(filter).then_iter(|mut iter| {
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
        let iter = Iter::new(self.filter.clone(), self.event_block);
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
}
