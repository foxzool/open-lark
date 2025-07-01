use open_lark::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenvy::dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("APP_ID environment variable not set");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET environment variable not set");

    // 创建客户端
    let _client = LarkClient::builder(&app_id, &app_secret)
        .with_enable_token_cache(true)
        .build();

    println!("✅ Lark client created successfully!");
    println!("📱 App ID: {app_id}");
    println!("🔧 Token cache: enabled");

    println!("\n🚀 Available services:");
    println!("- IM (instant messaging)");
    println!("- Drive (file management)");
    println!("- Docs (document operations)");
    println!("- Sheets (spreadsheet operations)");
    println!("- Wiki (knowledge base)");
    println!("- Bitable (multi-dimensional tables)");
    println!("- Comments (document comments)");
    println!("- Permission (access control)");
    println!("- Search (content search)");

    Ok(())
}
