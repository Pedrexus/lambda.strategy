use crate::exchange::yahoo::period::{ChartRange, CandlestickInterval};

pub struct Settings {
    range: ChartRange,
    interval: CandlestickInterval,
}

pub struct Investment {
    exchange: String,
    symbol: String,
    parameters: Option<String>,
    options: Option<Settings>
}