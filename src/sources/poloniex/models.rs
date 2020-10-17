use serde::{Deserialize, Serialize};
use market_finance::Bar;

ez_serde!(Candle {
    #[serde(rename = "timestamp", default)]
   date: u64,

   #[serde(rename = "open", default)]
   open: f64,

   #[serde(rename = "high", default)]
   high: f64,

   #[serde(rename = "low", default)]
   low: f64,

   #[serde(rename = "close", default)]
   close: f64,

   #[serde(rename = "volume", default)]
   volume: f64
});

impl From<Candle> for Bar {
    fn from(data: Candle) -> Bar {
        Bar {
            timestamp: data.date as i64,
            open: data.open,
            close: data.close,
            high: data.high,
            low: data.low,
            volume: Some(data.volume as u64),
        }
    }
}

// #[serde(rename = "quote_volume", default)]
// quote_volume: Option<f64>,
//
// #[serde(rename = "weighted_average", default)]
// weighted_average: Option<f64>,

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Chart {
    pub date: Vec<u64>,
    pub high: Vec<f64>,
    pub low: Vec<f64>,
    pub open: Vec<f64>,
    pub close: Vec<f64>,
    pub volume: Vec<f64>,
    // pub quote_volume: Vec<f64>,
    // pub weighted_average: Vec<f64>,
}

impl From<Vec<Candle>> for Chart {
    fn from(data: Vec<Candle>) -> Self {
        Chart {
            date: data.iter().map(|item| item.date).collect(),
            high: data.iter().map(|item| item.high).collect(),
            low: data.iter().map(|item| item.low).collect(),
            open: data.iter().map(|item| item.open).collect(),
            close: data.iter().map(|item| item.close).collect(),
            volume: data.iter().map(|item| item.volume).collect(),
            // quote_volume: data.iter().map(|item| item.quote_volume).collect(),
            // weighted_average: data
            //     .iter()
            //     .map(|item| item.weighted_average)
            //     .collect(),
        }
    }
}
