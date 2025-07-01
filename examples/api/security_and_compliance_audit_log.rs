use anyhow::Result;
use open_lark::{prelude::*, service::security_and_compliance::models::AuditLogGetRequest};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("APP_ID is required");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET is required");

    // 创建飞书客户端
    let client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .with_enable_token_cache(true)
        .build();

    // 构建查询参数
    let request = AuditLogGetRequest {
        data_type: "all".to_string(),
        start_time: 1701234567000, // 2023-11-29 的毫秒时间戳
        end_time: 1701320967000,   // 2023-11-30 的毫秒时间戳
        page: Some(1),
        page_size: Some(10),
        // 可以添加特定的审计类型过滤
        audit_types: Some(vec![
            "login".to_string(),
            "logout".to_string(),
            "file_operation".to_string(),
        ]),
        ..Default::default()
    };

    println!("获取行为审计日志数据...");

    match client
        .security_and_compliance
        .audit_log
        .audit_data_get(request, None)
        .await
    {
        Ok(response) => {
            println!("API 调用成功:");
            println!("- 返回码: {}", response.code());
            println!("- 消息: {}", response.msg());

            if let Some(data) = response.data {
                println!("- 总数量: {}", data.total);
                println!("- 当前页: {}", data.page);
                println!("- 页面大小: {}", data.page_size);
                println!("- 本页日志数量: {}", data.items.len());

                for (index, item) in data.items.iter().enumerate() {
                    println!("\n审计日志 {}:", index + 1);
                    println!("  - 日志 ID: {}", item.log_id);
                    println!("  - 时间戳: {}", item.timestamp);
                    println!("  - 审计类型: {}", item.audit_type);
                    println!("  - 操作人 ID: {}", item.operator_id);
                    println!("  - 操作人姓名: {}", item.operator_name);
                    println!("  - 操作详情: {}", item.operation_detail);

                    if let Some(object_id) = &item.object_id {
                        println!("  - 被操作对象 ID: {object_id}");
                    }
                    if let Some(object_name) = &item.object_name {
                        println!("  - 被操作对象名称: {object_name}");
                    }
                    if let Some(ip) = &item.ip {
                        println!("  - IP 地址: {ip}");
                    }
                    if let Some(device_info) = &item.device_info {
                        println!("  - 设备信息: {device_info}");
                    }
                    if let Some(extend_info) = &item.extend_info {
                        println!("  - 扩展信息: {extend_info}");
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("获取行为审计日志失败: {e}");
        }
    }

    Ok(())
}
