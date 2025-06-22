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

    println!("📨 Lark IM Message Example");
    println!("Note: This is a basic structure example");
    println!("To send actual messages, you need:");
    println!("- Valid chat_id or user_id");
    println!("- Proper message content");
    println!("- Required permissions");
    
    println!("\n📋 Available IM operations:");
    println!("- client.im.v1.message - Message operations");
    println!("- client.im.v1.chat - Chat operations");
    println!("- client.im.v2.message - V2 message operations");
    
    println!("\n💡 Tip: Check official API docs for detailed parameters");

    Ok(())
}