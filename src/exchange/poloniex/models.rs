use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Candle {
    pub date: u64,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    pub close: f64,
    pub volume: f64,
    pub quote_volume: f64,
    pub weighted_average: f64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Chart {
    pub date: Vec<u64>,
    pub high: Vec<f64>,
    pub low: Vec<f64>,
    pub open: Vec<f64>,
    pub close: Vec<f64>,
    pub volume: Vec<f64>,
    pub quote_volume: Vec<f64>,
    pub weighted_average: Vec<f64>,
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
            quote_volume: data.iter().map(|item| item.quote_volume).collect(),
            weighted_average: data
                .iter()
                .map(|item| item.weighted_average)
                .collect(),
        }
    }
}
