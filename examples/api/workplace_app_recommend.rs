use dotenvy::dotenv;
use open_lark::{prelude::*, service::workplace::app_recommend::*};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID 环境变量未设置");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET 环境变量未设置");

    let client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .build();

    // 获取用户自定义常用的应用
    println!("=== 获取用户自定义常用的应用 ===");
    let favourite_request = FavouriteAppsRequest {
        page_size: Some(10),
        user_id: Some("user_001".to_string()),
        ..Default::default()
    };

    match client
        .workplace
        .app_recommend
        .get_favourite_apps(favourite_request, None)
        .await
    {
        Ok(response) => {
            println!("查询用户常用应用成功：");
            for favourite_app in &response.favourite_apps.items {
                println!("  - 应用ID: {}", favourite_app.app_id);
                if let Some(app_info) = &favourite_app.app_info {
                    if let Some(app_name) = &app_info.app_name {
                        println!("    应用名称: {app_name}");
                    }
                    if let Some(app_description) = &app_info.app_description {
                        println!("    应用描述: {app_description}");
                    }
                    if let Some(app_type) = &app_info.app_type {
                        println!("    应用类型: {app_type}");
                    }
                }
                if let Some(favourited_at) = favourite_app.favourited_at {
                    println!("    添加到常用时间: {favourited_at}");
                }
                if let Some(usage_frequency) = favourite_app.usage_frequency {
                    println!("    使用频率: {usage_frequency}");
                }
                if let Some(last_used_at) = favourite_app.last_used_at {
                    println!("    最后使用时间: {last_used_at}");
                }
                println!();
            }

            if let Some(has_more) = response.favourite_apps.has_more {
                if has_more {
                    println!("还有更多数据，可使用 page_token 继续查询");
                }
            }
        }
        Err(e) => {
            eprintln!("查询用户常用应用失败: {e:?}");
        }
    }

    // 获取管理员推荐的应用
    println!("=== 获取管理员推荐的应用 ===");
    let recommend_request = RecommendedAppsRequest {
        page_size: Some(10),
        user_id: Some("user_001".to_string()),
        department_id: Some("dept_001".to_string()),
        ..Default::default()
    };

    match client
        .workplace
        .app_recommend
        .get_recommended_apps(recommend_request, None)
        .await
    {
        Ok(response) => {
            println!("查询管理员推荐应用成功：");
            for recommended_app in &response.recommended_apps.items {
                println!("  - 应用ID: {}", recommended_app.app_id);
                if let Some(app_info) = &recommended_app.app_info {
                    if let Some(app_name) = &app_info.app_name {
                        println!("    应用名称: {app_name}");
                    }
                    if let Some(app_description) = &app_info.app_description {
                        println!("    应用描述: {app_description}");
                    }
                }
                if let Some(recommend_reason) = &recommended_app.recommend_reason {
                    println!("    推荐原因: {recommend_reason}");
                }
                if let Some(recommend_score) = recommended_app.recommend_score {
                    println!("    推荐分数: {recommend_score:.2}");
                }
                if let Some(recommended_at) = recommended_app.recommended_at {
                    println!("    推荐时间: {recommended_at}");
                }
                if let Some(rule_id) = &recommended_app.rule_id {
                    println!("    推荐规则ID: {rule_id}");
                }
                println!();
            }
        }
        Err(e) => {
            eprintln!("查询管理员推荐应用失败: {e:?}");
        }
    }

    // 获取当前设置的推荐规则列表
    println!("=== 获取当前设置的推荐规则列表 ===");
    let rules_request = RecommendRulesListRequest {
        page_size: Some(10),
        rule_type: Some("admin_recommend".to_string()),
        status: Some("active".to_string()),
        ..Default::default()
    };

    match client
        .workplace
        .app_recommend
        .list_recommend_rules(rules_request, None)
        .await
    {
        Ok(response) => {
            println!("查询推荐规则列表成功：");
            for rule in &response.recommend_rules.items {
                println!("  - 规则ID: {}", rule.rule_id);
                println!("    规则名称: {}", rule.rule_name);
                if let Some(rule_description) = &rule.rule_description {
                    println!("    规则描述: {rule_description}");
                }
                if let Some(rule_type) = &rule.rule_type {
                    println!("    规则类型: {rule_type}");
                }
                if let Some(status) = &rule.status {
                    println!("    状态: {status}");
                }
                if let Some(app_ids) = &rule.app_ids {
                    println!("    适用应用: {app_ids:?}");
                }
                if let Some(user_ids) = &rule.user_ids {
                    println!("    适用用户: {user_ids:?}");
                }
                if let Some(department_ids) = &rule.department_ids {
                    println!("    适用部门: {department_ids:?}");
                }
                if let Some(priority) = rule.priority {
                    println!("    优先级: {priority}");
                }
                if let Some(start_time) = rule.start_time {
                    println!("    生效开始时间: {start_time}");
                }
                if let Some(end_time) = rule.end_time {
                    println!("    生效结束时间: {end_time}");
                }
                if let Some(creator) = &rule.creator {
                    println!("    创建者: {creator}");
                }
                if let Some(created_at) = rule.created_at {
                    println!("    创建时间: {created_at}");
                }
                if let Some(updated_at) = rule.updated_at {
                    println!("    更新时间: {updated_at}");
                }
                println!();
            }
        }
        Err(e) => {
            eprintln!("查询推荐规则列表失败: {e:?}");
        }
    }

    // 无条件查询所有常用应用
    println!("=== 无条件查询所有常用应用 ===");
    let all_favourite_request = FavouriteAppsRequest {
        page_size: Some(20),
        ..Default::default()
    };

    match client
        .workplace
        .app_recommend
        .get_favourite_apps(all_favourite_request, None)
        .await
    {
        Ok(response) => {
            println!(
                "查询所有常用应用成功，共 {} 个应用",
                response.favourite_apps.items.len()
            );
            for (index, favourite_app) in response.favourite_apps.items.iter().enumerate() {
                println!("  {}. 应用ID: {}", index + 1, favourite_app.app_id);
                if let Some(app_info) = &favourite_app.app_info {
                    if let Some(app_name) = &app_info.app_name {
                        println!("     应用名称: {app_name}");
                    }
                }
                if let Some(usage_frequency) = favourite_app.usage_frequency {
                    println!("     使用频率: {usage_frequency}");
                }
            }
        }
        Err(e) => {
            eprintln!("查询所有常用应用失败: {e:?}");
        }
    }

    Ok(())
}
