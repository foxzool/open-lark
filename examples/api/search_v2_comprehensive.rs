use dotenvy::dotenv;
use open_lark::{
    prelude::*,
    service::search::v2::models::{
        BatchCreateDataItemRequest, CreateDataItemRequest, CreateDataSourceRequest,
        CreateSchemaRequest, ListDataSourceRequest, SearchAppRequest, SearchMessageRequest,
        UpdateDataSourceRequest, UpdateSchemaRequest,
    },
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenv().ok();
    let app_id = env::var("APP_ID").expect("APP_ID not set");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET not set");

    // 创建客户端
    let client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .with_enable_token_cache(true)
        .build();

    println!("=== 搜索 v2 API 综合示例 ===");

    // === 套件搜索示例 ===
    println!("\n🔍 1. 套件搜索示例");

    // 示例1: 搜索消息
    println!("\n1.1 搜索消息");
    let search_message_req = SearchMessageRequest {
        query: "飞书".to_string(),
        page_size: Some(10),
        page_token: None,
    };

    match client
        .search
        .v2
        .suite_search
        .search_message(search_message_req, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 搜索消息成功");
                println!("找到 {} 条消息", data.search_result.items.len());
                for (i, item) in data.search_result.items.iter().take(3).enumerate() {
                    println!(
                        "  {}. 标题: {}, 内容: {}",
                        i + 1,
                        item.title.as_ref().unwrap_or(&"无标题".to_string()),
                        item.content
                            .as_ref()
                            .unwrap_or(&"无内容".to_string())
                            .chars()
                            .take(50)
                            .collect::<String>()
                    );
                }
            } else {
                println!("✅ 搜索消息成功，但未返回数据");
            }
        }
        Err(e) => println!("❌ 搜索消息失败: {e:?}"),
    }

    // 示例2: 搜索应用
    println!("\n1.2 搜索应用");
    let search_app_req = SearchAppRequest {
        query: "日历".to_string(),
        page_size: Some(5),
        page_token: None,
    };

    match client
        .search
        .v2
        .suite_search
        .search_app(search_app_req, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 搜索应用成功");
                println!("找到 {} 个应用", data.search_result.items.len());
                for (i, item) in data.search_result.items.iter().enumerate() {
                    println!(
                        "  {}. 应用: {}, URL: {}",
                        i + 1,
                        item.title.as_ref().unwrap_or(&"无标题".to_string()),
                        item.url.as_ref().unwrap_or(&"无链接".to_string())
                    );
                }
            } else {
                println!("✅ 搜索应用成功，但未返回数据");
            }
        }
        Err(e) => println!("❌ 搜索应用失败: {e:?}"),
    }

    // === 搜索连接器示例 ===
    println!("\n📊 2. 搜索连接器示例");

    // 示例3: 数据源管理
    println!("\n2.1 数据源管理");

    // 创建数据源
    let create_datasource_req = CreateDataSourceRequest {
        name: "测试数据源".to_string(),
        description: Some("这是一个用于测试的数据源".to_string()),
        config: Some(serde_json::json!({
            "type": "database",
            "connection": "postgresql://localhost:5432/test"
        })),
    };

    let data_source_id = match client
        .search
        .v2
        .data_source
        .create(create_datasource_req, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 创建数据源成功");
                let ds_id = data.data_source.id.clone().unwrap_or_default();
                println!("数据源ID: {ds_id}");
                println!("数据源名称: {}", data.data_source.name.unwrap_or_default());
                ds_id
            } else {
                println!("✅ 创建数据源成功，但未返回详细信息");
                "demo_data_source_id".to_string()
            }
        }
        Err(e) => {
            println!("❌ 创建数据源失败: {e:?}");
            "demo_data_source_id".to_string()
        }
    };

    // 获取数据源列表
    let list_req = Some(ListDataSourceRequest {
        page_size: Some(10),
        page_token: None,
    });

    match client.search.v2.data_source.list(list_req, None).await {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 获取数据源列表成功");
                println!("数据源总数: {}", data.items.len());
                for (i, ds) in data.items.iter().enumerate() {
                    println!(
                        "  {}. ID: {}, 名称: {}, 状态: {}",
                        i + 1,
                        ds.id.as_ref().unwrap_or(&"无ID".to_string()),
                        ds.name.as_ref().unwrap_or(&"无名称".to_string()),
                        ds.status.as_ref().unwrap_or(&"未知".to_string())
                    );
                }
            } else {
                println!("✅ 获取数据源列表成功，但未返回数据");
            }
        }
        Err(e) => println!("❌ 获取数据源列表失败: {e:?}"),
    }

    // 示例4: 数据范式管理
    println!("\n2.2 数据范式管理");

    let create_schema_req = CreateSchemaRequest {
        name: "文档范式".to_string(),
        description: Some("用于文档类型的数据范式".to_string()),
        definition: serde_json::json!({
            "type": "object",
            "properties": {
                "title": {"type": "string"},
                "content": {"type": "string"},
                "tags": {"type": "array", "items": {"type": "string"}}
            },
            "required": ["title", "content"]
        }),
    };

    let schema_id = match client
        .search
        .v2
        .schema
        .create(&data_source_id, create_schema_req, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 创建数据范式成功");
                let s_id = data.schema.id.clone().unwrap_or_default();
                println!("范式ID: {s_id}");
                println!("范式名称: {}", data.schema.name.unwrap_or_default());
                s_id
            } else {
                println!("✅ 创建数据范式成功，但未返回详细信息");
                "demo_schema_id".to_string()
            }
        }
        Err(e) => {
            println!("❌ 创建数据范式失败: {e:?}");
            "demo_schema_id".to_string()
        }
    };

    // 示例5: 数据项管理
    println!("\n2.3 数据项管理");

    // 创建单个数据项
    let create_item_req = CreateDataItemRequest {
        id: "doc_001".to_string(),
        title: Some("飞书使用指南".to_string()),
        content: Some("这是一份详细的飞书使用指南，包含了各种功能的使用方法...".to_string()),
        url: Some("https://docs.feishu.cn/guide".to_string()),
        properties: Some(serde_json::json!({
            "tags": ["指南", "教程", "飞书"],
            "category": "文档",
            "priority": "high"
        })),
    };

    match client
        .search
        .v2
        .data_item
        .create(&data_source_id, create_item_req, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 创建数据项成功");
                println!("数据项ID: {}", data.data_item.id.unwrap_or_default());
                println!("数据项标题: {}", data.data_item.title.unwrap_or_default());
            } else {
                println!("✅ 创建数据项成功，但未返回详细信息");
            }
        }
        Err(e) => println!("❌ 创建数据项失败: {e:?}"),
    }

    // 批量创建数据项
    let batch_create_req = BatchCreateDataItemRequest {
        items: vec![
            CreateDataItemRequest {
                id: "doc_002".to_string(),
                title: Some("飞书API开发指南".to_string()),
                content: Some("完整的飞书API开发教程和最佳实践...".to_string()),
                url: Some("https://docs.feishu.cn/api".to_string()),
                properties: Some(serde_json::json!({"tags": ["API", "开发"]})),
            },
            CreateDataItemRequest {
                id: "doc_003".to_string(),
                title: Some("飞书机器人开发".to_string()),
                content: Some("如何开发飞书机器人应用...".to_string()),
                url: Some("https://docs.feishu.cn/bot".to_string()),
                properties: Some(serde_json::json!({"tags": ["机器人", "开发"]})),
            },
        ],
    };

    match client
        .search
        .v2
        .data_item
        .batch_create(&data_source_id, batch_create_req, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 批量创建数据项成功");
                println!("成功创建 {} 个数据项", data.success_items.len());
                for item in &data.success_items {
                    println!(
                        "  - {}: {}",
                        item.id.as_ref().unwrap_or(&"无ID".to_string()),
                        item.title.as_ref().unwrap_or(&"无标题".to_string())
                    );
                }
            } else {
                println!("✅ 批量创建数据项成功，但未返回详细信息");
            }
        }
        Err(e) => println!("❌ 批量创建数据项失败: {e:?}"),
    }

    // 示例6: 更新操作
    println!("\n2.4 更新操作示例");

    // 更新数据源
    let update_datasource_req = UpdateDataSourceRequest {
        name: Some("更新后的测试数据源".to_string()),
        description: Some("数据源描述已更新".to_string()),
        config: None,
    };

    match client
        .search
        .v2
        .data_source
        .patch(&data_source_id, update_datasource_req, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 更新数据源成功");
                println!("更新后名称: {}", data.data_source.name.unwrap_or_default());
            } else {
                println!("✅ 更新数据源成功，但未返回详细信息");
            }
        }
        Err(e) => println!("❌ 更新数据源失败: {e:?}"),
    }

    // 更新数据范式
    let update_schema_req = UpdateSchemaRequest {
        name: Some("更新后的文档范式".to_string()),
        description: Some("范式描述已更新".to_string()),
        definition: Some(serde_json::json!({
            "type": "object",
            "properties": {
                "title": {"type": "string"},
                "content": {"type": "string"},
                "tags": {"type": "array", "items": {"type": "string"}},
                "author": {"type": "string"}
            },
            "required": ["title", "content", "author"]
        })),
    };

    match client
        .search
        .v2
        .schema
        .patch(&data_source_id, &schema_id, update_schema_req, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 更新数据范式成功");
                println!("更新后名称: {}", data.schema.name.unwrap_or_default());
            } else {
                println!("✅ 更新数据范式成功，但未返回详细信息");
            }
        }
        Err(e) => println!("❌ 更新数据范式失败: {e:?}"),
    }

    // 示例7: 查询操作
    println!("\n2.5 查询操作示例");

    // 获取单个数据源
    match client
        .search
        .v2
        .data_source
        .get(&data_source_id, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 获取数据源详情成功");
                println!("ID: {}", data.data_source.id.unwrap_or_default());
                println!("名称: {}", data.data_source.name.unwrap_or_default());
                println!("描述: {}", data.data_source.description.unwrap_or_default());
            } else {
                println!("✅ 获取数据源详情成功，但未返回数据");
            }
        }
        Err(e) => println!("❌ 获取数据源详情失败: {e:?}"),
    }

    // 获取数据项详情
    match client
        .search
        .v2
        .data_item
        .get(&data_source_id, "doc_001", None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 获取数据项详情成功");
                println!("ID: {}", data.data_item.id.unwrap_or_default());
                println!("标题: {}", data.data_item.title.unwrap_or_default());
                println!(
                    "内容长度: {} 字符",
                    data.data_item
                        .content
                        .as_ref()
                        .map(|c| c.len())
                        .unwrap_or(0)
                );
            } else {
                println!("✅ 获取数据项详情成功，但未返回数据");
            }
        }
        Err(e) => println!("❌ 获取数据项详情失败: {e:?}"),
    }

    // 获取数据范式详情
    match client
        .search
        .v2
        .schema
        .get(&data_source_id, &schema_id, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 获取数据范式详情成功");
                println!("ID: {}", data.schema.id.unwrap_or_default());
                println!("名称: {}", data.schema.name.unwrap_or_default());
                println!("描述: {}", data.schema.description.unwrap_or_default());
            } else {
                println!("✅ 获取数据范式详情成功，但未返回数据");
            }
        }
        Err(e) => println!("❌ 获取数据范式详情失败: {e:?}"),
    }

    // === 清理资源示例 ===
    println!("\n🗑️ 3. 清理资源示例");

    // 删除数据项
    match client
        .search
        .v2
        .data_item
        .delete(&data_source_id, "doc_001", None)
        .await
    {
        Ok(_) => println!("✅ 删除数据项成功"),
        Err(e) => println!("❌ 删除数据项失败: {e:?}"),
    }

    // 删除数据范式
    match client
        .search
        .v2
        .schema
        .delete(&data_source_id, &schema_id, None)
        .await
    {
        Ok(_) => println!("✅ 删除数据范式成功"),
        Err(e) => println!("❌ 删除数据范式失败: {e:?}"),
    }

    // 删除数据源
    match client
        .search
        .v2
        .data_source
        .delete(&data_source_id, None)
        .await
    {
        Ok(_) => println!("✅ 删除数据源成功"),
        Err(e) => println!("❌ 删除数据源失败: {e:?}"),
    }

    println!("\n=== 搜索 v2 API 综合示例完成 ===");
    println!("\n💡 此示例展示了:");
    println!("  • 套件搜索：消息搜索、应用搜索");
    println!("  • 数据源管理：CRUD操作、列表查询");
    println!("  • 数据范式管理：创建、更新、删除");
    println!("  • 数据项管理：单个创建、批量创建、查询");
    println!("  • 完整的资源生命周期管理");

    Ok(())
}
