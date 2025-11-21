//! Base V2 角色管理 API 示例
//!
//! 展示如何使用多维表格 V2 版本的角色管理功能：
//! - 创建自定义角色
//! - 查询角色列表
//! - 更新角色信息

use openlark_core::config::Config;
use openlark_docs::{
    base::bitable::{BitableService, v2::{CreateRoleV2RequestBuilder, ListRolesV2RequestBuilder, RoleV2, UpdateRoleV2RequestBuilder}},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化配置（实际使用时从环境变量或配置文件读取）
    let config = Config::default();

    // 创建Bitable服务
    let bitable_service = BitableService::new(config.clone());

    // 获取V2服务
    let v2_service = bitable_service.v2();
    let role_management = v2_service.role_management();

    // 示例1: 创建自定义角色
    println!("=== 创建自定义角色 V2 ===");

    let create_response = CreateRoleV2RequestBuilder::new(
        config.clone(),
        "app_token_xxx".to_string(),
        "高级编辑者".to_string(),
    )
    .description("具有高级编辑和查看权限的角色".to_string())
    .permission("table:read".to_string())
    .permission("table:write".to_string())
    .permission("table:delete".to_string())
    .execute()
    .await;

    match create_response {
        Ok(response) => {
            println!("✅ 角色创建成功!");
            if let Some(role_id) = response.role_id.as_ref() {
                println!("   角色 ID: {}", role_id);
            }
            if let Some(role) = &response.role {
                print_role_info(role);
            }
        }
        Err(e) => {
            println!("❌ 角色创建失败: {}", e);
        }
    }

    // 示例2: 列出自定义角色
    println!("\n=== 列出自定义角色 V2 ===");

    let list_response = ListRolesV2RequestBuilder::new(
        config.clone(),
        "app_token_xxx".to_string(),
    )
    .page_size(20)
    .execute()
    .await;

    match list_response {
        Ok(response) => {
            println!("✅ 角色列表获取成功! 共 {} 个角色", response.roles.len());
            for (index, role) in response.roles.iter().enumerate() {
                println!("\n[{}] 角色信息:", index + 1);
                print_role_info(role);
            }

            if response.has_more.unwrap_or(false) {
                println!("\n📄 还有更多角色数据可获取");
            }
        }
        Err(e) => {
            println!("❌ 角色列表获取失败: {}", e);
        }
    }

    // 示例3: 更新自定义角色
    println!("\n=== 更新自定义角色 V2 ===");

    // 假设我们知道角色ID
    let role_id = "role_123456";

    let update_response = UpdateRoleV2RequestBuilder::new(
        config.clone(),
        "app_token_xxx".to_string(),
        role_id.to_string(),
    )
    .name("超级管理员".to_string())
    .description("具有所有权限的管理员角色".to_string())
    .permission("table:read".to_string())
    .permission("table:write".to_string())
    .permission("table:delete".to_string())
    .permission("table:admin".to_string())
    .permission("view:manage".to_string())
    .execute()
    .await;

    match update_response {
        Ok(response) => {
            println!("✅ 角色更新成功!");
            if let Some(role) = &response.role {
                print_role_info(role);
            }
        }
        Err(e) => {
            println!("❌ 角色更新失败: {}", e);
        }
    }

    Ok(())
}

/// 打印角色信息的辅助函数
fn print_role_info(role: &RoleV2) {
    println!("   🏷️  角色ID: {}", role.role_id);
    println!("   👤 角色名称: {}", role.name);

    if let Some(description) = &role.description {
        println!("   📝 描述: {}", description);
    }

    if !role.permissions.is_empty() {
        println!("   🔑 权限列表:");
        for permission in &role.permissions {
            println!("      • {}", permission);
        }
    }

    if let Some(create_time) = role.create_time {
        println!("   📅 创建时间: {}", timestamp_to_string(create_time));
    }

    if let Some(update_time) = role.update_time {
        println!("   🔄 更新时间: {}", timestamp_to_string(update_time));
    }
}

/// 时间戳转字符串的辅助函数
fn timestamp_to_string(timestamp: i64) -> String {
    // 简单的时间戳转换，实际使用时可能需要更复杂的时间处理
    format!("{}", timestamp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_role_builder_pattern() {
        let config = Config::default();

        // 测试构建器模式
        let builder = CreateRoleV2RequestBuilder::new(
            config,
            "test_app_token".to_string(),
            "测试角色".to_string(),
        )
        .description("这是一个测试角色".to_string())
        .permission("read".to_string())
        .permission("write".to_string());

        // 验证构建器创建成功
        assert_eq!(builder.request.name, "测试角色");
        assert_eq!(builder.request.description, Some("这是一个测试角色".to_string()));
        assert_eq!(builder.request.permissions.len(), 2);
    }

    #[test]
    fn test_list_roles_builder_pattern() {
        let config = Config::default();

        // 测试列表构建器
        let builder = ListRolesV2RequestBuilder::new(
            config,
            "test_app_token".to_string(),
        )
        .page_size(10)
        .page_token("token_123".to_string());

        // 验证参数设置正确
        assert_eq!(builder.page_size, Some(10));
        assert_eq!(builder.page_token, Some("token_123".to_string()));
    }

    #[test]
    fn test_update_role_builder_pattern() {
        let config = Config::default();

        // 测试更新构建器
        let builder = UpdateRoleV2RequestBuilder::new(
            config,
            "test_app_token".to_string(),
            "role_123".to_string(),
        )
        .name("更新后的角色名".to_string())
        .permission("admin".to_string());

        // 验证参数设置正确
        assert_eq!(builder.request.name, Some("更新后的角色名".to_string()));
        assert_eq!(builder.request.permissions, vec!["admin".to_string()]);
    }
}