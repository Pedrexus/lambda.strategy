use reqwest::Url;
use snafu::{ensure, OptionExt, ResultExt};

use crate::sources::models::Source::Yahoo;
use crate::sources::yahoo::models::{Data, YahooResponse};
use crate::sources::yahoo::period::{CandlestickInterval, ChartRange};
use crate::sources::{error, error::Result};
use market_finance::Bar;

const YAHOO_BASE_URL: &str =
    "https://query1.finance.yahoo.com/v8/finance/chart/";

/// Helper function to build up the main query URL
fn build_query(symbol: &str) -> Result<Url> {
    Ok(Url::parse(YAHOO_BASE_URL)
        .context(error::InternalURL {
            url: YAHOO_BASE_URL,
        })?
        .join(symbol)
        .context(error::InternalURL { url: symbol })?)
}

async fn load(url: &Url) -> Result<Data> {
    // make the call - we do not really expect this to fail.
    // ie - we won't 404 if the symbol doesn't exist
    let response = reqwest::get(url.clone())
        .await
        .context(error::RequestFailed { api: Yahoo })?;
    ensure!(
        response.status().is_success(),
        error::CallFailed {
            api: Yahoo,
            url: response.url().to_string(),
            status: response.status().as_u16()
        }
    );

    let data = response.text().await.context(error::UnexpectedErrorRead {
        url: url.to_string(),
    })?;
    let chart = serde_json::from_str::<YahooResponse>(&data)
        .context(error::BadData { api: Yahoo })?
        .chart;

    if chart.result.is_none() {
        // no result so we'd better have an error
        let err = chart.error.context(error::InternalLogic {
            reason: "error block exists without values",
        })?;
        error::ChartFailed {
            api: Yahoo,
            code: err.code,
            description: err.description,
        }
        .fail()?;
    }

    // we have a result to process
    let result = chart
        .result
        .context(error::UnexpectedError { api: Yahoo })?;
    ensure!(!result.is_empty(), error::UnexpectedError { api: Yahoo });
    Ok(result[0].clone())
}

fn aggregate_bars(data: Data) -> Result<Vec<Bar>> {
    let mut result = Vec::new();

    let timestamps = &data.timestamps;
    let quotes = &data.indicators.quotes;

    // if we have no timestamps & no quotes we'll assume there is no data
    if timestamps.is_empty() && quotes.is_empty() {
        return Ok(result);
    }

    // otherwise see if one is empty and reflects bad data from Yahoo!
    ensure!(
        !timestamps.is_empty(),
        error::MissingData {
            api: Yahoo,
            reason: "no timestamps for OHLCV data"
        }
    );
    ensure!(
        !quotes.is_empty(),
        error::MissingData {
            api: Yahoo,
            reason: "no OHLCV data"
        }
    );

    // make sure timestamps lines up with the OHLCV data
    let quote = &quotes[0];
    ensure!(
        timestamps.len() == quote.volumes.len(),
        error::MissingData {
            api: Yahoo,
            reason: "timestamps do not line up with OHLCV data"
        }
    );
    ensure!(
        timestamps.len() == quote.opens.len(),
        error::MissingData {
            api: Yahoo,
            reason: "'open' values do not line up the timestamps"
        }
    );
    ensure!(
        timestamps.len() == quote.highs.len(),
        error::MissingData {
            api: Yahoo,
            reason: "'high' values do not line up the timestamps"
        }
    );
    ensure!(
        timestamps.len() == quote.lows.len(),
        error::MissingData {
            api: Yahoo,
            reason: "'low' values do not line up the timestamps"
        }
    );
    ensure!(
        timestamps.len() == quote.closes.len(),
        error::MissingData {
            api: Yahoo,
            reason: "'close' values do not line up the timestamps"
        }
    );

    #[allow(clippy::needless_range_loop)]
    for i in 0..timestamps.len() {
        // skip days where we have incomplete data
        if quote.opens[i].is_none()
            || quote.highs[i].is_none()
            || quote.lows[i].is_none()
            || quote.closes[i].is_none()
        {
            continue;
        }

        result.push(Bar {
            timestamp: timestamps[i] * 1000,
            open: quote.opens[i].context(error::InternalLogic {
                reason: "missing open not caught",
            })?,
            high: quote.highs[i].context(error::InternalLogic {
                reason: "missing high not caught",
            })?,
            low: quote.lows[i].context(error::InternalLogic {
                reason: "missing low not caught",
            })?,
            close: quote.closes[i].context(error::InternalLogic {
                reason: "missing close not caught",
            })?,
            volume: quote.volumes[i],
        })
    }
    Ok(result)
}

pub async fn return_chart_data(
    symbol: &str,
    range: ChartRange,
    interval: CandlestickInterval,
) -> Result<Vec<Bar>> {
    let mut lookup = build_query(symbol)?;
    lookup
        .query_pairs_mut()
        .append_pair("range", &range.to_string())
        .append_pair("interval", &interval.to_string());

    let data = load(&lookup).await?;

    aggregate_bars(data)
}
