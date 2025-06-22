use open_lark::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    
    let app_id = std::env::var("APP_ID").expect("APP_ID environment variable not set");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET environment variable not set");
    
    let client = LarkClient::builder(&app_id, &app_secret)
        .with_enable_token_cache(true)
        .build();
    
    println!("✅ 云文档服务重构测试");
    
    // 测试新的云文档聚合API
    println!("📁 云文档聚合服务可用:");
    println!("  - client.cloud_docs.drive: {:?}", &client.cloud_docs.drive as *const _);
    println!("  - client.cloud_docs.wiki: {:?}", &client.cloud_docs.wiki as *const _);
    println!("  - client.cloud_docs.docx: {:?}", &client.cloud_docs.docx as *const _);
    println!("  - client.cloud_docs.sheets: {:?}", &client.cloud_docs.sheets as *const _);
    println!("  - client.cloud_docs.bitable: {:?}", &client.cloud_docs.bitable as *const _);
    println!("  - client.cloud_docs.board: {:?}", &client.cloud_docs.board as *const _);
    println!("  - client.cloud_docs.permission: {:?}", &client.cloud_docs.permission as *const _);
    println!("  - client.cloud_docs.comments: {:?}", &client.cloud_docs.comments as *const _);
    println!("  - client.cloud_docs.assistant: {:?}", &client.cloud_docs.assistant as *const _);
    
    // 测试向后兼容的API
    println!("\n🔄 向后兼容API仍可用:");
    println!("  - client.drive: {:?}", &client.drive as *const _);
    println!("  - client.docs: {:?}", &client.docs as *const _);
    println!("  - client.sheets: {:?}", &client.sheets as *const _);
    println!("  - client.bitable: {:?}", &client.bitable as *const _);
    
    // 测试核心服务
    println!("\n🚀 核心服务:");
    println!("  - client.im: {:?}", &client.im as *const _);
    println!("  - client.attendance: {:?}", &client.attendance as *const _);
    println!("  - client.auth: {:?}", &client.auth as *const _);
    println!("  - client.search: {:?}", &client.search as *const _);
    
    println!("\n✅ 所有服务模块重构成功！");
    
    Ok(())
}