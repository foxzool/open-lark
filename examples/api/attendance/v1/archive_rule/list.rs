use dotenvy::dotenv;
use open_lark::{prelude::LarkClient, service::attendance::v1::models::ListArchiveRulesRequest};

/// 查询所有归档规则示例
///
/// 该接口用于获取企业的所有归档规则列表，包括规则名称、状态等信息。
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // 从环境变量获取应用配置
    let app_id = std::env::var("APP_ID").expect("APP_ID must be set");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET must be set");

    // 创建 LarkClient
    let client = LarkClient::builder(&app_id, &app_secret).build();

    // 构建查询所有归档规则请求
    let req = ListArchiveRulesRequest {
        employee_type: "employee_id".to_string(),
        page_size: Some(20),
        ..Default::default()
    };

    // 调用 API
    match client.attendance.v1.archive_rule.list(req, None).await {
        Ok(resp) => {
            println!("查询所有归档规则成功!");
            if let Some(data) = resp.data {
                println!("归档规则数量: {}", data.archive_rules.len());

                for (index, rule) in data.archive_rules.iter().enumerate() {
                    println!("归档规则 {}: ", index + 1);
                    println!("  规则ID: {}", rule.archive_rule_id);
                    println!("  规则名称: {}", rule.archive_rule_name);
                    println!("  状态: {}", if rule.is_enabled { "启用" } else { "禁用" });
                    println!("  创建时间: {}", rule.create_time);
                    println!("  更新时间: {}", rule.update_time);
                    if let Some(description) = &rule.description {
                        println!("  描述: {}", description);
                    }
                    println!();
                }

                // 检查是否有更多数据
                if let Some(page_token) = data.page_token {
                    println!("有更多数据，下一页token: {}", page_token);

                    // 如果需要获取下一页数据，可以这样做：
                    let _next_req = ListArchiveRulesRequest {
                        employee_type: "employee_id".to_string(),
                        page_size: Some(20),
                        page_token: Some(page_token),
                        ..Default::default()
                    };

                    println!("可以使用上述 next_req 获取下一页数据");
                }
            }
        }
        Err(e) => {
            eprintln!("查询所有归档规则失败: {:?}", e);
        }
    }

    Ok(())
}
