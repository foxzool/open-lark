use dotenvy::dotenv;
use open_lark::{prelude::LarkClient, service::attendance::v1::models::DeleteGroupRequest};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID must be set");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET must be set");

    let client = LarkClient::builder(&app_id, &app_secret).build();

    // 构建删除考勤组请求
    let mut req = DeleteGroupRequest::default();
    req.group_id = "group_id_to_delete".to_string(); // 要删除的考勤组ID

    println!("发送删除考勤组请求...");
    println!("考勤组ID: {}", req.group_id);

    match client.attendance.v1.group.delete(req, None).await {
        Ok(_) => {
            println!("✅ 删除考勤组成功!");
        }
        Err(e) => {
            eprintln!("❌ 删除考勤组失败: {:?}", e);
        }
    }

    Ok(())
}
