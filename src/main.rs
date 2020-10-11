use crate::handler::handler;
use crate::handler::HandlerError;
use lambda::handler_fn;

mod handler;
mod poloniex;

#[tokio::main]
async fn main() -> Result<(), HandlerError> {
    lambda::run(handler_fn(handler)).await
}
