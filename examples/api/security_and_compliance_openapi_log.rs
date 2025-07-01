use anyhow::Result;
use open_lark::{prelude::*, service::security_and_compliance::models::OpenapiLogListRequest};

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
    let request = OpenapiLogListRequest {
        page_size: Some(10),
        start_time: Some(1701234567000), // 2023-11-29 的毫秒时间戳
        end_time: Some(1701320967000),   // 2023-11-30 的毫秒时间戳
        ..Default::default()
    };

    println!("获取 OpenAPI 审计日志数据...");

    match client
        .security_and_compliance
        .openapi_log
        .list_data(request, None)
        .await
    {
        Ok(response) => {
            println!("API 调用成功:");
            println!("- 返回码: {}", response.code());
            println!("- 消息: {}", response.msg());

            if let Some(data) = response.data {
                println!("- 是否有更多数据: {}", data.has_more);
                println!("- 日志数量: {}", data.items.len());

                for (index, item) in data.items.iter().enumerate() {
                    println!("\n审计日志 {}:", index + 1);
                    println!("  - 时间戳: {}", item.timestamp);
                    println!("  - 应用 ID: {}", item.app_id);
                    println!("  - 应用名称: {}", item.app_name);
                    println!("  - API 接口: {}", item.api);
                    println!("  - HTTP 方法: {}", item.method);
                    println!("  - 请求 ID: {}", item.request_id);
                    println!("  - 返回码: {}", item.response_code);
                    println!("  - 响应时间: {} ms", item.response_time);

                    if let Some(user_id) = &item.user_id {
                        println!("  - 用户 ID: {user_id}");
                    }
                    if let Some(ip) = &item.ip {
                        println!("  - IP 地址: {ip}");
                    }
                }

                if let Some(next_token) = data.page_token {
                    println!("\n下一页令牌: {next_token}");
                }
            }
        }
        Err(e) => {
            eprintln!("获取审计日志失败: {e}");
        }
    }

    Ok(())
}
