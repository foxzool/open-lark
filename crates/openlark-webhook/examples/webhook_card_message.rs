//! Webhook Card Message Example
//!
//! This example demonstrates how to send an interactive card message using the webhook bot.
//! Requires the "card" feature to be enabled.
//!
//! Run with:
//! ```bash
//! cargo run --example webhook_card_message -p openlark-webhook --features card
//! ```

#[cfg(feature = "card")]
use openlark_webhook::prelude::*;

#[cfg(feature = "card")]
#[tokio::main]
async fn main() -> Result<()> {
    let webhook_url = "https://open.feishu.cn/open-apis/bot/v2/hook/YOUR_WEBHOOK_KEY".to_string();

    let card = serde_json::json!({
        "type": "template",
        "data": {
            "template_id": "AAqkweugf8sLk",
            "template_variable": {
                "title": "OpenLark Webhook Card",
                "description": "This is an interactive card message"
            }
        }
    });

    let response = SendWebhookMessageRequest::new(webhook_url)
        .card(card)
        .execute()
        .await?;

    println!("Card message sent successfully!");
    println!("Response code: {}", response.code);
    println!("Response message: {}", response.msg);

    Ok(())
}

#[cfg(not(feature = "card"))]
fn main() {
    println!("This example requires the 'card' feature to be enabled.");
    println!(
        "Run with: cargo run --example webhook_card_message -p openlark-webhook --features card"
    );
}
