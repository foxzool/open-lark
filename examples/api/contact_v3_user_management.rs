use dotenvy::dotenv;
use open_lark::prelude::*;
use open_lark::service_registry::{SharedConfig, SharedConfigFactory};
use open_lark::core::config::ConfigBuilder;
use open_lark::service::contact::*;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenv().ok();
    
    let app_id = env::var("APP_ID").expect("APP_ID must be set");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET must be set");

    // 使用共享配置创建飞书客户端
    let shared_config = SharedConfigFactory::create_shared(
        ConfigBuilder::default()
            .app_id(&app_id)
            .app_secret(&app_secret)
            .enable_token_cache(true)
            .build()
    );

    let client = LarkClient::new(shared_config.config().clone());

    println!("=== 飞书通讯录 v3 用户管理示例 ===");
    println!("🔄 使用新的共享配置接口，优化内存使用");
    println!("📊 配置引用计数: {}", shared_config.ref_count());
    println!();

    // 1. 获取通讯录权限范围
    println!("1. 获取通讯录权限范围");
    match client.contact.v3.scope.list(&GetScopeRequest::default()).await {
        Ok(resp) => {
            println!("✅ 获取权限范围成功:");
            println!("   权限范围数量: {}", resp.data.scopes.len());
            if let Some(first_scope) = resp.data.scopes.first() {
                if let Some(departments) = &first_scope.departments {
                    println!("   部门数量: {}", departments.len());
                }
                if let Some(users) = &first_scope.users {
                    println!("   用户数量: {}", users.len());
                }
            }
        }
        Err(e) => println!("❌ 获取权限范围失败: {:?}", e),
    }
    println!();

    // 2. 获取部门直属用户列表 
    println!("2. 获取部门直属用户列表");
    let find_users_req = FindUsersByDepartmentRequest {
        department_id: Some("0".to_string()), // 根部门
        user_id_type: Some("user_id".to_string()),
        department_id_type: Some("department_id".to_string()),
        page_size: Some(10),
        ..Default::default()
    };
    
    match client.contact.v3.user.find_by_department(&find_users_req).await {
        Ok(resp) => {
            println!("✅ 获取部门用户成功:");
            println!("   用户数量: {}", resp.items.len());
            println!("   是否有更多: {:?}", resp.has_more);
            
            for user in &resp.items {
                if let (Some(user_id), Some(name)) = (&user.user_id, &user.name) {
                    println!("   - 用户: {} ({})", name, user_id);
                    if let Some(email) = &user.email {
                        println!("     邮箱: {}", email);
                    }
                    if let Some(mobile) = &user.mobile {
                        println!("     手机: {}", mobile);
                    }
                }
            }
        }
        Err(e) => println!("❌ 获取部门用户失败: {:?}", e),
    }
    println!();

    // 3. 搜索用户（示例）
    println!("3. 搜索用户");
    let search_req = SearchUsersRequest {
        query: "张".to_string(), // 搜索姓张的用户
        page_size: Some(5),
        user_id_type: Some("user_id".to_string()),
        ..Default::default()
    };

    match client.contact.v3.user.search(&search_req).await {
        Ok(resp) => {
            println!("✅ 搜索用户成功:");
            println!("   找到用户数量: {}", resp.items.len());
            
            for user in &resp.items {
                if let (Some(user_id), Some(name)) = (&user.user_id, &user.name) {
                    println!("   - 搜索结果: {} ({})", name, user_id);
                }
            }
        }
        Err(e) => println!("❌ 搜索用户失败: {:?}", e),
    }
    println!();

    // 4. 获取子部门列表
    println!("4. 获取子部门列表");
    let children_req = GetChildrenDepartmentsRequest {
        parent_department_id: Some("0".to_string()), // 根部门的子部门
        user_id_type: Some("user_id".to_string()),
        department_id_type: Some("department_id".to_string()),
        fetch_child: Some(false),
        page_size: Some(10),
        ..Default::default()
    };

    match client.contact.v3.department.children(&children_req).await {
        Ok(resp) => {
            println!("✅ 获取子部门成功:");
            println!("   子部门数量: {}", resp.items.len());
            
            for dept in &resp.items {
                if let (Some(dept_id), Some(name)) = (&dept.department_id, &dept.name) {
                    println!("   - 部门: {} ({})", name, dept_id);
                    if let Some(member_count) = dept.member_count {
                        println!("     成员数量: {}", member_count);
                    }
                }
            }
        }
        Err(e) => println!("❌ 获取子部门失败: {:?}", e),
    }
    println!();

    // 5. 获取用户组列表
    println!("5. 获取用户组列表");
    let groups_req = ListGroupsRequest {
        page_size: Some(10),
        user_id_type: Some("user_id".to_string()),
        ..Default::default()
    };

    match client.contact.v3.group.simplelist(&groups_req).await {
        Ok(resp) => {
            println!("✅ 获取用户组成功:");
            println!("   用户组数量: {}", resp.groups.len());
            
            for group in &resp.groups {
                if let (Some(group_id), Some(name)) = (&group.id, &group.name) {
                    println!("   - 用户组: {} ({})", name, group_id);
                    if let Some(desc) = &group.description {
                        println!("     描述: {}", desc);
                    }
                    if let Some(user_count) = group.member_user_count {
                        println!("     成员数量: {}", user_count);
                    }
                }
            }
        }
        Err(e) => println!("❌ 获取用户组失败: {:?}", e),
    }
    println!();

    // 6. 获取自定义用户字段
    println!("6. 获取自定义用户字段");
    let custom_attrs_req = ListCustomAttrsRequest {
        page_size: Some(10),
        ..Default::default()
    };

    match client.contact.v3.custom_attr.list(&custom_attrs_req).await {
        Ok(resp) => {
            println!("✅ 获取自定义字段成功:");
            println!("   自定义字段数量: {}", resp.items.len());
            
            for attr in &resp.items {
                if let (Some(id), Some(name)) = (&attr.id, &attr.name) {
                    let name_zh = name.zh_cn.as_deref().unwrap_or("未知");
                    println!("   - 字段: {} ({})", name_zh, id);
                    if let Some(field_type) = &attr.r#type {
                        println!("     类型: {}", field_type);
                    }
                    if let Some(required) = attr.is_required {
                        println!("     必填: {}", if required { "是" } else { "否" });
                    }
                }
            }
        }
        Err(e) => println!("❌ 获取自定义字段失败: {:?}", e),
    }
    println!();

    println!("=== 通讯录管理示例完成 ===");
    Ok(())
}