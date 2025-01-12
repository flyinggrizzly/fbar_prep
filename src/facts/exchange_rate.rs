use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Serialize, Clone)]
pub struct ExchangeRate {
    pub currency_code: String,
    pub rate: f64,
}

impl ExchangeRate {
    /// Creates a new ExchangeRate with validation
    ///
    /// # Arguments
    /// * `currency_code` - The currency code (e.g., "EUR", "JPY")
    /// * `rate` - The exchange rate (must be positive)
    ///
    /// # Returns
    /// * `Result<ExchangeRate, String>` - Ok with the new ExchangeRate if valid, Err with error message if invalid
    pub fn new(currency_code: String, rate: f64) -> Result<Self, String> {
        if rate <= 0.0 {
            return Err("Exchange rate must be greater than 0".to_string());
        }
        Ok(Self {
            currency_code: currency_code.to_lowercase(),
            rate,
        })
    }

    /// Converts an amount from USD to the target currency
    pub fn convert_from_usd(&self, amount: f64) -> f64 {
        let result = amount * self.rate;
        (result * 100.0).round() / 100.0
    }

    /// Converts an amount from the target currency to USD
    pub fn convert_to_usd(&self, amount: f64) -> f64 {
        let result = amount / self.rate;
        (result * 100.0).round() / 100.0
    }
}

// Custom deserialize implementation
impl<'de> Deserialize<'de> for ExchangeRate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawExchangeRate {
            currency_code: String,
            rate: f64,
        }

        let raw = RawExchangeRate::deserialize(deserializer)?;
        ExchangeRate::new(raw.currency_code, raw.rate).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency_conversion() {
        let rate = ExchangeRate {
            currency_code: "EUR".to_string(),
            rate: 0.85, // Example rate: 1 USD = 0.85 EUR
        };

        // Test USD to EUR conversion
        assert_eq!(rate.convert_from_usd(100.0), 85.0); // 100 USD = 85 EUR
        assert_eq!(rate.convert_from_usd(50.0), 42.5); // 50 USD = 42.5 EUR

        // Test EUR to USD conversion
        assert_eq!(rate.convert_to_usd(85.0), 100.0); // 85 EUR = 100 USD
        assert_eq!(rate.convert_to_usd(42.5), 50.0); // 42.5 EUR = 50 USD
    }

    #[test]
    fn test_rounding() {
        let rate = ExchangeRate {
            currency_code: "EUR".to_string(),
            rate: 0.333, // Rate: 1 USD = 0.333 EUR
        };

        // Test rounding to 2 decimal places
        assert_eq!(rate.convert_from_usd(1.00), 0.33); // 1 USD = 0.33 EUR
        assert_eq!(rate.convert_from_usd(10.00), 3.33); // 10 USD = 3.33 EUR

        // Test EUR to USD conversion with rounding
        assert_eq!(rate.convert_to_usd(0.33), 0.99); // 0.33 EUR ≈ 0.99 USD
        assert_eq!(rate.convert_to_usd(3.33), 10.00); // 3.33 EUR = 10.00 USD
    }

    #[test]
    fn test_currency_code_case() {
        // Test constructor
        let rate1 = ExchangeRate::new("EUR".to_string(), 0.85).unwrap();
        assert_eq!(rate1.currency_code, "eur");

        let rate2 = ExchangeRate::new("eur".to_string(), 0.85).unwrap();
        assert_eq!(rate2.currency_code, "eur");

        let rate3 = ExchangeRate::new("eUr".to_string(), 0.85).unwrap();
        assert_eq!(rate3.currency_code, "eur");
    }
}
