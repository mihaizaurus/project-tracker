use core::fmt;

use ulid::Ulid;

use crate::EntityType;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Id<T> {
    ulid: Ulid,
    marker: std::marker::PhantomData<T>
}

impl<T: EntityType> fmt::Display for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", T::prefix(), self.ulid)
    }
}

impl<T: EntityType> fmt::Debug for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", T::prefix(), self.ulid)
    }
}

impl<T: EntityType> Id<T> {
    pub fn new() -> Self {
        Self {
            ulid: Ulid::new(),
            marker: std::marker::PhantomData
        }
    }
}