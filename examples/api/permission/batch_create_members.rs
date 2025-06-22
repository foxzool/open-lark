use dotenv::dotenv;
use open_lark::prelude::*;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID environment variable not set");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET environment variable not set");

    let client = LarkClient::builder(&app_id, &app_secret).build();

    // 示例：批量为文档添加协作者
    let request = BatchCreatePermissionMemberRequest::builder()
        .token("doccnxxxxxxxxxxxxxx") // 替换为实际的文档token
        .as_doc()
        .add_user("user_id_1", Permission::Edit) // 添加编辑者
        .add_user("user_id_2", Permission::View) // 添加阅读者
        .add_chat("chat_id_1", Permission::Comment) // 添加群组评论者
        .with_notification() // 通知用户
        .build();

    match client.permission.batch_create_member(&request, None).await {
        Ok(response) => {
            println!("批量添加协作者权限成功!");

            if let Some(data) = response.data {
                println!("操作摘要: {}", data.summary());

                // 显示成功的操作
                for member in data.successful_members() {
                    println!(
                        "✅ 成功: {} ({}) - {}",
                        member.member_id,
                        member.member_type,
                        member.perm.description()
                    );
                }

                // 显示失败的操作
                for member in data.failed_members() {
                    println!(
                        "❌ 失败: {} ({}) - {}",
                        member.member_id,
                        member.member_type,
                        member
                            .error_message()
                            .unwrap_or_else(|| "未知错误".to_string())
                    );
                }

                // 统计信息
                println!("\n统计信息:");
                println!("- 总计操作: {}", data.members.len());
                println!("- 成功数量: {}", data.success_count());
                println!("- 失败数量: {}", data.failed_count());
            }
        }
        Err(e) => {
            eprintln!("批量添加协作者权限失败: {:?}", e);
        }
    }

    Ok(())
}
