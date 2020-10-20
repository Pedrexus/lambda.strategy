use crate::strategies::{Error, Order, Strategy};
use ta::{
    indicators::RelativeStrengthIndex as RelativeStrengthIndexIndicator, Next,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct RelativeStrengthIndex {
    rsi_indicator: RelativeStrengthIndexIndicator,
    lower_bound: f64,
    upper_bound: f64,
}

impl RelativeStrengthIndex {
    pub fn new(
        n: u32,
        lower_bound: f64,
        upper_bound: f64,
    ) -> Result<Self, Error> {
        let rsi_indicator = RelativeStrengthIndexIndicator::new(n)?;
        let rsi_strategy = Self {
            rsi_indicator,
            lower_bound,
            upper_bound,
        };

        Ok(rsi_strategy)
    }
}

impl Next<f64> for RelativeStrengthIndex {
    type Output = Order;

    fn next(&mut self, input: f64) -> Self::Output {
        match self.rsi_indicator.next(input) {
            rsi if rsi < self.lower_bound => Order::Buy, // oversold
            rsi if rsi > self.upper_bound => Order::Sell, // overbought
            _ => Order::Hold,
        }
    }
}

impl Strategy for RelativeStrengthIndex {}
