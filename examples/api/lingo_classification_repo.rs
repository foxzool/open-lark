use dotenvy::dotenv;
use open_lark::{
    prelude::*,
    service::lingo::{classification::*, repo::*},
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

    // 获取词库列表
    println!("=== 获取词库列表 ===");
    let repo_request = RepoListRequest {
        page_size: Some(10),
        ..Default::default()
    };

    match client.lingo.repo.list_repos(repo_request, None).await {
        Ok(response) => {
            if let Some(repo_data) = response.data {
                println!("获取词库列表成功：");
                for repo in &repo_data.repos.items {
                    println!("  - 词库ID: {}", repo.id);
                    println!("    词库名称: {}", repo.name);
                    if let Some(description) = &repo.description {
                        println!("    描述: {description}");
                    }
                    if let Some(repo_type) = &repo.repo_type {
                        println!("    类型: {repo_type}");
                    }
                    println!();
                }

                // 如果有词库，获取分类列表
                if let Some(first_repo) = repo_data.repos.items.first() {
                    println!("=== 获取分类列表 ===");
                    let classification_request = ClassificationListRequest {
                        repo_id: Some(first_repo.id.clone()),
                        page_size: Some(10),
                        ..Default::default()
                    };

                    match client
                        .lingo
                        .classification
                        .list_classifications(classification_request, None)
                        .await
                    {
                        Ok(response) => {
                            println!("获取分类列表成功：");
                            if let Some(classification_data) = response.data {
                                for classification in &classification_data.classifications.items {
                                    println!("  - 分类ID: {}", classification.id);
                                    println!("    分类名称: {}", classification.name);
                                    if let Some(father_id) = &classification.father_id {
                                        println!("    父分类ID: {father_id}");
                                    }
                                    println!();
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("获取分类列表失败: {e:?}");
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("获取词库列表失败: {e:?}");
        }
    }

    // 获取全部分类（不指定词库）
    println!("=== 获取全部分类 ===");
    let all_classification_request = ClassificationListRequest {
        page_size: Some(20),
        ..Default::default()
    };

    match client
        .lingo
        .classification
        .list_classifications(all_classification_request, None)
        .await
    {
        Ok(response) => {
            println!("获取全部分类成功：");
            if let Some(classification_data) = response.data {
                for classification in &classification_data.classifications.items {
                    println!("  - 分类ID: {}", classification.id);
                    println!("    分类名称: {}", classification.name);
                    if let Some(father_id) = &classification.father_id {
                        println!("    父分类ID: {father_id}");
                    }
                    println!();
                }
            }
        }
        Err(e) => {
            eprintln!("获取全部分类失败: {e:?}");
        }
    }

    Ok(())
}
