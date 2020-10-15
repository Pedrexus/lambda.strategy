use crate::exchange::poloniex;
use crate::strategies::{Order, RelativeStrengthIndex, Strategy};
use serde_json::{json, Value};

pub type HandlerError = Box<dyn std::error::Error + Sync + Send + 'static>;

pub async fn handler(event: Value, _: lambda::Context) -> Result<Value, HandlerError> {
    let currency_pair = event["currency_pair"].as_str().unwrap();
    let period = event["period"].as_u64().unwrap();
    let start = event["start"].as_u64().unwrap();
    let end = event["end"].as_u64().unwrap();

    let chart = poloniex::return_chart_data(currency_pair, period, start, end).await?;

    let mut strategy = RelativeStrengthIndex::new(14, 30., 70.).unwrap();
    let analysis = strategy.evaluate(chart.close);

    let msg = match analysis.last().unwrap() {
        Order::Buy => format!("time to buy {}!", currency_pair),
        Order::Sell => format!("time to sell {}!", currency_pair),
        Order::Hold => format!("time to sell {}!", currency_pair),
    };

    // hyperparam: take
    // if strategy_analysis.iter().rev().take(3).iter().any(|x| x is Buy) => Buy,

    Ok(json!({ "msg": msg }))
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

        assert_eq!(response, json!({ "msg": "time to buy BTC_XMR!" }));
    }
}
