use crate::notifications::notify_in_discord;
use crate::sources::return_chart_data;
use crate::strategies::{Order, RelativeStrengthIndex, Strategy};
use serde_json::{json, Value};

pub type HandlerError = Box<dyn std::error::Error + Sync + Send + 'static>;

// TODO: this should be fetch from dynamodb
pub async fn handler(
    _event: Value,
    _: lambda::Context,
) -> Result<String, HandlerError> {
    let event: Value = json!([
        // B3
        {
            "source": "Yahoo",
            "symbol": "PETR4.SA",
            "strategy": "RSI",
        },
        {
            "source": "Yahoo",
            "symbol": "MGLU3.SA",
            "strategy": "RSI",
        },
        {
            "source": "Yahoo",
            "symbol": "AZUL4.SA",
            "strategy": "RSI",
        },
        {
            "source": "Yahoo",
            "symbol": "GGBR4.SA",
            "strategy": "RSI",
        },
        {
            "source": "Yahoo",
            "symbol": "B3SA3.SA",
            "strategy": "RSI",
        },
        {
            "source": "Yahoo",
            "symbol": "VALE3.SA",
            "strategy": "RSI",
        },
        // Crypto
        {
            "source": "Poloniex",
            "symbol": "USDT_BTC",
            "strategy": "RSI",
        },
        {
            "source": "Poloniex",
            "symbol": "USDT_ETH",
            "strategy": "RSI",
        },
        {
            "source": "Poloniex",
            "symbol": "USDT_DASH",
            "strategy": "RSI",
        },
        {
            "source": "Poloniex",
            "symbol": "USDT_BULL",
            "strategy": "RSI",
        },
        {
            "source": "Poloniex",
            "symbol": "BTC_ETH",
            "strategy": "RSI",
        },
        {
            "source": "Poloniex",
            "symbol": "BTC_XMR",
            "strategy": "RSI",
        },
        {
            "source": "Poloniex",
            "symbol": "BTC_XRP",
            "strategy": "RSI",
        },
        {
            "source": "Poloniex",
            "symbol": "BTC_LTC",
            "strategy": "RSI",
        },
    ]);

    let mut complete_message = String::new();

    for row in event.as_array().unwrap().iter() {
        let chart =
            return_chart_data(&row["source"], &row["symbol"], &row["settings"])
                .await
                .expect("chart didn't returned");

        let mut strategy = RelativeStrengthIndex::new(14, 20., 80.).unwrap();
        // let mut strategy = Strategy::from(row["strategy"], row["parameters"]);
        let analysis = strategy.evaluate(chart);

        let symb = &row["symbol"].as_str().unwrap();

        let msg = match analysis.last().unwrap() {
            Order::Buy => format!("time to buy {}!\n", symb),
            Order::Sell => format!("time to sell {}!\n", symb),
            Order::Hold => String::new(), // format!("hold {}", symb),
        };

        complete_message.push_str(msg.as_str());

        // notification
        notify_in_discord(msg).await;
    }

    // hyperparam: take
    // if strategy_analysis.iter().rev().take(3).iter().any(|x| x is Buy) => Buy,

    Ok(complete_message)
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda::Context;
    use serde_json::json;

    #[tokio::test]
    async fn handler_handles() -> Result<(), HandlerError> {
        let event = json!([]);

        let response = handler(event.clone(), Context::default()).await?;

        // PETR4.SA is Hold => no msg | Only Sell/Buy orders are sent to user
        let expected = json!({ "message": "time to buy BTC_XMR!" });

        assert_eq!(response, expected);

        Ok(())
    }
}
//
// {
//     "Records": [
//         {
//             "eventID": "7de3041dd709b024af6f29e4fa13d34c",
//             "eventName": "INSERT",
//             "eventVersion": "1.1",
//             "eventSource": "aws:dynamodb",
//             "awsRegion": "us-west-2",
//             "dynamodb": {
//                 "ApproximateCreationDateTime": 1479499740,
//                 "Keys": {
//                     "Timestamp": {
//                         "S": "2016-11-18:12:09:36"
//                     },
//                     "Username": {
//                         "S": "John Doe"
//                     }
//                 },
//                 "SequenceNumber": "13021600000000001596893679",
//                 "SizeBytes": 112,
//                 "StreamViewType": "KEYS_ONLY"
//             },
//             "eventSourceARN": "arn:aws:dynamodb:us-east-1:123456789012:table/BarkTable/stream/2016-11-16T20:42:48.104"
//         }
//     ]
// }
