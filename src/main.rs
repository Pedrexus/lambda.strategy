use crate::exchange::yahoo::period::{CandlestickInterval, ChartRange};
use crate::exchange::yahoo::return_chart_data;
use crate::handler::{handler, HandlerError};
use lambda::handler_fn;
use market_finance::Timestamped;

mod exchange;
mod handler;
mod strategies;

#[tokio::main]
async fn main() -> Result<(), HandlerError> {
    lambda::run(handler_fn(handler)).await
}

// #[tokio::main]
// async fn main() {
//     match return_chart_data("PETR4.SA", ChartRange::_5d, CandlestickInterval::_30m).await {
//         Err(e) => println!("Failed to call Yahoo {:?}", e),
//         Ok(data) =>
//             for bar in &data {
//                 println!("On {} Apple closed at ${:.2}", bar.datetime().format("%b %e %Y"), bar.close)
//             }
//     }
// }
