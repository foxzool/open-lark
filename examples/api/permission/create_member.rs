use dotenv::dotenv;
use open_lark::prelude::*;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID environment variable not set");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET environment variable not set");

    let client = LarkClient::builder(&app_id, &app_secret).build();

    // 示例1：为用户添加编辑权限
    println!("🔹 示例1: 为用户添加编辑权限");
    let request = CreatePermissionMemberRequest::builder()
        .token("doccnxxxxxxxxxxxxxx") // 替换为实际的文档token
        .as_doc()
        .user("user_id_example") // 替换为实际的用户ID
        .as_editor()
        .with_notification()
        .build();

    match client.permission.create_member(&request, None).await {
        Ok(response) => {
            println!("✅ 添加协作者成功!");
            
            if let Some(data) = response.data {
                println!("{}", data.success_summary());
                
                let member = &data.member;
                println!("📋 详细信息:");
                println!("  用户ID: {}", member.member_id());
                println!("  类型: {}", member.member_type_description());
                println!("  权限: {}", member.permission_description());
                println!("  权限级别: {}", member.perm.level());
                
                if member.was_notified() {
                    println!("  📧 已发送通知");
                }
                
                if let Some(time) = member.create_time_formatted() {
                    println!("  🕒 {}", time);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 添加协作者失败: {:?}", e);
        }
    }

    println!("\n" + &"=".repeat(50));

    // 示例2：使用便捷方法为群组添加查看权限
    println!("🔹 示例2: 为群组添加查看权限");
    let request = CreatePermissionMemberRequest::for_chat(
        "doccnxxxxxxxxxxxxxx", // 替换为实际的文档token
        "doc",
        "chat_id_example", // 替换为实际的群组ID
        Permission::View,
    );

    match client.permission.create_member(&request, None).await {
        Ok(response) => {
            println!("✅ 添加群组协作者成功!");
            
            if let Some(data) = response.data {
                println!("{}", data.success_summary());
                
                let member = &data.member;
                println!("📋 群组信息:");
                println!("  群组ID: {}", member.member_id());
                println!("  权限: {}", member.permission_description());
                
                if member.can_edit() {
                    println!("  ✏️  可以编辑");
                } else {
                    println!("  👁️  仅可查看");
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 添加群组协作者失败: {:?}", e);
        }
    }

    println!("\n" + &"=".repeat(50));

    // 示例3：为部门添加评论权限（无通知）
    println!("🔹 示例3: 为部门添加评论权限（静默）");
    let request = CreatePermissionMemberRequest::builder()
        .token("doccnxxxxxxxxxxxxxx") // 替换为实际的文档token
        .as_sheet() // 电子表格
        .department("department_id_example") // 替换为实际的部门ID
        .as_commenter()
        .without_notification() // 不发送通知
        .build();

    match client.permission.create_member(&request, None).await {
        Ok(response) => {
            println!("✅ 添加部门协作者成功!");
            
            if let Some(data) = response.data {
                println!("{}", data.success_summary());
                
                let member = &data.member;
                if !member.was_notified() {
                    println!("  🔇 静默添加，未发送通知");
                }
                
                println!("  🏢 部门ID: {}", member.member_id());
                println!("  💬 权限: {} (可评论)", member.permission_description());
            }
        }
        Err(e) => {
            eprintln!("❌ 添加部门协作者失败: {:?}", e);
        }
    }

    Ok(())
}