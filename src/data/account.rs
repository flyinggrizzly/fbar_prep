use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::data::provider::Provider;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountDefinition {
    pub handle: String,
    pub provider_handle: String,
    pub provider: Option<Provider>,
    pub currency_code: String,

    pub identifier1: String,
    pub identifier1_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier2_name: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub joint_holder_names: Vec<String>,

    pub opening_date: NaiveDate,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub closing_date: Option<NaiveDate>,
}

impl AccountDefinition {
    pub fn is_joint(&self) -> bool {
        !self.joint_holder_names.is_empty()
    }

    pub fn open_during(&self, year: i32) -> bool {
        let year_start = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
        let year_end = NaiveDate::from_ymd_opt(year, 12, 31).unwrap();

        self.opening_date <= year_end && self.closing_date.map_or(true, |date| date >= year_start)
    }

    pub fn open_on(&self, date: NaiveDate) -> bool {
        self.opening_date <= date
            && self
                .closing_date
                .map_or(true, |closing_date| closing_date >= date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_joint() {
        let mut account = AccountDefinition {
            handle: "test".to_string(),
            provider_handle: "test_provider".to_string(),
            provider: None,
            currency_code: "USD".to_string(),
            identifier1: "123".to_string(),
            identifier1_name: "account".to_string(),
            identifier2: None,
            identifier2_name: None,
            joint_holder_names: vec![],
            opening_date: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
            closing_date: None,
        };

        // Test single account
        assert!(!account.is_joint());

        // Test joint account
        account.joint_holder_names = vec!["Jane Doe".to_string()];
        assert!(account.is_joint());
    }

    #[test]
    fn test_open_during() {
        let mut account = AccountDefinition {
            handle: "test".to_string(),
            provider_handle: "test_provider".to_string(),
            provider: None,
            currency_code: "USD".to_string(),
            identifier1: "123".to_string(),
            identifier1_name: "account".to_string(),
            identifier2: None,
            identifier2_name: None,
            joint_holder_names: vec![],
            opening_date: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
            closing_date: None,
        };

        // Test still open account
        assert!(account.open_during(2023));
        assert!(account.open_during(2024));
        assert!(!account.open_during(2022));

        // Test closed account
        account.closing_date = Some(NaiveDate::from_ymd_opt(2024, 6, 30).unwrap());
        assert!(account.open_during(2023));
        assert!(account.open_during(2024));
        assert!(!account.open_during(2025));
        assert!(!account.open_during(2022));
    }

    #[test]
    fn test_open_on() {
        let mut account = AccountDefinition {
            handle: "test".to_string(),
            provider_handle: "test_provider".to_string(),
            provider: None,
            currency_code: "USD".to_string(),
            identifier1: "123".to_string(),
            identifier1_name: "account".to_string(),
            identifier2: None,
            identifier2_name: None,
            joint_holder_names: vec![],
            opening_date: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
            closing_date: None,
        };

        // Test still open account
        assert!(account.open_on(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap())); // Opening date
        assert!(account.open_on(NaiveDate::from_ymd_opt(2024, 6, 30).unwrap())); // Future date
        assert!(!account.open_on(NaiveDate::from_ymd_opt(2022, 12, 31).unwrap())); // Before opening

        // Test closed account
        account.closing_date = Some(NaiveDate::from_ymd_opt(2024, 6, 30).unwrap());
        assert!(account.open_on(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap())); // Opening date
        assert!(account.open_on(NaiveDate::from_ymd_opt(2024, 6, 30).unwrap())); // Closing date
        assert!(account.open_on(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())); // During open period
        assert!(!account.open_on(NaiveDate::from_ymd_opt(2024, 7, 1).unwrap())); // After closing
        assert!(!account.open_on(NaiveDate::from_ymd_opt(2022, 12, 31).unwrap()));
        // Before opening
    }
}
