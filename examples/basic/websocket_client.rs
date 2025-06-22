use open_lark::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("APP_ID environment variable not set");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET environment variable not set");

    // 创建客户端
    let _client = LarkClient::builder(&app_id, &app_secret)
        .with_enable_token_cache(true)
        .build();

    println!("🔌 WebSocket Client Example");
    println!("Note: WebSocket features require the 'websocket' feature flag");
    
    #[cfg(feature = "websocket")]
    {
        println!("✅ WebSocket feature is enabled");
        println!("🎯 You can now use real-time event handling");
        println!("📡 Example event types:");
        println!("- Message received events");
        println!("- Chat member changes");
        println!("- Document updates");
    }
    
    #[cfg(not(feature = "websocket"))]
    {
        println!("❌ WebSocket feature is not enabled");
        println!("💡 Enable with: cargo run --example websocket_client --features websocket");
    }

    println!("\n🔧 To enable WebSocket support:");
    println!("1. Add 'websocket' to your Cargo.toml features");
    println!("2. Use EventDispatcherHandler for event handling");
    println!("3. Register event handlers for different event types");

    Ok(())
}