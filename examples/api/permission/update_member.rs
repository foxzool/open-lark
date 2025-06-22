use dotenvy::dotenv;
use open_lark::prelude::*;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID environment variable not set");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET environment variable not set");

    let client = LarkClient::builder(&app_id, &app_secret).build();

    // 示例1：将用户权限从查看者升级为编辑者
    println!("🔹 示例1: 将用户权限升级为编辑者");
    let request = UpdatePermissionMemberRequest::builder()
        .token("doccnxxxxxxxxxxxxxx") // 替换为实际的文档token
        .as_doc()
        .user("user_id_example") // 替换为实际的用户ID
        .to_editor() // 升级为编辑者
        .with_notification()
        .build();

    match client.permission.update_member(&request, None).await {
        Ok(response) => {
            println!("✅ 更新协作者权限成功!");

            if let Some(data) = response.data {
                println!("{}", data.success_summary());

                let member = &data.member;
                println!("📋 权限变化:");
                println!("  用户ID: {}", member.member_id());
                println!("  权限变化: {}", member.permission_change_description());

                if member.permission_upgraded() {
                    println!("  📈 权限已升级");
                } else if member.permission_downgraded() {
                    println!("  📉 权限已降级");
                } else if !member.permission_changed() {
                    println!("  ⚪ 权限无变化");
                }

                if member.was_notified() {
                    println!("  📧 已发送通知");
                }

                if let Some(time) = member.update_time_formatted() {
                    println!("  🕒 {}", time);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 更新协作者权限失败: {:?}", e);
        }
    }

    println!("\n" + &"=".repeat(50));

    // 示例2：使用便捷方法将群组权限降级为查看者
    println!("🔹 示例2: 将群组权限降级为查看者");
    let request = UpdatePermissionMemberRequest::for_chat(
        "doccnxxxxxxxxxxxxxx", // 替换为实际的文档token
        "doc",
        "chat_id_example", // 替换为实际的群组ID
        Permission::View,
    );

    match client.permission.update_member(&request, None).await {
        Ok(response) => {
            println!("✅ 更新群组权限成功!");

            if let Some(data) = response.data {
                println!("{}", data.success_summary());

                let member = &data.member;
                println!("📋 群组权限更新:");
                println!("  群组ID: {}", member.member_id());

                if let Some(old_perm) = member.old_permission() {
                    println!("  原权限: {}", old_perm.description());
                }
                println!("  新权限: {}", member.new_permission().description());

                if member.permission_downgraded() {
                    println!("  ⚠️  权限已降级，群组成员编辑能力受限");
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 更新群组权限失败: {:?}", e);
        }
    }

    println!("\n" + &"=".repeat(50));

    // 示例3：在电子表格中更新部门权限（静默更新）
    println!("🔹 示例3: 在电子表格中更新部门权限（静默）");
    let request = UpdatePermissionMemberRequest::builder()
        .token("shtcnxxxxxxxxxxxxxx") // 替换为实际的电子表格token
        .as_sheet()
        .department("department_id_example") // 替换为实际的部门ID
        .to_commenter() // 更新为评论者
        .without_notification() // 静默更新
        .build();

    match client.permission.update_member(&request, None).await {
        Ok(response) => {
            println!("✅ 更新部门权限成功!");

            if let Some(data) = response.data {
                println!("{}", data.success_summary());

                let member = &data.member;
                if !member.was_notified() {
                    println!("  🔇 静默更新，未发送通知");
                }

                println!("  🏢 部门ID: {}", member.member_id());
                println!("  📊 表格权限: {}", member.new_permission().description());

                // 权限级别分析
                let level = member.new_permission().level();
                match level {
                    1 => println!("  👁️  仅可查看数据"),
                    2 => println!("  💬 可查看和评论"),
                    3 => println!("  ✏️  可编辑数据"),
                    4 => println!("  🛡️  完全管理权限"),
                    _ => println!("  ❓ 未知权限级别"),
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 更新部门权限失败: {:?}", e);
        }
    }

    Ok(())
}
