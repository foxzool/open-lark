use open_lark::{prelude::*, service::attendance::v1::models::DeleteShiftRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    env_logger::init();

    let app_id = std::env::var("APP_ID").expect("APP_ID is required");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET is required");

    // 创建客户端
    let client = LarkClient::builder(&app_id, &app_secret)
        .with_enable_token_cache(true)
        .build();

    println!("=== 删除班次接口示例 ===\n");

    // 从环境变量或命令行参数获取要删除的班次ID
    let shift_id = std::env::var("SHIFT_ID").unwrap_or_else(|_| {
        println!("⚠️ 未设置 SHIFT_ID 环境变量，使用默认测试ID");
        println!("   请设置 SHIFT_ID 环境变量为实际的班次ID");
        "test_shift_id".to_string()
    });

    println!("🗑️ 删除班次 ID: {}", shift_id);

    let request = DeleteShiftRequest::new(&shift_id);

    match client.attendance.v1.shift.delete(request, None).await {
        Ok(response) => {
            if response.success() {
                println!("✅ 班次删除成功!");
                println!("   响应代码: {}", response.code());
                println!("   响应消息: {}", response.msg());
            } else {
                println!("❌ 删除失败: {} - {}", response.code(), response.msg());
                if let Some(err) = response.err() {
                    println!("   错误详情: {:?}", err);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 请求失败: {:?}", e);
        }
    }

    println!("\n💡 使用说明:");
    println!("   1. 设置环境变量 SHIFT_ID 为要删除的班次ID");
    println!("   2. 运行: SHIFT_ID=your_shift_id cargo run --example attendance_shift_delete");
    println!("   3. 注意：删除操作不可逆，请谨慎操作!");

    println!("\n=== 示例结束 ===");
    Ok(())
}
