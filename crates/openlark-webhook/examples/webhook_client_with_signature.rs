//! Webhook Client with Signature Example
//!
//! This example demonstrates how to use WebhookClient with signature verification.
//! Requires the "signature" feature to be enabled.
//!
//! Run with:
//! ```bash
//! cargo run --example webhook_client_with_signature -p openlark-webhook --features signature
//! ```

#[cfg(feature = "signature")]
use openlark_webhook::prelude::*;

#[cfg(feature = "signature")]
#[tokio::main]
async fn main() -> Result<()> {
    let webhook_url = "https://open.feishu.cn/open-apis/bot/v2/hook/YOUR_WEBHOOK_KEY".to_string();
    let secret = "YOUR_WEBHOOK_SECRET".to_string();

    // Create client with signature
    let client = WebhookClient::new().with_secret(secret);

    // Send text message
    let response = client
        .send_text(
            &webhook_url,
            "Hello from WebhookClient with signature!".to_string(),
        )
        .await?;

    println!("Message sent successfully using WebhookClient!");
    println!("Response code: {}", response.code);
    println!("Response message: {}", response.msg);

    // Send image message
    let image_response = client
        .send_image(&webhook_url, "img_abc123".to_string())
        .await?;

    println!("\nImage message sent!");
    println!("Response code: {}", image_response.code);

    Ok(())
}

#[cfg(not(feature = "signature"))]
fn main() {
    println!("This example requires the 'signature' feature to be enabled.");
    println!(
        "Run with: cargo run --example webhook_client_with_signature -p openlark-webhook --features signature"
    );
}
