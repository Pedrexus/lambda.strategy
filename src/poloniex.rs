use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone)]
enum CandlestickPeriod {
    _5Min = 5 * 60,
    _15Min = 15 * 60,
    _30Min = 30 * 60,
    _2Hrs = 2 * 3600,
    _4Hrs = 4 * 3600,
    _1Day = 24 * 3600,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Candle {
    date: usize,
    high: f64,
    low: f64,
    open: f64,
    close: f64,
    volume: f64,
    quote_volume: f64,
    weighted_average: f64,
}

pub async fn return_chart_data(
    currency_pair: &str,
    period: u64,
    start: u64,
    end: u64,
) -> Vec<Candle> {
    let request_url = format!(
            "https://poloniex.com/public?command={command}&currencyPair={currency_pair}&start={start}&end={end}&period={period}",
            command = "returnChartData",
            currency_pair = currency_pair,
            start = start,
            end = end,
            period = period
        );

    let response = reqwest::get(&request_url).await.expect("get response");

    if response.status().as_u16() >= 400 {
        panic!(response.text())
    }

    response.json().await.expect("get json")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_url() {
        let request_url = format!(
            "https://poloniex.com/public?command={command}&currencyPair={currency_pair}&start={start}&end={end}&period={period}",
            command = "returnChartData",
            currency_pair = "BTC_XMR",
            start = 1546300800,
            end = 1546646400,
            period = CandlestickPeriod::_4Hrs as usize
        );

        assert_eq!(request_url, "https://poloniex.com/public?command=returnChartData&currencyPair=BTC_XMR&start=1546300800&end=1546646400&period=14400");
    }

    #[tokio::test]
    async fn test_get() {
        let url = "https://poloniex.com/public?command=returnChartData&currencyPair=BTC_XMR&start=1546300800&end=1546646400&period=14400";
        let response: Vec<Candle> = reqwest::get(url)
            .await
            .expect("get response")
            .json()
            .await
            .expect("get json");
        assert_eq!(response.len(), 25);
    }

    #[tokio::test]
    async fn test_return_chart_data() {
        let currency_pair = "BTC_XMR";
        let period = 14400;
        let start = 1546300800;
        let end = 1546646400;

        let data = return_chart_data(&currency_pair, period, start, end).await;

        assert_eq!(data.len(), 25);

        let item = &data[0];

        let expected = Candle {
            date: 1546300800,
            high: 0.01232199,
            low: 0.012105,
            open: 0.01227412,
            close: 0.01224702,
            volume: 11.47474031,
            quote_volume: 938.52999477,
            weighted_average: 0.01222629,
        };

        assert_eq!(item, &expected);
    }
}
