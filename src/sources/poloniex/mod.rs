mod models;

use super::error;
use super::error::Result;
use crate::sources::models::Source::Poloniex;
use crate::sources::poloniex::models::Candle;
use crate::sources::yahoo::period::{CandlestickInterval, ChartRange};
use chrono::Utc;
use market_finance::Bar;
use snafu::{ensure, ResultExt};
use url::Url;

pub async fn return_chart_data(
    symbol: &str,
    range: ChartRange,
    interval: CandlestickInterval,
) -> Result<Vec<Bar>> {
    let period = match interval {
        CandlestickInterval::_5m => 300,   // 5min
        CandlestickInterval::_15m => 900,  // 15min
        CandlestickInterval::_30m => 1800, // 30min
        CandlestickInterval::_60m => 3600, // 1hr
        CandlestickInterval::_90m => 7200, // 2hrs
        // CandlestickInterval::?? => 14400 // 4hrs
        CandlestickInterval::_1d => 86400, // 1day
        _ => panic!("poloniex does not accept interval of {}", interval),
    };

    let start = range.as_datetime().timestamp();
    let end = Utc::now().timestamp();

    let data = load(symbol, period, start as u64, end as u64).await?;

    Ok(data)
}

const BASE_URL: &str = "https://poloniex.com/public";

pub async fn load(
    currency_pair: &str,
    period: u64,
    start: u64,
    end: u64,
) -> Result<Vec<Bar>> {
    let mut url = Url::parse(BASE_URL)
        .context(error::InternalURL { url: BASE_URL })
        .unwrap();
    url.query_pairs_mut()
        .append_pair("command", "returnChartData")
        .append_pair("currencyPair", currency_pair)
        .append_pair("period", &period.to_string())
        .append_pair("start", &start.to_string())
        .append_pair("end", &end.to_string());

    let response = reqwest::get(url)
        .await
        .context(error::RequestFailed { api: Poloniex })
        .unwrap();

    let response_url = response.url().to_string();

    ensure!(
        response.status().is_success(),
        error::CallFailed {
            api: Poloniex,
            url: &response_url,
            status: response.status().as_u16()
        }
    );

    let data = response
        .text()
        .await
        .context(error::UnexpectedErrorRead { url: &response_url })?;
    let chart = serde_json::from_str::<Vec<Candle>>(&data)
        .context(error::BadData { api: Poloniex })?;

    Ok(chart.into_iter().map(Bar::from).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    // todo
}
