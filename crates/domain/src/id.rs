use core::fmt;
use std::marker::PhantomData;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use ulid::Ulid;

use crate::EntityType;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Id<T> {
    ulid: Ulid,
    marker: std::marker::PhantomData<T>,
}

impl<T: EntityType> fmt::Display for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", T::prefix(), self.ulid)
    }
}

impl<T: EntityType> Id<T> {
    pub fn new() -> Self {
        Self {
            ulid: Ulid::new(),
            marker: std::marker::PhantomData,
        }
    }
}

impl<T: EntityType> Default for Id<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: EntityType> std::str::FromStr for Id<T> {
    type Err = ParseIdError; // to be improved

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        let string_parts: Vec<&str> = id.split("-").collect();
        let prefix = string_parts[0];
        let ulid_part = string_parts[1];

        if prefix != T::prefix() {
            return Err(ParseIdError::WrongPrefix);
        }

        let ulid = ulid_part
            .parse::<Ulid>()
            .map_err(|_| ParseIdError::InvalidUlid)?;

        Ok(Id {
            ulid,
            marker: PhantomData,
        })
    }
}

#[derive(Debug, Serialize)]
pub enum ParseIdError {
    InvalidFormat,
    WrongPrefix,
    InvalidUlid,
}

impl<T: EntityType> Serialize for Id<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de, T: EntityType> Deserialize<'de> for Id<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse::<Id<T>>()
            .map_err(|e| serde::de::Error::custom(format!("Invalid ID format: {e:?}")))
    }
}

