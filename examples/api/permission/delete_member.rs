use dotenvy::dotenv;
use open_lark::prelude::*;
use open_lark::service::permission::member::DeletePermissionMemberRequest;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID environment variable not set");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET environment variable not set");

    let client = LarkClient::builder(&app_id, &app_secret).build();

    // 示例1：移除用户的协作权限
    println!("🔹 示例1: 移除用户的协作权限");
    let request = DeletePermissionMemberRequest::builder()
        .token("doccnxxxxxxxxxxxxxx") // 替换为实际的文档token
        .as_doc()
        .user("user_id_example") // 替换为实际的用户ID
        .with_notification()
        .build();

    match client.permission.delete_member(&request, None).await {
        Ok(response) => {
            println!("✅ 移除协作者成功!");

            if let Some(data) = response.data {
                println!("{}", data.success_summary());

                let member = &data.member;
                println!("📋 移除详情:");
                println!("  用户ID: {}", member.member_id());
                println!("  风险级别: {}", data.risk_level());

                if let Some(old_perm) = member.old_permission() {
                    println!("  原权限: {}", old_perm.description());

                    // 权限分析
                    if member.was_owner() {
                        println!("  ⚠️  移除了所有者权限，请确认已转移所有权");
                    } else if member.could_edit() {
                        println!("  📝 移除了编辑权限");
                    } else {
                        println!("  👁️  移除了查看权限");
                    }
                }

                if member.was_notified() {
                    println!("  📧 已通知用户权限移除");
                }

                if let Some(time) = member.delete_time_formatted() {
                    println!("  🕒 {}", time);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 移除协作者失败: {:?}", e);
        }
    }

    println!("{}", "\n".to_string() + &"=".repeat(50));

    // 示例2：使用便捷方法移除群组权限
    println!("🔹 示例2: 移除群组的协作权限");
    let request = DeletePermissionMemberRequest::for_chat(
        "doccnxxxxxxxxxxxxxx", // 替换为实际的文档token
        "doc",
        "chat_id_example", // 替换为实际的群组ID
    );

    match client.permission.delete_member(&request, None).await {
        Ok(response) => {
            println!("✅ 移除群组协作权限成功!");

            if let Some(data) = response.data {
                println!("{}", data.success_summary());

                let member = &data.member;
                println!("📋 群组移除详情:");
                println!("  群组ID: {}", member.member_id());
                println!("  风险级别: {}", data.risk_level());

                if data.deleted_editor() {
                    println!("  📝 该群组失去了编辑权限");
                    println!("  💡 群组成员将无法再编辑此文档");
                }

                println!("  👥 群组所有成员的权限已移除");
            }
        }
        Err(e) => {
            eprintln!("❌ 移除群组协作权限失败: {:?}", e);
        }
    }

    println!("{}", "\n".to_string() + &"=".repeat(50));

    // 示例3：静默移除部门权限
    println!("🔹 示例3: 静默移除部门权限");
    let request = DeletePermissionMemberRequest::builder()
        .token("shtcnxxxxxxxxxxxxxx") // 替换为实际的电子表格token
        .as_sheet()
        .department("department_id_example") // 替换为实际的部门ID
        .without_notification() // 静默移除
        .build();

    match client.permission.delete_member(&request, None).await {
        Ok(response) => {
            println!("✅ 移除部门权限成功!");

            if let Some(data) = response.data {
                println!("{}", data.success_summary());

                let member = &data.member;
                if !member.was_notified() {
                    println!("  🔇 静默移除，未发送通知");
                }

                println!("  🏢 部门ID: {}", member.member_id());
                println!("  📊 电子表格权限已移除");
                println!("  风险级别: {}", data.risk_level());

                // 操作建议
                match data.risk_level() {
                    "高风险" => {
                        println!("  ⚠️  高风险操作！建议:");
                        println!("      - 确认已有其他管理员");
                        println!("      - 检查是否影响业务流程");
                    }
                    "中风险" => {
                        println!("  ⚠️  中风险操作，请确认:");
                        println!("      - 部门成员是否仍需要编辑权限");
                        println!("      - 是否有其他协作方式");
                    }
                    "低风险" => {
                        println!("  ✅ 低风险操作，影响较小");
                    }
                    _ => {}
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 移除部门权限失败: {:?}", e);
        }
    }

    println!("\n💡 权限管理提示:");
    println!("   - 移除所有者权限前请先转移所有权");
    println!("   - 移除编辑权限可能影响协作流程");
    println!("   - 建议在移除重要权限前备份相关数据");

    Ok(())
}
