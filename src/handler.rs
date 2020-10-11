use lambda::Context;
use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::poloniex;

pub type HandlerError = Box<dyn std::error::Error + Sync + Send + 'static>;

pub async fn handler(event: Value, _: Context) -> Result<Value, HandlerError> {
    let currency_pair = event["currency_pair"].as_str().unwrap();
    let period = event.get("period").unwrap().as_u64().unwrap();

    let end = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let start = end - 86400;

    let response = poloniex::return_chart_data(&currency_pair, period, start, end).await;

    let value = serde_json::to_value(response).unwrap();
    Ok(value)
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use serde_json::json;

    // #[tokio::test]
    // async fn handler_handles() {
    //     let event = json!({
    //         "answer": 42
    //     });
    //     assert_eq!(
    //         handler(event.clone(), Context::default())
    //             .await
    //             .expect("expected Ok(_) value"),
    //         event
    //     )
    // }
}
