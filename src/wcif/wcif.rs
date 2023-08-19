use std::ops::{Deref, DerefMut};

use super::Competition;

use crate::{Disabled, Enabled};

pub struct PublicWcif {
    pub(crate) inner: Competition<Disabled>,
}

impl PublicWcif {
    pub fn from_string(wcif: &str) -> Result<PublicWcif, serde_json::Error> {
        serde_json::from_str(wcif).map(|inner| PublicWcif { inner })
    }
}

impl Deref for PublicWcif {
    type Target = Competition<Disabled>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for PublicWcif {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

pub struct PrivateWcif {
    pub(crate) inner: Competition<Enabled>,
}

impl PrivateWcif {
    pub fn from_string(wcif: &str) -> Result<PrivateWcif, serde_json::Error> {
        serde_json::from_str(wcif).map(|inner| PrivateWcif { inner })
    }
}

impl Deref for PrivateWcif {
    type Target = Competition<Enabled>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for PrivateWcif {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
