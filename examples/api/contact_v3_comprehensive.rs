use dotenvy::dotenv;
use open_lark::{prelude::*, service::contact::models::*};
use std::env;

/// 飞书 Contact v3 通讯录 API 综合演示
///
/// 本示例展示了通讯录系统的主要功能模块：
/// - 用户管理：创建、查询、搜索、更新、删除用户
/// - 部门管理：创建、查询、搜索部门
/// - 用户组管理：创建、管理用户组
/// - 权限范围查询
/// - 职级、序列、职务管理
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID environment variable not set");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET environment variable not set");

    // 创建客户端
    let client = LarkClient::builder(app_id, app_secret)
        .with_app_type(AppType::SelfBuilt)
        .with_enable_token_cache(true)
        .build();

    println!("🚀 开始演示飞书 Contact v3 通讯录 API 功能");

    // ========== 权限范围查询 ==========
    println!("\n📋 1. 权限范围管理");

    match client
        .contact
        .v3
        .scope
        .list(&GetScopeRequest::default())
        .await
    {
        Ok(response) => {
            println!("✅ 权限范围查询成功:");
            println!("   - 权限范围数量: {}", response.scopes.len());
            if let Some(scope) = response.scopes.first() {
                if let Some(departments) = &scope.departments {
                    println!("   - 可访问部门数: {}", departments.len());
                }
                if let Some(users) = &scope.users {
                    println!("   - 可访问用户数: {}", users.len());
                }
            }
        }
        Err(e) => println!("❌ 权限范围查询失败: {:?}", e),
    }

    // ========== 用户管理演示 ==========
    println!("\n👤 2. 用户管理功能");

    // 2.1 获取用户列表
    match client
        .contact
        .v3
        .user
        .list(&ListUsersRequest {
            page_size: Some(10),
            ..Default::default()
        })
        .await
    {
        Ok(response) => {
            println!("✅ 用户列表获取成功:");
            println!("   - 用户数量: {}", response.items.len());
            println!("   - 是否还有更多: {:?}", response.has_more);

            for (i, user) in response.items.iter().take(3).enumerate() {
                println!(
                    "   {}. {} ({})",
                    i + 1,
                    user.name.as_deref().unwrap_or("未知"),
                    user.email.as_deref().unwrap_or("无邮箱")
                );
            }
        }
        Err(e) => println!("❌ 用户列表获取失败: {:?}", e),
    }

    // 2.2 搜索用户
    match client
        .contact
        .v3
        .user
        .search(&SearchUsersRequest {
            query: "test".to_string(),
            page_size: Some(5),
            ..Default::default()
        })
        .await
    {
        Ok(response) => {
            println!("✅ 用户搜索成功:");
            println!("   - 搜索结果数量: {}", response.items.len());
            for user in &response.items {
                println!(
                    "   - {}: {}",
                    user.name.as_deref().unwrap_or("未知"),
                    user.email.as_deref().unwrap_or("无邮箱")
                );
            }
        }
        Err(e) => println!("❌ 用户搜索失败: {:?}", e),
    }

    // ========== 部门管理演示 ==========
    println!("\n🏢 3. 部门管理功能");

    // 3.1 获取子部门列表
    match client
        .contact
        .v3
        .department
        .children(&GetChildDepartmentsRequest {
            department_id: Some("0".to_string()), // 根部门
            page_size: Some(10),
            ..Default::default()
        })
        .await
    {
        Ok(response) => {
            println!("✅ 子部门列表获取成功:");
            println!("   - 子部门数量: {}", response.items.len());
            for dept in &response.items {
                println!(
                    "   - 部门: {} ({})",
                    dept.name.as_deref().unwrap_or("未知"),
                    dept.department_id.as_deref().unwrap_or("无ID")
                );
            }
        }
        Err(e) => println!("❌ 子部门列表获取失败: {:?}", e),
    }

    // 3.2 搜索部门
    match client
        .contact
        .v3
        .department
        .search(&SearchDepartmentsRequest {
            query: "技术".to_string(),
            page_size: Some(5),
            ..Default::default()
        })
        .await
    {
        Ok(response) => {
            println!("✅ 部门搜索成功:");
            println!("   - 搜索结果数量: {}", response.items.len());
        }
        Err(e) => println!("❌ 部门搜索失败: {:?}", e),
    }

    // ========== 用户组管理演示 ==========
    println!("\n👥 4. 用户组管理功能");

    // 4.1 获取用户组列表
    match client
        .contact
        .v3
        .group
        .simplelist(&GetGroupsRequest {
            page_size: Some(10),
            ..Default::default()
        })
        .await
    {
        Ok(response) => {
            println!("✅ 用户组列表获取成功:");
            println!("   - 用户组数量: {}", response.groups.len());

            // 4.2 如果有用户组，获取详细信息
            if let Some(group) = response.groups.first() {
                if let Some(group_id) = &group.id {
                    match client
                        .contact
                        .v3
                        .group
                        .get_detail(
                            group_id,
                            &GetGroupDetailRequest {
                                include_members: Some(true),
                                ..Default::default()
                            },
                        )
                        .await
                    {
                        Ok(detail_response) => {
                            println!("✅ 用户组详细信息获取成功:");
                            let group_detail = &detail_response.group;
                            println!(
                                "   - 用户组名称: {}",
                                group_detail.name.as_deref().unwrap_or("未知")
                            );
                            println!("   - 成员数量: {:?}", group_detail.member_count);
                            if let Some(members) = &group_detail.members {
                                println!("   - 成员列表: {} 个成员", members.len());
                            }
                        }
                        Err(e) => println!("❌ 用户组详细信息获取失败: {:?}", e),
                    }
                }
            }
        }
        Err(e) => println!("❌ 用户组列表获取失败: {:?}", e),
    }

    // ========== 自定义字段查询 ==========
    println!("\n🔧 5. 自定义字段管理");

    match client
        .contact
        .v3
        .custom_attr
        .list(&GetCustomAttrsRequest {
            page_size: Some(10),
            ..Default::default()
        })
        .await
    {
        Ok(response) => {
            println!("✅ 自定义字段列表获取成功:");
            println!("   - 自定义字段数量: {}", response.items.len());
            for attr in &response.items {
                println!(
                    "   - 字段: {} ({})",
                    attr.name
                        .as_ref()
                        .and_then(|n| n.zh_cn.as_deref())
                        .unwrap_or("未知"),
                    attr.r#type.as_deref().unwrap_or("unknown")
                );
            }
        }
        Err(e) => println!("❌ 自定义字段列表获取失败: {:?}", e),
    }

    // ========== 人员类型管理 ==========
    println!("\n👨‍💼 6. 人员类型管理");

    match client
        .contact
        .v3
        .employee_type_enum
        .list(&GetEmployeeTypeEnumsRequest {
            page_size: Some(10),
            ..Default::default()
        })
        .await
    {
        Ok(response) => {
            println!("✅ 人员类型列表获取成功:");
            println!("   - 人员类型数量: {}", response.items.len());
            for enum_type in &response.items {
                println!(
                    "   - 类型: {} ({})",
                    enum_type.content.as_deref().unwrap_or("未知"),
                    enum_type.enum_value.as_deref().unwrap_or("unknown")
                );
            }
        }
        Err(e) => println!("❌ 人员类型列表获取失败: {:?}", e),
    }

    // ========== 职级管理 ==========
    println!("\n📊 7. 职级管理");

    match client
        .contact
        .v3
        .job_level
        .list(&GetJobLevelsRequest {
            page_size: Some(10),
            ..Default::default()
        })
        .await
    {
        Ok(response) => {
            println!("✅ 职级列表获取成功:");
            println!("   - 职级数量: {}", response.items.len());
            for job_level in &response.items {
                let name = job_level
                    .name
                    .as_ref()
                    .and_then(|names| names.first())
                    .and_then(|name| name.value.as_deref())
                    .unwrap_or("未知");
                println!(
                    "   - 职级: {} (等级: {})",
                    name,
                    job_level.rank.unwrap_or(0)
                );
            }
        }
        Err(e) => println!("❌ 职级列表获取失败: {:?}", e),
    }

    // ========== 序列管理 ==========
    println!("\n🔗 8. 序列管理");

    match client
        .contact
        .v3
        .job_family
        .list(&GetJobFamiliesRequest {
            page_size: Some(10),
            ..Default::default()
        })
        .await
    {
        Ok(response) => {
            println!("✅ 序列列表获取成功:");
            println!("   - 序列数量: {}", response.items.len());
            for job_family in &response.items {
                let name = job_family
                    .name
                    .as_ref()
                    .and_then(|names| names.first())
                    .and_then(|name| name.value.as_deref())
                    .unwrap_or("未知");
                println!(
                    "   - 序列: {} (状态: {})",
                    name,
                    if job_family.status.unwrap_or(false) {
                        "启用"
                    } else {
                        "禁用"
                    }
                );
            }
        }
        Err(e) => println!("❌ 序列列表获取失败: {:?}", e),
    }

    // ========== 角色管理演示 ==========
    println!("\n👑 9. 角色管理");

    match client
        .contact
        .v3
        .functional_role_member
        .list(
            "example_role_id",
            &GetRoleMembersRequest {
                page_size: Some(10),
                ..Default::default()
            },
        )
        .await
    {
        Ok(response) => {
            println!("✅ 角色成员列表获取成功:");
            println!("   - 角色成员数量: {}", response.items.len());
        }
        Err(e) => println!("❌ 角色成员列表获取失败: {:?}", e),
    }

    println!("\n🎉 Contact v3 通讯录 API 综合演示完成!");
    println!("本示例展示了:");
    println!("  📋 权限范围管理: 获取访问权限范围");
    println!("  👤 用户管理: 列表、搜索、CRUD操作");
    println!("  🏢 部门管理: 层级查询、搜索");
    println!("  👥 用户组管理: 创建、详情、成员管理");
    println!("  🔧 自定义字段: 企业字段配置");
    println!("  👨‍💼 人员类型: 员工分类管理");
    println!("  📊 职级序列: 职业发展体系");
    println!("  👑 角色权限: 功能角色管理");

    Ok(())
}
