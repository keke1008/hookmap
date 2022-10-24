use std::fmt::{Debug, Formatter};

pub struct RequiredProcedure<E>(Box<dyn Fn(E)>);
pub struct OptionalProcedure<E>(Box<dyn Fn(Option<E>)>);

impl<E> Debug for RequiredProcedure<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RequiredProcedure").finish_non_exhaustive()
    }
}
impl<E> Debug for OptionalProcedure<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OptionalProcedure").finish_non_exhaustive()
    }
}

#[derive(Debug)]
pub enum Procedure<E> {
    Required(RequiredProcedure<E>),
    Optional(OptionalProcedure<E>),
}

impl<E> Procedure<E> {
    pub fn call(&self, event: E) {
        match self {
            Self::Required(proc) => proc.0(event),
            Self::Optional(proc) => proc.0(Some(event)),
        }
    }

    pub fn call_optional(&self, event: Option<E>) {
        match self {
            Self::Required(_) => {
                panic!("Attempt to call `Procedure::Required` with optional event.");
            }
            Self::Optional(proc) => proc.0(event),
        }
    }
}

impl<E, F: Fn(E) + 'static> From<F> for RequiredProcedure<E> {
    fn from(f: F) -> Self {
        RequiredProcedure(Box::new(f))
    }
}
impl<E, F: Fn(Option<E>) + 'static> From<F> for OptionalProcedure<E> {
    fn from(f: F) -> Self {
        OptionalProcedure(Box::new(f))
    }
}
