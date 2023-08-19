use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Extension<T> {
    pub id: String,
    pub spec_url: String,
    pub data: T
}

impl<T> Extension<T> where T: Serialize {
    /// Converts an extension into a generic JSON extension which can be stored in the wcif.
    /// Returns Err if T contains maps with non-string keys or if the T serialization
    /// implementation errors.
    pub fn into_value(self) -> serde_json::Result<Extension<Value>> {
        serde_json::to_value(self.data).map(|value| Extension { 
            id: self.id, 
            spec_url: self.spec_url, 
            data: value 
        })
    }
}

impl Extension<Value> {
    /// Converts a generic value extension into a specific extension
    /// Returns Err if the T deserialization errors.
    pub fn into_generic<T>(self) -> serde_json::Result<Extension<T>> where T: DeserializeOwned {
        serde_json::from_value(self.data).map(|value| Extension { 
            id: self.id, 
            spec_url: self.spec_url, 
            data: value 
        })
    }
}
