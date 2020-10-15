use chrono::{DateTime, Utc};
use serde::{Deserialize};
use chrono::serde::ts_seconds;

ez_serde!(Meta {
   symbol: String,

   #[serde(with = "ts_seconds")]
   first_trade_date: DateTime<Utc>,

   #[serde(rename = "regularMarketPrice")]
   current_price: f32,

   #[serde(rename = "chartPreviousClose")]
   previous_close: f32
});

ez_serde!(OHLCV {
   #[serde(rename = "open", default)]
   opens: Vec<Option<f64>>,

   #[serde(rename = "high", default)]
   highs: Vec<Option<f64>>,

   #[serde(rename = "low", default)]
   lows: Vec<Option<f64>>,

   #[serde(rename = "close", default)]
   closes: Vec<Option<f64>>,

   #[serde(rename = "volume", default)]
   volumes: Vec<Option<u64>>
});

ez_serde!(Indicators { #[serde(rename = "quote", default)] quotes: Vec<OHLCV> });

ez_serde!(Data {
   meta: Meta,

   #[serde(rename = "timestamp", default)]
   timestamps: Vec<i64>,

   indicators: Indicators
});

ez_serde!(Error {code: String, description: String });
ez_serde!(Chart { result: Option<Vec<Data>>, error: Option<Error> });
ez_serde!(YahooResponse { chart: Chart });
