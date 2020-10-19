use crate::handler::{handler, HandlerError};
use lambda::handler_fn;

mod handler;
// mod models;
mod aws;
mod sources;
mod strategies;

#[tokio::main]
async fn main() -> Result<(), HandlerError> {
    lambda::run(handler_fn(handler)).await
}
