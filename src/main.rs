use crate::handler::HandlerError;
use crate::handler::handler;
use lambda::{handler_fn, Context};
use serde_json::Value;

mod poloniex;
mod handler;

#[tokio::main]
async fn main() -> Result<(), HandlerError> {
    lambda::run(handler_fn(handler)).await
}
