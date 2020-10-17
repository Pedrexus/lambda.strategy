use crate::sources::return_chart_data;
use crate::strategies::{Order, RelativeStrengthIndex, Strategy};
use serde_json::{json, Value};

pub type HandlerError = Box<dyn std::error::Error + Sync + Send + 'static>;

pub async fn handler(
    event: Value,
    _: lambda::Context,
) -> Result<Value, HandlerError> {
    let mut messages = Vec::new();
    for row in event.as_array().unwrap().into_iter() {
        let chart =
            return_chart_data(&row["source"], &row["symbol"], &row["settings"])
                .await
                .expect("chart not returned");

        let mut strategy = RelativeStrengthIndex::new(14, 20., 80.).unwrap();
        // let mut strategy = Strategy::from(row["strategy"], row["parameters"]);
        let analysis = strategy.evaluate(chart);

        let symb = &row["symbol"].as_str().unwrap();
        let msg = match analysis.last().unwrap() {
            Order::Buy => format!("time to buy {}!", symb),
            Order::Sell => format!("time to sell {}!", symb),
            Order::Hold => format!("hold {}", symb),
        };

        messages.push(msg);
    }

    // hyperparam: take
    // if strategy_analysis.iter().rev().take(3).iter().any(|x| x is Buy) => Buy,

    Ok(json!({ "messages": messages }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda::Context;
    use serde_json::json;

    #[tokio::test]
    async fn handler_handles() -> Result<(), HandlerError> {
        let event = json!([
            {
                "source": "Yahoo",
                "symbol": "PETR4.SA",
                "strategy": "RSI",
                "parameters": {
                    "window": 14,
                    "lower bound": 30,
                    "upper bound": 70,
                },
                "settings": {
                    "range": "6mo",
                    "interval": "1d"
                }
            },
            {
                "source": "Poloniex",
                "symbol": "BTC_XMR",
                "strategy": "RSI",
                "parameters": {
                    "window": 14,
                    "lower bound": 30,
                    "upper bound": 70,
                }
            },
        ]);

        let response = handler(event.clone(), Context::default()).await?;

        // PETR4.SA is Hold => no msg | Only Sell/Buy orders are sent to user
        let expected = json!({ "message": "time to buy BTC_XMR!" });

        assert_eq!(response, expected);

        Ok(())
    }
}
