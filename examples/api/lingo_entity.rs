use dotenvy::dotenv;
use open_lark::{prelude::*, service::lingo::entity::*};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID 环境变量未设置");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET 环境变量未设置");

    let client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .build();

    // 获取词条列表
    println!("=== 获取词条列表 ===");
    let list_request = EntityListRequest {
        page_size: Some(10),
        ..Default::default()
    };

    match client.lingo.entity.list_entities(list_request, None).await {
        Ok(response) => {
            if let Some(entity_data) = response.data {
                println!("获取词条列表成功：");
                for entity in &entity_data.entities.items {
                    println!("  - 词条ID: {}", entity.id);
                    println!("    主名称: {:?}", entity.main_keys);
                    let description_preview: String = entity.description.chars().take(50).collect();
                    if !description_preview.is_empty() {
                        println!("    描述: {description_preview}...");
                    }
                    println!();
                }
            }
        }
        Err(e) => {
            eprintln!("获取词条列表失败: {e:?}");
        }
    }

    // 模糊搜索词条
    println!("=== 模糊搜索词条 ===");
    let search_request = EntitySearchRequest {
        query: "API".to_string(),
        page_size: Some(5),
        ..Default::default()
    };

    match client
        .lingo
        .entity
        .search_entities(search_request, None)
        .await
    {
        Ok(response) => {
            if let Some(search_data) = response.data {
                println!("搜索词条成功：");
                for result in &search_data.results.items {
                    println!("  - 词条ID: {}", result.entity.id);
                    println!("    主名称: {:?}", result.entity.main_keys);
                    if let Some(score) = result.score {
                        println!("    匹配分数: {score}");
                    }
                    println!();
                }
            }
        }
        Err(e) => {
            eprintln!("搜索词条失败: {e:?}");
        }
    }

    // 精准匹配词条
    println!("=== 精准匹配词条 ===");
    let match_request = EntityMatchRequest {
        word: "API".to_string(),
        repo_id: None,
    };

    match client
        .lingo
        .entity
        .match_entities(match_request, None)
        .await
    {
        Ok(response) => {
            if let Some(match_data) = response.data {
                println!("精准匹配成功：");
                for result in &match_data.results {
                    println!("  - 词条ID: {}", result.entity.id);
                    println!("    主名称: {:?}", result.entity.main_keys);
                    println!("    匹配词: {}", result.matched_word);
                    println!();
                }
            }
        }
        Err(e) => {
            eprintln!("精准匹配失败: {e:?}");
        }
    }

    // 词条高亮
    println!("=== 词条高亮 ===");
    let highlight_request = EntityHighlightRequest {
        text: "这是一个包含API和SDK的文本示例".to_string(),
        repo_id: None,
    };

    match client
        .lingo
        .entity
        .highlight_entities(highlight_request, None)
        .await
    {
        Ok(response) => {
            if let Some(highlight_data) = response.data {
                println!("词条高亮成功：");
                println!("  原始文本: {}", highlight_data.result.text);
                println!("  高亮范围:");
                for range in &highlight_data.result.ranges {
                    println!(
                        "    - 位置: {} - {}, 词条ID: {}",
                        range.start, range.end, range.entity_id
                    );
                }
            }
        }
        Err(e) => {
            eprintln!("词条高亮失败: {e:?}");
        }
    }

    Ok(())
}
