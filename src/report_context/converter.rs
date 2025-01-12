use crate::facts::ExchangeRate;
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq)]
pub enum RateSource {
    UserProvided,
    IrsProvided,
}

pub struct Converter {
    exchange_rate: ExchangeRate,
    source: RateSource,
}

/***
 * Decorator around the ExchangeRate struct that adds a source to the rate.
 *
 * Expectation is that the source can be used to generate auditable logs for the report, indicating why and how values were
 * derived when performing conversions.
 *
 * Helpful since users will be able to provide their own exchange rates, and these rates may be different
 * from the ones provided by the IRS.
 *
 * If in the future the tool is extended to support e.g. realtime conversion rates, the source can be used to indicate
 * the source, time, request ID etc of the conversion rate.
 */
impl Converter {
    pub fn new(exchange_rate: ExchangeRate, source: RateSource) -> Self {
        Self {
            exchange_rate,
            source,
        }
    }

    pub fn source(&self) -> &RateSource {
        &self.source
    }
}

impl Deref for Converter {
    type Target = ExchangeRate;

    fn deref(&self) -> &Self::Target {
        &self.exchange_rate
    }
}
