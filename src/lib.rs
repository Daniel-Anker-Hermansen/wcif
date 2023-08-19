use serde::{de::DeserializeOwned, Serialize, Deserialize};

use std::fmt::Debug;

use wcif::Date;

pub mod wcif;

pub trait ScopeTypes {
    type Email: DeserializeOwned + Serialize + Debug + Eq + PartialEq;

    type DateOfBirth: DeserializeOwned + Serialize + Debug + Eq + PartialEq;

    type Guests: DeserializeOwned + Serialize + Debug + Eq + PartialEq;
    
    type Comments: DeserializeOwned + Serialize + Debug + Eq + PartialEq;
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Enabled;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Disabled;

impl ScopeTypes for Enabled {
    type Email = String;

    type DateOfBirth = Date;

    type Guests = u32;

    type Comments = String;
}

impl ScopeTypes for Disabled {
    type Email = Unavailable;

    type DateOfBirth = Unavailable;

    type Guests = Unavailable;

    type Comments = Unavailable;
}

#[derive(Debug, PartialEq, Eq)]
pub struct Unavailable;

impl<'de> Deserialize<'de> for Unavailable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        // Serialize whatever value might be there and throw it away.
        Option::<serde_json::Value>::deserialize(deserializer).map(|_| Unavailable)
    }
}

impl Serialize for Unavailable {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        // Serializing none in json leads to the field being removed which is what we
        // want.
        serializer.serialize_none()
    }
}
