use crate::handler::{handler, HandlerError};
use lambda::handler_fn;

mod handler;
// mod models;
mod sources;
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
