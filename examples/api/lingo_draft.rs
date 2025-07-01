use dotenvy::dotenv;
use open_lark::{prelude::*, service::lingo::draft::*};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID 环境变量未设置");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET 环境变量未设置");

    let client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .build();

    // 创建草稿
    println!("=== 创建草稿 ===");
    let create_request = DraftCreateRequest {
        entity_id: None,
        main_keys: vec!["API接口".to_string(), "应用编程接口".to_string()],
        aliases: Some(vec!["API".to_string()]),
        description:
            "应用程序编程接口（Application Programming Interface）是一套定义、协议和工具的集合。"
                .to_string(),
        classification_id: None,
        outer_info: None,
        related_meta: None,
    };

    match client.lingo.draft.create_draft(create_request, None).await {
        Ok(response) => {
            if let Some(draft_data) = response.data {
                println!("创建草稿成功：");
                let draft = &draft_data.draft;
                println!("  草稿ID: {}", draft.draft_id);
                println!("  主名称: {:?}", draft.main_keys);
                if let Some(aliases) = &draft.aliases {
                    println!("  别名: {aliases:?}");
                }
                println!("  描述: {}", draft.description);

                // 更新草稿
                println!("\n=== 更新草稿 ===");
                let update_request = DraftUpdateRequest {
                main_keys: Some(vec!["API接口".to_string(), "应用程序接口".to_string()]),
                aliases: Some(vec!["API".to_string(), "程序接口".to_string()]),
                description: Some("应用程序编程接口（Application Programming Interface，简称API）是一套定义、协议和工具的集合，用于构建软件应用程序。".to_string()),
                classification_id: None,
                outer_info: None,
                related_meta: None,
            };

                match client
                    .lingo
                    .draft
                    .update_draft(&draft.draft_id, update_request, None)
                    .await
                {
                    Ok(update_response) => {
                        if let Some(update_data) = update_response.data {
                            println!("更新草稿成功：");
                            let updated_draft = &update_data.draft;
                            println!("  草稿ID: {}", updated_draft.draft_id);
                            println!("  主名称: {:?}", updated_draft.main_keys);
                            if let Some(aliases) = &updated_draft.aliases {
                                println!("  别名: {aliases:?}");
                            }
                            println!("  描述: {}", updated_draft.description);
                        }
                    }
                    Err(e) => {
                        eprintln!("更新草稿失败: {e:?}");
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("创建草稿失败: {e:?}");
        }
    }

    Ok(())
}
