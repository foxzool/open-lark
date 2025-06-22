use dotenv::dotenv;
use open_lark::prelude::*;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID environment variable not set");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET environment variable not set");

    let client = LarkClient::builder(&app_id, &app_secret).build();

    // 示例：获取文档协作者列表
    let request = ListPermissionMembersRequest::builder()
        .token("doccnxxxxxxxxxxxxxx") // 替换为实际的文档token
        .as_doc()
        .page_size(50)
        .build();

    match client.permission.list_members(&request, None).await {
        Ok(response) => {
            println!("获取协作者列表成功!");

            if let Some(data) = response.data {
                println!("协作者列表 ({}个):", data.count());
                println!("{}", data.permission_summary());

                // 显示所有协作者
                for member in &data.members {
                    println!("\n👤 {}", member.summary());

                    if member.has_inherited_permission() {
                        println!("   📎 继承权限");
                        if let Some(inherit_info) = &member.inherit_info {
                            println!("   📍 来源: {}", inherit_info);
                        }
                    }
                }

                // 按权限类型分组显示
                println!("\n📊 按权限分组:");
                let permission_groups = data.group_by_permission();
                for (permission, members) in permission_groups {
                    println!("  {} ({}个):", permission, members.len());
                    for member in members {
                        println!("    - {} ({})", member.display_name(), member.member_id);
                    }
                }

                // 按成员类型分组显示
                println!("\n👥 按类型分组:");
                let type_groups = data.group_by_member_type();
                for (member_type, members) in type_groups {
                    let type_desc = match member_type.as_str() {
                        "user" => "用户",
                        "chat" => "群组",
                        "department" => "部门",
                        _ => "其他",
                    };
                    println!("  {} ({}个):", type_desc, members.len());
                    for member in members {
                        println!(
                            "    - {} - {}",
                            member.display_name(),
                            member.perm.description()
                        );
                    }
                }

                // 分页信息
                if data.has_more {
                    println!("\n📄 还有更多协作者");
                    if let Some(page_token) = &data.page_token {
                        println!("   下一页标记: {}", page_token);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("获取协作者列表失败: {:?}", e);
        }
    }

    Ok(())
}
