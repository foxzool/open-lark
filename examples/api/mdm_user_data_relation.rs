use dotenvy::dotenv;
use open_lark::{prelude::*, service::mdm::user_auth_data_relation::*};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID 环境变量未设置");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET 环境变量未设置");

    let client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .build();

    // 用户数据维度绑定
    println!("=== 用户数据维度绑定 ===");
    let bind_request = UserDataRelationBindRequest {
        user_id: "user_001".to_string(),
        data_dimension_id: "dimension_hr_001".to_string(),
        bind_type: Some("read_write".to_string()),
        permission_level: Some("full".to_string()),
        effective_start_time: Some(1640995200), // 2022-01-01 00:00:00 UTC
        effective_end_time: Some(1672531199),   // 2022-12-31 23:59:59 UTC
        remark: Some("为HR数据维度绑定用户访问权限".to_string()),
    };

    match client
        .mdm
        .user_auth_data_relation
        .bind(bind_request, None)
        .await
    {
        Ok(response) => {
            if let Some(bind_data) = response.data {
                if bind_data.success {
                    println!("用户数据维度绑定成功：");
                    let relation = &bind_data.relation;
                    println!("  - 用户ID: {}", relation.user_id);
                    println!("  - 数据维度ID: {}", relation.data_dimension_id);
                    if let Some(relation_id) = &relation.relation_id {
                        println!("  - 关系ID: {relation_id}");
                    }
                    if let Some(data_dimension_name) = &relation.data_dimension_name {
                        println!("  - 数据维度名称: {data_dimension_name}");
                    }
                    if let Some(bind_type) = &relation.bind_type {
                        println!("  - 绑定类型: {bind_type}");
                    }
                    if let Some(permission_level) = &relation.permission_level {
                        println!("  - 权限级别: {permission_level}");
                    }
                    if let Some(status) = &relation.status {
                        println!("  - 状态: {status}");
                    }
                    if let Some(effective_start_time) = relation.effective_start_time {
                        println!("  - 生效开始时间: {effective_start_time}");
                    }
                    if let Some(effective_end_time) = relation.effective_end_time {
                        println!("  - 生效结束时间: {effective_end_time}");
                    }
                    if let Some(creator) = &relation.creator {
                        println!("  - 创建者: {creator}");
                    }
                    if let Some(created_at) = relation.created_at {
                        println!("  - 创建时间: {created_at}");
                    }
                } else {
                    println!("用户数据维度绑定失败");
                    if let Some(error_message) = &bind_data.error_message {
                        println!("  错误信息: {error_message}");
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("用户数据维度绑定失败: {e:?}");
        }
    }

    // 用户数据维度解绑
    println!("\n=== 用户数据维度解绑 ===");
    let unbind_request = UserDataRelationUnbindRequest {
        user_id: "user_001".to_string(),
        data_dimension_id: "dimension_hr_001".to_string(),
        unbind_reason: Some("用户职责变更，不再需要该数据维度访问权限".to_string()),
    };

    match client
        .mdm
        .user_auth_data_relation
        .unbind(unbind_request, None)
        .await
    {
        Ok(response) => {
            if let Some(unbind_data) = response.data {
                if unbind_data.success {
                    println!("用户数据维度解绑成功：");
                    if let Some(relation_id) = &unbind_data.relation_id {
                        println!("  - 解绑的关系ID: {relation_id}");
                    }
                } else {
                    println!("用户数据维度解绑失败");
                    if let Some(error_message) = &unbind_data.error_message {
                        println!("  错误信息: {error_message}");
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("用户数据维度解绑失败: {e:?}");
        }
    }

    // 批量绑定示例
    println!("\n=== 批量绑定多个数据维度 ===");
    let dimensions = vec![
        ("dimension_finance_001", "财务数据维度"),
        ("dimension_sales_001", "销售数据维度"),
        ("dimension_marketing_001", "市场数据维度"),
    ];

    for (dimension_id, dimension_name) in dimensions {
        let batch_bind_request = UserDataRelationBindRequest {
            user_id: "user_002".to_string(),
            data_dimension_id: dimension_id.to_string(),
            bind_type: Some("read_only".to_string()),
            permission_level: Some("limited".to_string()),
            effective_start_time: Some(1640995200),
            effective_end_time: Some(1672531199),
            remark: Some(format!("为用户绑定{dimension_name}")),
        };

        match client
            .mdm
            .user_auth_data_relation
            .bind(batch_bind_request, None)
            .await
        {
            Ok(response) => {
                if let Some(bind_data) = response.data {
                    if bind_data.success {
                        println!("  ✓ {dimension_name} 绑定成功");
                    } else {
                        println!("  ✗ {dimension_name} 绑定失败");
                        if let Some(error_message) = &bind_data.error_message {
                            println!("    错误信息: {error_message}");
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("  ✗ {dimension_name} 绑定失败: {e:?}");
            }
        }
    }

    // 不同权限级别的绑定示例
    println!("\n=== 不同权限级别的绑定示例 ===");
    let permission_examples = vec![
        ("admin", "full", "管理员权限"),
        ("editor", "read_write", "编辑权限"),
        ("viewer", "read_only", "查看权限"),
    ];

    for (user_suffix, permission, description) in permission_examples {
        let permission_bind_request = UserDataRelationBindRequest {
            user_id: format!("user_{user_suffix}"),
            data_dimension_id: "dimension_test_001".to_string(),
            bind_type: Some("role_based".to_string()),
            permission_level: Some(permission.to_string()),
            effective_start_time: Some(1640995200),
            effective_end_time: None, // 永久有效
            remark: Some(format!("测试{description}绑定")),
        };

        match client
            .mdm
            .user_auth_data_relation
            .bind(permission_bind_request, None)
            .await
        {
            Ok(response) => {
                if let Some(bind_data) = response.data {
                    if bind_data.success {
                        println!("  ✓ {description} 绑定成功");
                    } else {
                        println!("  ✗ {description} 绑定失败");
                    }
                }
            }
            Err(e) => {
                eprintln!("  ✗ {description} 绑定失败: {e:?}");
            }
        }
    }

    Ok(())
}
