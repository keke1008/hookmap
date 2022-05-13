use hookmap_core::button::{Button, ButtonAction};
use hookmap_core::event::{ButtonEvent, NativeEventOperation};

use std::sync::mpsc::{self, Receiver, SyncSender};
use std::{collections::HashSet, fmt::Debug, sync::Arc};

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
        self.action.map_or(true, |action| action == event.action)
            && match self.target {
                Some(Target::Single(button)) => event.target == button,
                Some(Target::Multiple(ref buttons)) => buttons.contains(&event.target),
                None => true,
            }
            && self.callback.iter().all(|callback| callback.0(event))
    }
}

#[derive(Debug)]
struct EventSender {
    tx: SyncSender<ButtonEvent>,
    filter: Arc<Filter>,
}

#[derive(Debug, Default)]
pub(super) struct EventBroker {
    dispatch: Vec<EventSender>,
    block: Vec<EventSender>,
}

impl EventBroker {
    pub(super) fn subscribe_once(
        &mut self,
        filter: Arc<Filter>,
        operation: NativeEventOperation,
    ) -> Receiver<ButtonEvent> {
        let (tx, rx) = mpsc::sync_channel(1);
        let event_sender = EventSender { tx, filter };

        match operation {
            NativeEventOperation::Block => self.block.push(event_sender),
            NativeEventOperation::Dispatch => self.dispatch.push(event_sender),
        }

        rx
    }

    pub(super) fn publish(&mut self, event: ButtonEvent) -> NativeEventOperation {
        if !self.block.is_empty() {
            let satisfied_index = self
                .block
                .iter()
                .rposition(|EventSender { filter, .. }| filter.filter(&event));
            if let Some(index) = satisfied_index {
                let EventSender { tx, .. } = self.block.remove(index);
                tx.send(event).unwrap();
                return NativeEventOperation::Block;
            }
        }

        // drain_filter (https://doc.rust-lang.org/std/vec/struct.Vec.html#method.drain_filter)
        let mut i = 0;
        while i < self.dispatch.len() {
            if self.dispatch[i].filter.filter(&event) {
                self.dispatch.remove(i).tx.send(event).unwrap();
            } else {
                i += 1;
            }
        }

        NativeEventOperation::Dispatch
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hookmap_core::button::{Button, ButtonAction};

    fn create_button_event(target: Button, action: ButtonAction) -> ButtonEvent {
        ButtonEvent {
            target,
            action,
            injected: false,
        }
    }

    #[test]
    fn event_sender_sends_block_events() {
        let mut broker = EventBroker::default();
        let filter = Arc::new(Filter::new());
        let rx = broker.subscribe_once(filter, NativeEventOperation::Block);

        let event = create_button_event(Button::A, ButtonAction::Press);
        broker.publish(event);
        assert_eq!(event, rx.recv().unwrap());
    }

    #[test]
    fn event_sender_does_not_send_block_events() {
        let mut broker = EventBroker::default();
        let filter = Filter::new().target(Button::A);
        let rx = broker.subscribe_once(Arc::new(filter), NativeEventOperation::Block);

        let event = create_button_event(Button::B, ButtonAction::Press);
        broker.publish(event);
        assert!(rx.try_recv().is_err());
    }

    #[test]
    fn event_sender_does_not_send_unblock_events() {
        let mut broker = EventBroker::default();
        let filter = Arc::new(Filter::new());

        let rx_dispatch =
            broker.subscribe_once(Arc::clone(&filter), NativeEventOperation::Dispatch);
        let rx_block = broker.subscribe_once(filter, NativeEventOperation::Block);

        let event = create_button_event(Button::A, ButtonAction::Press);
        broker.publish(event);

        assert!(rx_dispatch.try_recv().is_err());
        assert_eq!(rx_block.recv().unwrap(), event);

        let event = create_button_event(Button::B, ButtonAction::Press);
        broker.publish(event);
        assert_eq!(rx_dispatch.recv().unwrap(), event);
        assert!(rx_block.try_recv().is_err());
    }

    #[test]
    fn event_sender_sends_the_same_event_to_all_unblock_event_receiver() {
        let mut broker = EventBroker::default();
        let filter = Arc::new(Filter::new());

        let rx1 = broker.subscribe_once(Arc::clone(&filter), NativeEventOperation::Dispatch);
        let rx2 = broker.subscribe_once(filter, NativeEventOperation::Dispatch);

        let event = create_button_event(Button::C, ButtonAction::Release);
        broker.publish(event);

        assert_eq!(rx1.recv().unwrap(), event);
        assert_eq!(rx2.recv().unwrap(), event);
    }

    fn test_filter(expect: bool, filter: &Filter, target: Button, action: ButtonAction) {
        let event = create_button_event(target, action);
        assert_eq!(expect, filter.filter(&event));
    }

    #[test]
    fn filtering_events_by_target_matching_conditions() {
        let filter = Filter::new().target(Button::A);
        test_filter(true, &filter, Button::A, ButtonAction::Press);
        test_filter(true, &filter, Button::A, ButtonAction::Release);
    }

    #[test]
    fn filtering_events_by_target_not_matching_conditions() {
        let filter = Filter::new().target(Button::A);
        test_filter(false, &filter, Button::B, ButtonAction::Press);
        test_filter(false, &filter, Button::B, ButtonAction::Release);
    }

    #[test]
    fn filtering_events_by_targets_matching_conditions() {
        let targets = [Button::A, Button::B].iter().cloned().collect();
        let filter = Filter::new().targets(targets);
        test_filter(true, &filter, Button::A, ButtonAction::Press);
        test_filter(true, &filter, Button::A, ButtonAction::Release);
        test_filter(true, &filter, Button::B, ButtonAction::Release);
    }

    #[test]
    fn filtering_events_by_targets_not_matching_conditions() {
        let targets = [Button::A, Button::B].iter().cloned().collect();
        let filter = Filter::new().targets(targets);
        test_filter(false, &filter, Button::C, ButtonAction::Press);
        test_filter(false, &filter, Button::C, ButtonAction::Release);
    }

    #[test]
    fn filtering_events_by_action() {
        let filter = Filter::new().action(ButtonAction::Press);
        test_filter(true, &filter, Button::A, ButtonAction::Press);
        test_filter(false, &filter, Button::A, ButtonAction::Release);

        let filter = Filter::new().action(ButtonAction::Release);
        test_filter(false, &filter, Button::A, ButtonAction::Press);
        test_filter(true, &filter, Button::A, ButtonAction::Release);
    }

    #[test]
    fn filtering_events_by_target_and_action() {
        let filter = Filter::new().target(Button::A).action(ButtonAction::Press);
        test_filter(true, &filter, Button::A, ButtonAction::Press);
        test_filter(false, &filter, Button::A, ButtonAction::Release);
        test_filter(false, &filter, Button::B, ButtonAction::Press);
        test_filter(false, &filter, Button::B, ButtonAction::Release);
    }

    #[test]
    fn filtering_events_by_targets_and_action() {
        let targets = [Button::A, Button::B].iter().cloned().collect();
        let filter = Filter::new().targets(targets).action(ButtonAction::Press);
        test_filter(true, &filter, Button::A, ButtonAction::Press);
        test_filter(true, &filter, Button::B, ButtonAction::Press);
        test_filter(false, &filter, Button::B, ButtonAction::Release);
        test_filter(false, &filter, Button::C, ButtonAction::Press);
    }

    #[test]
    fn filtering_events_by_callback() {
        let filter = Filter::new().callback(|e| e.action == ButtonAction::Press);
        test_filter(true, &filter, Button::A, ButtonAction::Press);
        test_filter(false, &filter, Button::A, ButtonAction::Release);
    }
}
