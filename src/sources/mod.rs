// make sure our macros are all loaded
#[macro_use]
mod macros;

mod error;
pub mod models;
mod poloniex;
mod yahoo;

use crate::sources::yahoo::period::{CandlestickInterval, ChartRange};
use crate::sources::yahoo::Bar;
use error::Result;
use models::Source;
use serde_json::Value;

const DEFAULT_RANGE: ChartRange = ChartRange::_5d;
const DEFAULT_INTERVAL: CandlestickInterval = CandlestickInterval::_30m;

pub async fn return_chart_data(
    source: &Value,
    symbol: &Value,
    settings: &Value,
) -> Result<Vec<Bar>> {
    let src: Source = source.as_str().unwrap().parse().unwrap();
    let symbol = symbol.as_str().unwrap();
    // is it a problem to limit one source for the other?
    let range: ChartRange = settings["range"]
        .as_str()
        .unwrap_or(DEFAULT_RANGE.to_string().as_str())
        .parse()
        .unwrap();
    let interval: CandlestickInterval = settings["interval"]
        .as_str()
        .unwrap_or(DEFAULT_INTERVAL.to_string().as_str())
        .parse()
        .unwrap();

    match src {
        Source::Yahoo => yahoo::return_chart_data(symbol, range, interval).await,
        Source::Poloniex => poloniex::return_chart_data(symbol, range, interval).await,
    }
}
