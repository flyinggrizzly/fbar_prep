use super::{AccountDefinition, Provider, UserData};
use serde::de::{self, Deserializer, MapAccess, Visitor};
use std::fmt;

#[derive(serde::Deserialize)]
#[serde(field_identifier, rename_all = "snake_case")]
enum Field {
    Providers,
    FactExtensions,
    Accounts,
}

pub(super) struct UserDataVisitor;

impl<'de> Visitor<'de> for UserDataVisitor {
    type Value = UserData;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("struct UserData")
    }

    fn visit_map<V>(self, mut map: V) -> Result<UserData, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut providers: Option<Vec<Provider>> = None;
        let mut fact_extensions: Option<crate::facts::Facts> = None;
        let mut accounts: Option<Vec<AccountDefinition>> = None;

        while let Some(key) = map.next_key()? {
            match key {
                Field::Providers => {
                    providers = Some(map.next_value()?);
                }
                Field::FactExtensions => {
                    fact_extensions = Some(map.next_value()?);
                }
                Field::Accounts => {
                    // Here we can access providers that were already parsed
                    if providers.is_none() {
                        return Err(de::Error::custom(
                            "accounts must come after providers in YAML",
                        ));
                    }
                    let providers_ref = providers.as_ref().unwrap();

                    // Parse accounts and validate provider handles
                    let mut raw_accounts: Vec<AccountDefinition> = map.next_value()?;

                    // Create new accounts with provider references
                    for account in &mut raw_accounts {
                        let referenced_provider = providers_ref
                            .iter()
                            .find(|p| p.handle == account.provider_handle)
                            .cloned();

                        if referenced_provider.is_none() {
                            return Err(de::Error::custom(format!(
                                "account {} references unknown provider {}",
                                account.handle, account.provider_handle
                            )));
                        }

                        account.provider = referenced_provider;
                    }
                    accounts = Some(raw_accounts);
                }
            }
        }

        let providers = providers.ok_or_else(|| de::Error::missing_field("providers"))?;

        Ok(UserData {
            providers,
            fact_extensions,
            accounts,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn load_demo_data() -> UserData {
        let demo_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("demo_data");
        UserData::load_from_path(&demo_path).expect("Failed to load demo data")
    }

    #[test]
    fn test_demo_data_providers() {
        let data = load_demo_data();

        // Test that we have the expected providers
        assert_eq!(data.providers.len(), 2);

        // Test first provider
        let brit_bank = &data.providers[0];
        assert_eq!(brit_bank.name, "A Very British Bank");
        assert_eq!(brit_bank.handle, "britbank");
        assert_eq!(brit_bank.address, "123 Main St, Anytown, UK");

        // Test second provider
        let german_bank = &data.providers[1];
        assert_eq!(german_bank.name, "A Very German Bank");
        assert_eq!(german_bank.handle, "germanbank");
        assert_eq!(german_bank.address, "123 Main St, Anytown, Germany");
    }

    #[test]
    fn test_demo_data_accounts() {
        let data = load_demo_data();

        let accounts = data
            .accounts
            .as_ref()
            .expect("Expected accounts to be present");
        assert_eq!(accounts.len(), 3);

        // Test checking account
        let checking = &accounts[0];
        assert_eq!(checking.handle, "britbank_checking");
        assert_eq!(checking.provider_handle, "britbank");
        assert_eq!(checking.currency_code, "GBP");
        assert_eq!(checking.identifier1, "12345678");
        assert_eq!(checking.identifier1_name, "account_number");
        assert_eq!(checking.identifier2.as_ref().unwrap(), "12-34-56");
        assert_eq!(checking.identifier2_name.as_ref().unwrap(), "sort_code");
        assert_eq!(checking.joint_holder_names, vec!["John Doe", "Jane Doe"]);
        assert!(checking.closing_date.is_none());

        // Test savings account
        let savings = &accounts[1];
        assert_eq!(savings.handle, "britbank_savings");
        assert_eq!(savings.provider_handle, "britbank");
        assert_eq!(savings.currency_code, "GBP");
        assert_eq!(savings.identifier1, "87654321");
        assert_eq!(savings.identifier1_name, "account_number");
        assert_eq!(savings.identifier2.as_ref().unwrap(), "12-34-56");
        assert_eq!(savings.identifier2_name.as_ref().unwrap(), "sort_code");
        assert_eq!(savings.joint_holder_names, vec!["John Doe", "Jane Doe"]);
        assert!(savings.closing_date.is_some());

        // Test pension account
        let pension = &accounts[2];
        assert_eq!(pension.handle, "germanbank_pension");
        assert_eq!(pension.provider_handle, "germanbank");
        assert_eq!(pension.currency_code, "EUR");
        assert_eq!(pension.identifier1, "1234567890");
        assert_eq!(pension.identifier1_name, "policy_number");
        assert!(pension.identifier2.is_none());
        assert!(pension.identifier2_name.is_none());
        assert!(pension.joint_holder_names.is_empty());
        assert!(pension.closing_date.is_none());
    }

    #[test]
    fn test_account_provider_references() {
        let data = load_demo_data();
        let accounts = data
            .accounts
            .as_ref()
            .expect("Expected accounts to be present");

        // Test that each account's provider reference matches the actual provider
        for account in accounts {
            let provider = account
                .provider
                .as_ref()
                .expect("Expected provider to be set");
            let matching_provider = data
                .providers
                .iter()
                .find(|p| p.handle == account.provider_handle)
                .expect("Expected to find matching provider");

            assert_eq!(provider.handle, matching_provider.handle);
            assert_eq!(provider.name, matching_provider.name);
            assert_eq!(provider.address, matching_provider.address);
        }
    }
}
