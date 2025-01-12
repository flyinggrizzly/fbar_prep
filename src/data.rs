use crate::facts::Facts;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub providers: Vec<Provider>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fact_extensions: Option<Facts>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Provider {
    pub name: String,
    pub handle: String,
    pub address: String,
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
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_yaml(dir: &Path) -> std::io::Result<()> {
        let yaml_content = r#"
providers:
  - name: "Example Bank"
    handle: "example_bank"
    address: "123 Bank Street, Zurich, Switzerland"
  - name: "Another Bank"
    handle: "another_bank"
    address: "456 Finance Ave, Frankfurt, Germany"

fact_extensions:
  years:
    - year: 2023
      exchange_rates:
        - currency_code: "CHF"
          rate: 1.08
        - currency_code: "EUR"
          rate: 0.91
    - year: 2022
      exchange_rates:
        - currency_code: "CHF"
          rate: 1.07
        - currency_code: "EUR"
          rate: 0.92
"#;
        fs::write(dir.join("data.yml"), yaml_content)
    }

    #[test]
    fn test_load_valid_data() -> Result<()> {
        // Create a temporary directory that will be automatically cleaned up
        let temp_dir = TempDir::new()?;
        create_test_yaml(temp_dir.path())?;

        // Load and validate the data
        let data = UserData::load_from_path(temp_dir.path())?;

        // Verify providers
        assert_eq!(data.providers.len(), 2);
        assert_eq!(data.providers[0].name, "Example Bank");
        assert_eq!(data.providers[0].handle, "example_bank");
        assert_eq!(
            data.providers[0].address,
            "123 Bank Street, Zurich, Switzerland"
        );

        // Verify user_fact_overrides
        let fact_extensions = data.fact_extensions.as_ref().unwrap();
        assert_eq!(fact_extensions.years.len(), 2);

        let year_2023 = &fact_extensions.years[0];
        assert_eq!(year_2023.year, 2023);
        assert_eq!(year_2023.exchange_rates[0].currency_code, "chf");
        assert_eq!(year_2023.exchange_rates[0].rate, 1.08);
        assert_eq!(year_2023.exchange_rates[1].currency_code, "eur");
        assert_eq!(year_2023.exchange_rates[1].rate, 0.91);

        let year_2022 = &fact_extensions.years[1];
        assert_eq!(year_2022.year, 2022);
        assert_eq!(year_2022.exchange_rates[0].currency_code, "chf");
        assert_eq!(year_2022.exchange_rates[0].rate, 1.07);
        assert_eq!(year_2022.exchange_rates[1].currency_code, "eur");
        assert_eq!(year_2022.exchange_rates[1].rate, 0.92);

        Ok(())
    }

    #[test]
    fn test_missing_yaml() {
        // Create an empty temp directory
        let temp_dir = TempDir::new().unwrap();

        // Attempt to load from directory with no yaml file
        let result = UserData::load_from_path(temp_dir.path());

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("data.yml not found"));
    }

    #[test]
    fn test_invalid_yaml() -> std::io::Result<()> {
        let temp_dir = TempDir::new()?;

        // Create invalid YAML file
        fs::write(
            temp_dir.path().join("data.yml"),
            "invalid: yaml: content: - [",
        )?;

        let result = UserData::load_from_path(temp_dir.path());
        assert!(result.is_err());

        Ok(())
    }
}
