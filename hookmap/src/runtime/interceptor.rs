//! Gets keyboard events dynamically.

pub use super::event_broker::Filter;

use hookmap_core::{event::ButtonEvent, hook::NativeEventOperation};

use super::event_broker::EventBroker;

use std::sync::Arc;
use std::sync::Mutex;

use once_cell::sync::Lazy;

static BROKER: Lazy<Mutex<EventBroker>> = Lazy::new(Mutex::default);

pub(super) fn publish_event(event: ButtonEvent) -> NativeEventOperation {
    BROKER.lock().unwrap().publish(event)
}

/// Set the hook that receives input events;
///
/// # Examples
///
/// ```no_run
/// use hookmap::prelude::*;
///
/// let filter = Filter::new().action(ButtonAction::Press);
/// let event = Interceptor::dispatch(filter).get();
/// println!("{:?}, {:?}", event.target, event.action);
/// ```
///
pub struct Interceptor {
    filter: Arc<Filter>,
    native_event_operation: NativeEventOperation,
}

impl Interceptor {
    /// Creates a new instance of [`Interceptor`].
    /// Captured events are blocked.
    ///
    /// ```no_run
    /// use hookmap::prelude::*;
    ///
    /// let filter = Filter::new();
    /// let event = Interceptor::blocking(filter).get();
    /// println!("This event is blocked: {:?}", event);
    /// ```
    ///
    pub fn blocking(filter: Filter) -> Self {
        Self {
            filter: Arc::new(filter),
            native_event_operation: NativeEventOperation::Block,
        }
    }

    /// Creates a new instance of [`Interceptor`].
    /// Captured events are not blocked.
    ///
    /// ```no_run
    /// use hookmap::prelude::*;
    ///
    /// let filter = Filter::new();
    /// let event = Interceptor::dispatch(filter).get();
    /// println!("This event is blocked: {:?}", event);
    /// ```
    ///
    pub fn dispatch(filter: Filter) -> Self {
        Self {
            filter: Arc::new(filter),
            native_event_operation: NativeEventOperation::Dispatch,
        }
    }

    /// Captures a single event.
    ///
    /// ```no_run
    /// use hookmap::prelude::*;
    ///
    /// let filter = Filter::new();
    /// let event = Interceptor::dispatch(filter).get();
    /// println!("{:?}", event);
    /// ```
    pub fn get(&self) -> ButtonEvent {
        let rx = BROKER
            .lock()
            .unwrap()
            .subscribe_once(Arc::clone(&self.filter), self.native_event_operation);

        rx.recv().unwrap()
    }

    /// Captures events with an iterator.
    ///
    /// ```no_run
    /// use hookmap::prelude::*;
    ///
    /// let filter = Filter::new();
    /// let keys: Vec<Button> = Interceptor::blocking(filter)
    ///     .iter()
    ///     .filter(|e| e.action == ButtonAction::Press)
    ///     .take(3)
    ///     .map(|e| e.target)
    ///     .collect();
    /// ```
    ///
    pub fn iter(&self) -> Iter {
        Iter {
            filter: Arc::clone(&self.filter),
            native_event_operation: self.native_event_operation,
        }
    }
}

pub struct Iter {
    filter: Arc<Filter>,
    native_event_operation: NativeEventOperation,
}

impl Iterator for Iter {
    type Item = ButtonEvent;

    fn next(&mut self) -> Option<ButtonEvent> {
        let rx = BROKER
            .lock()
            .unwrap()
            .subscribe_once(Arc::clone(&self.filter), self.native_event_operation);

        rx.recv().ok()
    }
}
