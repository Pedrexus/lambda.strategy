use crate::exchange::Candle;
use crate::exchange::Chart;
use reqwest::get;

pub async fn return_chart_data(
    currency_pair: &str,
    period: u64,
    start: u64,
    end: u64,
) -> Result<Chart, reqwest::Error> {
    let request_url = format!(
            "https://poloniex.com/public?command={command}&currencyPair={currency_pair}&start={start}&end={end}&period={period}",
            command = "returnChartData",
            currency_pair = currency_pair,
            start = start,
            end = end,
            period = period
        );

    let response = get(&request_url).await?;

    if response.status().as_u16() >= 400 {
        panic!(response.text())
    }

    let body: Vec<Candle> = response.json().await?;

    Ok(Chart::from(body))
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
            period = 14400
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

        let chart = return_chart_data(&currency_pair, period, start, end)
            .await
            .unwrap();

        assert_eq!(chart.high.len(), 25);

        // let item = &chart[0];

        // let expected = Candle {
        //     date: 1546300800,
        //     high: 0.01232199,
        //     low: 0.012105,
        //     open: 0.01227412,
        //     close: 0.01224702,
        //     volume: 11.47474031,
        //     quote_volume: 938.52999477,
        //     weighted_average: 0.01222629,
        // };

        // assert_eq!(item, &expected);
    }
}
