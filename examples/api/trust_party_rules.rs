use dotenvy::dotenv;
use open_lark::{
    prelude::*,
    service::trust_party::{models::*, searchable_visible_rules::*},
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

    // 查询现有的可搜可见规则
    println!("=== 查询可搜可见规则 ===");
    let list_request = RuleListRequest {
        page_size: Some(10),
        ..Default::default()
    };

    match client
        .trust_party
        .searchable_visible_rules
        .list_rules(list_request, None)
        .await
    {
        Ok(response) => {
            if let Some(rule_data) = response.data {
                println!("查询规则列表成功：");
                for rule in &rule_data.rules.items {
                    println!("  - 规则ID: {}", rule.rule_id);
                    println!("    规则名称: {}", rule.name);
                    if let Some(description) = &rule.description {
                        println!("    描述: {description}");
                    }
                    if let Some(rule_type) = &rule.rule_type {
                        println!("    类型: {rule_type}");
                    }
                    if let Some(status) = &rule.status {
                        println!("    状态: {status}");
                    }
                    println!();
                }

                // 如果已有规则，演示更新和删除操作
                if let Some(first_rule) = rule_data.rules.items.first() {
                    println!("=== 更新可搜可见规则 ===");
                    let update_request = RuleUpdateRequest {
                        name: Some(format!("更新后的{}", first_rule.name)),
                        description: Some("这是一个更新后的规则描述".to_string()),
                        rule_type: Some("visibility".to_string()),
                        status: Some("active".to_string()),
                        config: Some(RuleConfig {
                            visibility: Some("public".to_string()),
                            searchable: Some(true),
                            scope: Some(RuleScope {
                                department_ids: Some(vec!["dept_001".to_string()]),
                                user_ids: Some(vec!["user_001".to_string()]),
                                group_ids: None,
                            }),
                            exceptions: Some(vec!["exception_001".to_string()]),
                        }),
                    };

                    match client
                        .trust_party
                        .searchable_visible_rules
                        .update_rule(&first_rule.rule_id, update_request, None)
                        .await
                    {
                        Ok(response) => {
                            if let Some(update_data) = response.data {
                                println!("更新规则成功：");
                                let rule = &update_data.rule;
                                println!("  规则ID: {}", rule.rule_id);
                                println!("  规则名称: {}", rule.name);
                                if let Some(description) = &rule.description {
                                    println!("  描述: {description}");
                                }
                                if let Some(updated_at) = rule.updated_at {
                                    println!("  更新时间: {updated_at}");
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("更新规则失败: {e:?}");
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("查询规则列表失败: {e:?}");
        }
    }

    // 创建新的可搜可见规则
    println!("\n=== 创建可搜可见规则 ===");
    let create_request = RuleCreateRequest {
        name: "示例可见性规则".to_string(),
        description: Some("这是一个示例的可搜可见规则".to_string()),
        rule_type: Some("visibility".to_string()),
        org_id: Some("org_example_001".to_string()),
        config: Some(RuleConfig {
            visibility: Some("restricted".to_string()),
            searchable: Some(false),
            scope: Some(RuleScope {
                department_ids: Some(vec!["dept_001".to_string(), "dept_002".to_string()]),
                user_ids: Some(vec!["user_001".to_string(), "user_002".to_string()]),
                group_ids: Some(vec!["group_001".to_string()]),
            }),
            exceptions: Some(vec![
                "exception_user_001".to_string(),
                "exception_dept_001".to_string(),
            ]),
        }),
    };

    match client
        .trust_party
        .searchable_visible_rules
        .create_rule(create_request, None)
        .await
    {
        Ok(response) => {
            if let Some(create_data) = response.data {
                println!("创建规则成功：");
                let rule = &create_data.rule;
                println!("  规则ID: {}", rule.rule_id);
                println!("  规则名称: {}", rule.name);
                if let Some(description) = &rule.description {
                    println!("  描述: {description}");
                }
                if let Some(rule_type) = &rule.rule_type {
                    println!("  类型: {rule_type}");
                }
                if let Some(org_id) = &rule.org_id {
                    println!("  组织ID: {org_id}");
                }
                if let Some(created_at) = rule.created_at {
                    println!("  创建时间: {created_at}");
                }

                // 演示删除操作（可选）
                println!("\n=== 删除可搜可见规则示例 ===");
                println!("注意：以下删除操作已注释，如需测试请取消注释");
                // match client
                // .trust_party
                // .searchable_visible_rules
                // .delete_rule(&rule.rule_id, None)
                // .await
                // {
                // Ok(response) => {
                // if let Some(delete_data) = response.data {
                // if delete_data.success {
                // println!("删除规则成功：规则ID {}", rule.rule_id);
                // } else {
                // println!("删除规则失败：规则ID {}", rule.rule_id);
                // }
                // }
                // }
                // Err(e) => {
                // eprintln!("删除规则失败: {:?}", e);
                // }
                // }
            }
        }
        Err(e) => {
            eprintln!("创建规则失败: {e:?}");
        }
    }

    // 按条件查询规则
    println!("\n=== 按条件查询规则 ===");
    let filtered_request = RuleListRequest {
        page_size: Some(5),
        rule_type: Some("visibility".to_string()),
        status: Some("active".to_string()),
        ..Default::default()
    };

    match client
        .trust_party
        .searchable_visible_rules
        .list_rules(filtered_request, None)
        .await
    {
        Ok(response) => {
            if let Some(filtered_data) = response.data {
                println!("按条件查询规则成功：");
                for rule in &filtered_data.rules.items {
                    println!("  - 规则ID: {}", rule.rule_id);
                    println!("    规则名称: {}", rule.name);
                    if let Some(rule_type) = &rule.rule_type {
                        println!("    类型: {rule_type}");
                    }
                    if let Some(status) = &rule.status {
                        println!("    状态: {status}");
                    }
                    println!();
                }
            }
        }
        Err(e) => {
            eprintln!("按条件查询规则失败: {e:?}");
        }
    }

    Ok(())
}
