use crate::sources::error;
use crate::sources::models::Source::Discord;
use serde_json::json;
use snafu::ResultExt;

pub async fn notify_in_discord(msg: String) -> bool {
    let webhook_url = std::env::var("DISCORD_WEBHOOK_URL")
        .expect("DISCORD_WEBHOOK_URL env var not available");
    let client = reqwest::Client::new();

    let payload = json!({"content": msg, "tts": false});

    client
        .post(&webhook_url)
        .json(&payload)
        .send()
        .await
        .context(error::RequestFailed { api: Discord })
        .is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    // integration test
    // #[tokio::test]
    // async fn test_discord() {
    //     assert!(notify_in_discord("testing...".to_string()).await);
    // }
}
