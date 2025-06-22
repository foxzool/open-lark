use dotenvy::dotenv;
use open_lark::{prelude::*, service::permission::member::TransferOwnerRequest};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID environment variable not set");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET environment variable not set");

    let client = LarkClient::builder(&app_id, &app_secret).build();

    // 示例1：转移文档所有权给用户（保留原所有者权限）
    println!("🔹 示例1: 转移文档所有权给用户（保留原所有者权限）");
    let request = TransferOwnerRequest::builder()
        .token("doccnxxxxxxxxxxxxxx") // 替换为实际的文档token
        .as_doc()
        .to_user("new_owner_user_id") // 替换为实际的新所有者用户ID
        .keep_current_owner() // 保留当前所有者的编辑权限
        .with_notification()
        .build();

    match client.permission.transfer_owner(&request, None).await {
        Ok(response) => {
            println!("✅ 所有权转移成功!");

            if let Some(data) = response.data {
                println!("{}", data.success_summary());

                let member = &data.member;
                println!("📋 转移详情:");
                println!("  新所有者: {} ({})", member.member_id, member.member_type);

                if let Some(old_info) = member.old_owner_info() {
                    println!("  {}", old_info);
                }

                if let Some(time) = member.transfer_time_formatted() {
                    println!("  🕒 {}", time);
                }

                println!("  ✅ 原所有者保留编辑权限");
                println!("  📧 已通知相关用户");
            }
        }
        Err(e) => {
            eprintln!("❌ 所有权转移失败: {:?}", e);
        }
    }

    println!("{}", "\n".to_string() + &"=".repeat(50));

    // 示例2：完全转移所有权（移除原所有者权限）
    println!("🔹 示例2: 完全转移所有权（移除原所有者权限）");
    let request = TransferOwnerRequest::to_user(
        "doccnxxxxxxxxxxxxxx", // 替换为实际的文档token
        "doc",
        "new_owner_user_id", // 替换为实际的新所有者用户ID
    );

    // 使用builder进一步配置
    let request = TransferOwnerRequest::builder()
        .token(request.token)
        .obj_type(request.obj_type)
        .new_owner(request.member_type, request.member_id)
        .remove_current_owner() // 完全移除当前所有者权限
        .without_notification() // 静默转移
        .build();

    match client.permission.transfer_owner(&request, None).await {
        Ok(response) => {
            println!("✅ 完全所有权转移成功!");

            if let Some(data) = response.data {
                println!("{}", data.success_summary());

                let member = &data.member;
                println!("📋 完全转移详情:");
                println!("  新所有者: {}", member.new_owner_info());

                if member.has_old_owner_info() {
                    if let Some(old_info) = member.old_owner_info() {
                        println!("  {}", old_info);
                    }
                    println!("  ⚠️  原所有者权限已完全移除");
                } else {
                    println!("  📝 原所有者信息未提供");
                }

                println!("  🔇 静默转移，未发送通知");

                if data.is_transferred() {
                    println!("  ✅ 所有权转移确认完成");
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 完全所有权转移失败: {:?}", e);
        }
    }

    println!("{}", "\n".to_string() + &"=".repeat(50));

    // 示例3：转移电子表格所有权给群组
    println!("🔹 示例3: 转移电子表格所有权给群组");
    let request = TransferOwnerRequest::builder()
        .token("shtcnxxxxxxxxxxxxxx") // 替换为实际的电子表格token
        .as_sheet()
        .to_chat("target_chat_id") // 替换为实际的群组ID
        .keep_current_owner() // 保留原所有者权限
        .with_notification()
        .build();

    match client.permission.transfer_owner(&request, None).await {
        Ok(response) => {
            println!("✅ 表格所有权转移给群组成功!");

            if let Some(data) = response.data {
                println!("{}", data.success_summary());

                let member = &data.member;
                println!("📋 群组所有权转移:");
                println!("  群组ID: {}", member.member_id);
                println!("  📊 电子表格现由群组管理");

                if let Some(transfer_time) = data.transfer_time() {
                    println!("  🕒 转移时间戳: {}", transfer_time);
                }

                println!("  👥 群组成员权限说明:");
                println!("     - 群组管理员：完全管理权限");
                println!("     - 群组成员：根据群组设置获得相应权限");
                println!("     - 非群组成员：需要单独授权");
            }
        }
        Err(e) => {
            eprintln!("❌ 群组所有权转移失败: {:?}", e);
        }
    }

    println!("{}", "\n".to_string() + &"=".repeat(50));

    // 示例4：转移知识库所有权给部门
    println!("🔹 示例4: 转移知识库所有权给部门");
    let request = TransferOwnerRequest::builder()
        .token("wikicnxxxxxxxxxxxxxx") // 替换为实际的知识库token
        .as_wiki()
        .to_department("target_department_id") // 替换为实际的部门ID
        .remove_current_owner() // 移除原所有者
        .with_notification()
        .build();

    match client.permission.transfer_owner(&request, None).await {
        Ok(response) => {
            println!("✅ 知识库所有权转移给部门成功!");

            if let Some(data) = response.data {
                println!("{}", data.success_summary());

                let member = &data.member;
                println!("📋 部门所有权转移:");
                println!("  部门ID: {}", member.member_id);
                println!("  📚 知识库现由部门管理");

                println!("  🏢 部门管理优势:");
                println!("     - 统一的权限管理");
                println!("     - 部门成员自动继承相应权限");
                println!("     - 便于组织架构变更时的权限调整");

                if member.has_transfer_time() {
                    println!("  ✅ 转移已完成并记录时间");
                }

                println!("  📧 已通知相关部门成员");
            }
        }
        Err(e) => {
            eprintln!("❌ 部门所有权转移失败: {:?}", e);
        }
    }

    println!("\n⚠️  所有权转移重要提醒:");
    println!("   1. 转移所有权是高风险操作，请谨慎执行");
    println!("   2. 建议在转移前确认新所有者的身份和权限需求");
    println!("   3. 如选择移除原所有者权限，请确保有其他管理员");
    println!("   4. 转移给群组或部门时，注意成员权限的继承规则");
    println!("   5. 重要文档建议保留原所有者的编辑权限作为备份");

    Ok(())
}
