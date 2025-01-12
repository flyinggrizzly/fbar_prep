pub mod converter;
pub use self::converter::{Converter, RateSource};
use anyhow::{bail, Result};

use crate::facts::Facts;

pub struct ReportContext {
    facts: Facts,
    extensions: Facts,
}

impl ReportContext {
    pub fn new(facts: Facts, extensions: impl Into<Option<Facts>>) -> Self {
        Self {
            facts,
            extensions: extensions.into().unwrap_or_else(Facts::empty),
        }
    }

    /// Converts an amount from a source currency to USD for a specific year
    ///
    /// # Arguments
    /// * `year` - The year for which to perform the conversion
    /// * `source_currency` - The currency code to convert from (e.g., "EUR", "CHF")
    /// * `amount` - The amount in the source currency
    ///
    /// # Returns
    /// * `Result<f64, anyhow::Error>` - The converted amount in USD
    pub fn convert_to_usd(&self, year: i32, source_currency: &str, amount: f64) -> Result<f64> {
        self.find_exchange_rate(year, source_currency)
            .map(|rate| rate.convert_to_usd(amount))
    }

    /// Converts an amount from USD to a target currency for a specific year
    ///
    /// # Arguments
    /// * `year` - The year for which to perform the conversion
    /// * `target_currency` - The currency code to convert to (e.g., "EUR", "CHF")
    /// * `amount` - The amount in USD
    ///
    /// # Returns
    /// * `Result<f64, anyhow::Error>` - The converted amount in the target currency
    pub fn convert_from_usd(&self, year: i32, target_currency: &str, amount: f64) -> Result<f64> {
        self.find_exchange_rate(year, target_currency)
            .map(|rate| rate.convert_from_usd(amount))
    }

    // Helper method to find the appropriate exchange rate
    fn find_exchange_rate(&self, year: i32, currency_code: &str) -> Result<Converter> {
        let lookup_code = currency_code.to_lowercase();

        // First check extensions, then fall back to facts
        if let Some(rate) = self
            .extensions
            .get_exchange_rate(year, lookup_code.to_string())
        {
            Ok(Converter::new(rate.clone(), RateSource::UserProvided))
        } else if let Some(rate) = self.facts.get_exchange_rate(year, lookup_code.clone()) {
            Ok(Converter::new(rate.clone(), RateSource::IrsProvided))
        } else {
            bail!(
                "No exchange rate found for {} in year {}",
                currency_code,
                year
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::facts::{AnnualFact, ExchangeRate, Facts};

    fn create_test_facts() -> Facts {
        Facts {
            years: vec![AnnualFact {
                year: 2023,
                exchange_rates: vec![
                    ExchangeRate::new("EUR".to_string(), 0.85).unwrap(),
                    ExchangeRate::new("CHF".to_string(), 0.90).unwrap(),
                ],
            }],
        }
    }

    fn create_test_fact_extensions() -> Facts {
        Facts {
            years: vec![AnnualFact {
                year: 2023,
                exchange_rates: vec![
                    ExchangeRate::new("EUR".to_string(), 0.80).unwrap(),
                    // CHF is not present in the extensions to test that the IRS rates are used
                ],
            }],
        }
    }

    #[test]
    fn test_conversion() -> Result<()> {
        let facts = create_test_facts();
        let extensions = create_test_fact_extensions();
        let context = ReportContext::new(facts, extensions);

        // Test EUR conversion
        assert_eq!(context.convert_to_usd(2023, "EUR", 85.0)?, 106.25);
        assert_eq!(context.convert_from_usd(2023, "EUR", 100.0)?, 80.0);

        // Test CHF conversion
        assert_eq!(context.convert_to_usd(2023, "CHF", 90.0)?, 100.0);
        assert_eq!(context.convert_from_usd(2023, "CHF", 100.0)?, 90.0);

        Ok(())
    }

    #[test]
    fn test_invalid_currency() {
        let facts = create_test_facts();
        let extensions = create_test_fact_extensions();
        let context = ReportContext::new(facts, extensions);

        let result = context.convert_to_usd(2023, "INVALID", 100.0);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("No exchange rate found"));
    }

    #[test]
    fn test_invalid_year() {
        let facts = create_test_facts();
        let extensions = create_test_facts();
        let context = ReportContext::new(facts, extensions);

        let result = context.convert_to_usd(1999, "EUR", 100.0);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("No exchange rate found"));
    }

    #[test]
    fn test_rate_source() -> Result<()> {
        let facts = create_test_facts();
        let context = ReportContext::new(facts.clone(), None);

        // Test that IRS rates come from IRS source
        let rate = context.find_exchange_rate(2023, "EUR")?;
        assert_eq!(rate.source(), &RateSource::IrsProvided);

        // Test that user rates come from user source
        let user_facts = create_test_facts();
        let context = ReportContext::new(facts, Some(user_facts));
        let rate = context.find_exchange_rate(2023, "EUR")?;
        assert_eq!(rate.source(), &RateSource::UserProvided);

        Ok(())
    }
}
