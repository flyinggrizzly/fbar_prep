mod account;
mod provider;
mod user_data_deserializer;

pub use account::AccountDefinition;
pub use provider::Provider;

use crate::facts::Facts;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize)]
pub struct UserData {
    pub providers: Vec<Provider>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fact_extensions: Option<Facts>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<AccountDefinition>>,
}

impl<'de> Deserialize<'de> for UserData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_struct(
            "UserData",
            &["providers", "fact_extensions", "accounts"],
            user_data_deserializer::UserDataVisitor,
        )
    }
}

impl UserData {
    pub fn load_from_path(base_path: &Path) -> Result<Self> {
        let yaml_path = base_path.join("data.yml");

        if !yaml_path.exists() {
            anyhow::bail!("data.yml not found in {:?}", base_path);
        }

        let contents = std::fs::read_to_string(yaml_path)?;
        let data: UserData = serde_yaml::from_str(&contents)?;
        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    // ... existing tests ...
}
