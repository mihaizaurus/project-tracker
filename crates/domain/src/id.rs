use core::fmt;
use std::marker::PhantomData;

use ulid::Ulid;
use serde::Serialize;

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

impl<T: EntityType> std::str::FromStr for Id<T> {
    type Err = ParseIdError; // to be improved

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        let string_parts: Vec<&str> = id.split("-").collect();
        let prefix = string_parts[0];
        let ulid_part = string_parts[1];

        if prefix != T::prefix() {
            return Err(ParseIdError::WrongPrefix)
        }

        let ulid = ulid_part.parse::<Ulid>().map_err(|_| ParseIdError::InvalidUlid)?;

        Ok(Id {
            ulid,
            marker: PhantomData
        })
    }
}

#[derive(Debug, Serialize)]
pub enum ParseIdError {
    InvalidFormat,
    WrongPrefix,
    InvalidUlid,
}