use std::sync::Arc;

use crate::runtime::hook::{OptionalProcedure, RequiredProcedure};

impl<E, F: Fn(E) + Send + Sync + 'static> From<F> for RequiredProcedure<E> {
    fn from(f: F) -> Self {
        RequiredProcedure(Arc::new(f))
    }
}

impl<E, F: Fn(Option<E>) + Send + Sync + 'static> From<F> for OptionalProcedure<E> {
    fn from(f: F) -> Self {
        OptionalProcedure(Arc::new(f))
    }
}
