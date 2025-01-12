pub mod exchange_rate;
pub use self::exchange_rate::ExchangeRate;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Facts {
    pub years: Vec<AnnualFact>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnnualFact {
    pub year: i32,
    pub exchange_rates: Vec<ExchangeRate>,
}

impl Facts {
    pub fn load_facts() -> Result<Facts, Box<dyn std::error::Error>> {
        // Include the YAML file at compile time
        let yaml_content = include_str!("../../facts/years.yml");

        // Parse the YAML content
        let facts: Facts = serde_yaml::from_str(yaml_content)?;

        Ok(facts)
    }

    pub fn get_exchange_rate(
        &self,
        year: i32,
        currency_code: impl Into<String>,
    ) -> Option<&ExchangeRate> {
        let lookup_code = currency_code.into().to_lowercase();
        self.years
            .iter()
            .find(|annual_fact| annual_fact.year == year)
            .and_then(|annual_fact| {
                annual_fact
                    .exchange_rates
                    .iter()
                    .find(|rate| rate.currency_code == lookup_code)
            })
    }

    /// Creates an empty Facts instance with no exchange rates
    pub fn empty() -> Self {
        Facts { years: Vec::new() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_facts() {
        let facts = Facts::load_facts().unwrap();
        let years = &facts.years;

        // Verify we have the expected number of years
        assert!(!facts.years.is_empty());

        // Test the first year (2024)
        let year_2024 = &years[0];
        assert_eq!(year_2024.year, 2024);
        assert_eq!(year_2024.exchange_rates.len(), 2);

        // Test exchange rates for 2024
        let gbp = &year_2024.exchange_rates[0];
        assert_eq!(gbp.currency_code, "gbp");
        assert_eq!(gbp.rate, 0.783);

        let eur = &year_2024.exchange_rates[1];
        assert_eq!(eur.currency_code, "eur");
        assert_eq!(eur.rate, 0.924);
    }

    #[test]
    fn test_get_exchange_rate() {
        let facts = Facts::load_facts().unwrap();

        // Test existing exchange rate
        let gbp_rate = facts.get_exchange_rate(2024, "gbp").unwrap();
        assert_eq!(gbp_rate.currency_code, "gbp");
        assert_eq!(gbp_rate.rate, 0.783);

        // Test case insensitivity
        let gbp_upper = facts.get_exchange_rate(2024, "GBP").unwrap();
        let gbp_mixed = facts.get_exchange_rate(2024, "GbP").unwrap();
        assert_eq!(gbp_upper.rate, gbp_rate.rate);
        assert_eq!(gbp_mixed.rate, gbp_rate.rate);

        // Test non-existent year
        assert!(facts.get_exchange_rate(2000, "gbp").is_none());

        // Test non-existent currency
        assert!(facts.get_exchange_rate(2024, "xyz").is_none());
    }
}
