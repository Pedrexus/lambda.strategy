use crate::handler::{handler, HandlerError};
use lambda::handler_fn;

mod exchange;
mod handler;
mod strategies;

#[tokio::main]
async fn main() -> Result<(), HandlerError> {
    lambda::run(handler_fn(handler)).await
}
