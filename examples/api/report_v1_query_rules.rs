//! 飞书汇报 v1 - 查询规则示例
//!
//! 该示例演示如何使用汇报 API 查询规则列表

use dotenvy::dotenv;
use open_lark::{
    core::req_option::RequestOption, prelude::*, service::report::rule::RuleQueryRequest,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID 环境变量未设置");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET 环境变量未设置");

    let client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .build();

    println!("===== 查询规则列表 =====");

    // 查询所有规则
    let request = RuleQueryRequest {
        page_size: Some(10),
        ..Default::default()
    };

    let response = client
        .report
        .rule
        .query(request, Option::<RequestOption>::None)
        .await?;

    if response.code() == 0 {
        println!("查询成功！");
        if let Some(data) = response.data {
            println!("规则总数: {}", data.rules.items.len());

            for (index, rule) in data.rules.items.iter().enumerate() {
                println!("\n规则 {}:", index + 1);
                println!("  - 规则ID: {}", rule.rule_id);
                println!("  - 规则名称: {}", rule.name);
                if let Some(description) = &rule.description {
                    println!("  - 描述: {description}");
                }
                if let Some(rule_type) = &rule.rule_type {
                    println!("  - 类型: {rule_type}");
                }
                if let Some(status) = &rule.status {
                    println!("  - 状态: {status}");
                }
            }

            if let Some(has_more) = data.rules.has_more {
                if has_more {
                    println!("\n还有更多规则数据...");
                }
            }
        }
    } else {
        eprintln!("查询失败: {} - {}", response.code(), response.msg());
    }

    println!("\n===== 按条件查询规则 =====");

    // 按状态查询规则
    let filtered_request = RuleQueryRequest {
        status: Some("active".to_string()),
        page_size: Some(5),
        ..Default::default()
    };

    let filtered_response = client
        .report
        .rule
        .query(filtered_request, Option::<RequestOption>::None)
        .await?;

    if filtered_response.code() == 0 {
        println!("按状态查询成功！");
        if let Some(data) = filtered_response.data {
            println!("活跃规则数: {}", data.rules.items.len());

            for rule in &data.rules.items {
                println!("  - {} ({})", rule.name, rule.rule_id);
            }
        }
    } else {
        eprintln!(
            "按状态查询失败: {} - {}",
            filtered_response.code(),
            filtered_response.msg()
        );
    }

    Ok(())
}
