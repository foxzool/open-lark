//! Webhook Text Message Example
//!
//! This example demonstrates how to send a simple text message using the webhook bot.
//!
//! Run with:
//! ```bash
//! cargo run --example webhook_text_message -p openlark-webhook
//! ```

use openlark_webhook::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Replace with your actual webhook URL
    let webhook_url = "https://open.feishu.cn/open-apis/bot/v2/hook/YOUR_WEBHOOK_KEY".to_string();

    // Create and send a text message using the builder pattern
    let response = SendWebhookMessageRequest::new(webhook_url)
        .text("Hello from OpenLark Webhook! 🚀".to_string())
        .execute()
        .await?;

    println!("Message sent successfully!");
    println!("Response code: {}", response.code);
    println!("Response message: {}", response.msg);

    Ok(())
}
