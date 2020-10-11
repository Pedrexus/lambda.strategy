use lambda::{handler_fn, Context};
use serde_json::Value;

pub type HandlerError = Box<dyn std::error::Error + Sync + Send + 'static>;

pub async fn handler(event: Value, _: Context) -> Result<Value, HandlerError> {
    Ok(event)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn handler_handles() {
        let event = json!({
            "answer": 42
        });
        assert_eq!(
            handler(event.clone(), Context::default())
                .await
                .expect("expected Ok(_) value"),
            event
        )
    }
}
