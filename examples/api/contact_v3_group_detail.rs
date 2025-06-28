use open_lark::{prelude::*, service::contact::v3::group::GetGroupDetailRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("APP_ID must be set");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET must be set");

    let client = LarkClient::builder(&app_id, &app_secret)
        .with_enable_token_cache(true)
        .build();

    // 用户组ID - 需要替换为实际的用户组ID
    let group_id = "your_group_id_here";

    // 构建获取用户组详细信息请求
    let req = GetGroupDetailRequest {
        user_id_type: Some("user_id".to_string()),
        department_id_type: Some("department_id".to_string()),
        include_members: Some(true), // 包含成员信息
    };

    match client.contact.v3.group.get_detail(group_id, &req).await {
        Ok(resp) => {
            println!("获取用户组详细信息成功:");
            let group = resp.group;

            if let Some(group_id) = &group.group_id {
                println!("用户组ID: {}", group_id);
            }
            if let Some(name) = &group.name {
                println!("用户组名称: {}", name);
            }
            if let Some(description) = &group.description {
                println!("用户组描述: {}", description);
            }
            if let Some(group_type) = group.group_type {
                println!("用户组类型: {}", group_type);
            }
            if let Some(member_count) = group.member_count {
                println!("成员数量: {}", member_count);
            }
            if let Some(create_time) = &group.create_time {
                println!("创建时间: {}", create_time);
            }
            if let Some(update_time) = &group.update_time {
                println!("更新时间: {}", update_time);
            }

            if let Some(members) = &group.members {
                println!("用户组成员:");
                for member in members {
                    if let Some(member_id) = &member.member_id {
                        println!("- 成员ID: {}", member_id);
                    }
                    if let Some(member_type) = &member.member_type {
                        println!("  成员类型: {}", member_type);
                    }
                    if let Some(member_id_type) = &member.member_id_type {
                        println!("  成员ID类型: {}", member_id_type);
                    }
                    println!("---");
                }
            }
        }
        Err(e) => eprintln!("获取用户组详细信息失败: {:?}", e),
    }

    Ok(())
}
