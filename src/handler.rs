use lambda::Context;
use serde_json::Value;

use crate::poloniex;

pub type HandlerError = Box<dyn std::error::Error + Sync + Send + 'static>;

pub async fn handler(event: Value, _: Context) -> Result<Value, HandlerError> {
    let currency_pair = event["currency_pair"].as_str().unwrap();
    let period = event["period"].as_u64().unwrap();
    let start = event["start"].as_u64().unwrap();
    let end = event["end"].as_u64().unwrap();

    let response = poloniex::return_chart_data(currency_pair, period, start, end).await;
    let value = serde_json::to_value(response).unwrap();

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn handler_handles() {
        let event = json!({
            "currency_pair": "BTC_XMR",
            "period": 14400,
            "start": 1602366808,
            "end": 1602453273,
        });

        let response = handler(event.clone(), Context::default())
            .await
            .expect("expected Ok(_) value");

        assert_eq!(response.as_array().unwrap().len(), 7);
        assert_eq!(response[0]["high"].as_f64().unwrap(), 0.01025603);
    }
}
