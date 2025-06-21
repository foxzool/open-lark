use dotenv::dotenv;
use open_lark::prelude::*;
use open_lark::service::docs::v1::document::CreateDocumentRequest;
use open_lark::service::docs::v1::document_block::{
    BatchDeleteBlockRequest, BatchUpdateBlockRequest, BlockData, CreateBlockRequest,
    ListChildrenRequest, PatchBlockRequest, UpdateBlockItem,
};
use serde_json::{json, Value};
use std::env;
use tracing::info;

/// 文档块操作综合示例
///
/// 演示文档块的创建、更新、删除等操作
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从环境变量获取配置
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let app_id = env::var("APP_ID").expect("APP_ID 必须设置");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET 必须设置");
    let user_access_token = env::var("USER_ACCESS_TOKEN").expect("USER_ACCESS_TOKEN 必须设置");

    // 创建客户端，使用用户访问凭证
    let client = LarkClient::builder(app_id, app_secret)
        .with_user_access_token(user_access_token)
        .build();

    info!("开始文档块操作演示...");

    // 1. 首先创建一个测试文档
    println!("📝 创建测试文档...");
    let doc_title = format!("块操作测试文档_{}", chrono::Utc::now().timestamp());
    let create_doc_request = CreateDocumentRequest::new(doc_title.clone());

    let document_id = match client.docs.v1.document.create(create_doc_request, None).await {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 文档创建成功: {}", data.document_id);
                data.document_id
            } else {
                eprintln!("❌ 创建文档失败：没有返回数据");
                return Ok(());
            }
        }
        Err(e) => {
            eprintln!("❌ 创建文档失败: {}", e);
            return Ok(());
        }
    };

    // 2. 创建文本块
    println!("\n🧱 创建文本块...");
    
    // 创建段落块的示例数据
    let text_block = BlockData {
        block_type: 2, // 段落类型
        block: json!({
            "paragraph": {
                "elements": [
                    {
                        "text_run": {
                            "content": "这是通过API创建的第一个段落！"
                        }
                    }
                ]
            }
        }),
    };

    let create_block_request = CreateBlockRequest::new(&document_id, vec![text_block]);

    let first_block_id = match client
        .docs
        .v1
        .document_block
        .create(&document_id, create_block_request, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 文本块创建成功:");
                println!("  - 创建块数量: {}", data.blocks.len());
                println!("  - 文档版本: {}", data.document_revision_id);
                
                if let Some(block) = data.blocks.first() {
                    println!("  - 第一个块ID: {}", block.block_id);
                    println!("  - 块类型: {}", block.block_type);
                    println!("  - 块索引: {}", block.index);
                    Some(block.block_id.clone())
                } else {
                    None
                }
            } else {
                None
            }
        }
        Err(e) => {
            eprintln!("❌ 创建文本块失败: {}", e);
            None
        }
    };

    // 3. 添加更多块
    println!("\n➕ 添加更多块...");
    
    let heading_block = BlockData {
        block_type: 1, // 标题类型
        block: json!({
            "heading1": {
                "elements": [
                    {
                        "text_run": {
                            "content": "这是一个标题"
                        }
                    }
                ]
            }
        }),
    };

    let bullet_block = BlockData {
        block_type: 3, // 无序列表
        block: json!({
            "bullet": {
                "elements": [
                    {
                        "text_run": {
                            "content": "这是一个无序列表项"
                        }
                    }
                ]
            }
        }),
    };

    let create_more_blocks = CreateBlockRequest::new(&document_id, vec![heading_block, bullet_block]);

    let additional_block_ids = match client
        .docs
        .v1
        .document_block
        .create(&document_id, create_more_blocks, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 额外块创建成功:");
                println!("  - 创建块数量: {}", data.blocks.len());
                
                let mut block_ids = Vec::new();
                for (i, block) in data.blocks.iter().enumerate() {
                    println!("  - 块 {}: {} (类型: {})", i + 1, block.block_id, block.block_type);
                    block_ids.push(block.block_id.clone());
                }
                block_ids
            } else {
                Vec::new()
            }
        }
        Err(e) => {
            eprintln!("❌ 创建额外块失败: {}", e);
            Vec::new()
        }
    };

    // 4. 获取并显示某个块的详细信息
    if let Some(block_id) = &first_block_id {
        println!("\n🔍 获取块的详细信息...");
        match client
            .docs
            .v1
            .document_block
            .get(&document_id, block_id, None)
            .await
        {
            Ok(response) => {
                if let Some(data) = response.data {
                    let block = data.block;
                    println!("✅ 块详细信息:");
                    println!("  - 块ID: {}", block.block_id);
                    println!("  - 父块ID: {}", block.parent_id);
                    println!("  - 块类型: {}", block.block_type);
                    println!("  - 块索引: {}", block.index);
                    println!("  - 子块数量: {}", block.children.len());
                    println!("  - 块内容: {}", block.block);
                }
            }
            Err(e) => {
                eprintln!("❌ 获取块信息失败: {}", e);
            }
        }
    }

    // 5. 更新块内容
    if let Some(block_id) = &first_block_id {
        println!("\n✏️  更新块内容...");
        
        let update_content = json!({
            "paragraph": {
                "elements": [
                    {
                        "text_run": {
                            "content": "这是更新后的段落内容！时间: "
                        }
                    },
                    {
                        "text_run": {
                            "content": &chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                            "text_element_style": {
                                "bold": true
                            }
                        }
                    }
                ]
            }
        });

        let patch_request = PatchBlockRequest::new(update_content);

        match client
            .docs
            .v1
            .document_block
            .patch(&document_id, block_id, patch_request, None)
            .await
        {
            Ok(response) => {
                if let Some(data) = response.data {
                    println!("✅ 块更新成功:");
                    println!("  - 更新块ID: {}", data.block.block_id);
                    println!("  - 文档版本: {}", data.document_revision_id);
                }
            }
            Err(e) => {
                eprintln!("❌ 更新块失败: {}", e);
            }
        }
    }

    // 6. 批量更新多个块
    if additional_block_ids.len() >= 2 {
        println!("\n🔄 批量更新多个块...");
        
        let updates = vec![
            UpdateBlockItem {
                block_id: additional_block_ids[0].clone(),
                block: json!({
                    "heading1": {
                        "elements": [
                            {
                                "text_run": {
                                    "content": "更新后的标题 - API演示"
                                }
                            }
                        ]
                    }
                }),
            },
            UpdateBlockItem {
                block_id: additional_block_ids[1].clone(),
                block: json!({
                    "bullet": {
                        "elements": [
                            {
                                "text_run": {
                                    "content": "更新后的列表项 - 批量操作成功"
                                }
                            }
                        ]
                    }
                }),
            },
        ];

        let batch_update_request = BatchUpdateBlockRequest::new(updates);

        match client
            .docs
            .v1
            .document_block
            .batch_update(&document_id, batch_update_request, None)
            .await
        {
            Ok(response) => {
                if let Some(data) = response.data {
                    println!("✅ 批量更新成功:");
                    println!("  - 更新块数量: {}", data.blocks.len());
                    println!("  - 文档版本: {}", data.document_revision_id);
                }
            }
            Err(e) => {
                eprintln!("❌ 批量更新失败: {}", e);
            }
        }
    }

    // 7. 获取子块（演示）
    println!("\n👶 获取子块...");
    let list_children_request = ListChildrenRequest::new().with_page_size(10);
    
    match client
        .docs
        .v1
        .document_block
        .list_children(&document_id, &document_id, list_children_request, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 子块信息:");
                println!("  - 是否还有更多: {}", data.has_more);
                println!("  - 子块数量: {}", data.items.len());
                
                for (i, block) in data.items.iter().enumerate() {
                    println!("  - 子块 {}: {} (类型: {})", i + 1, block.block_id, block.block_type);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 获取子块失败: {}", e);
        }
    }

    // 8. 删除部分块（清理）
    if additional_block_ids.len() >= 1 {
        println!("\n🗑️  删除部分块...");
        
        let delete_request = BatchDeleteBlockRequest::new(vec![additional_block_ids[0].clone()]);

        match client
            .docs
            .v1
            .document_block
            .batch_delete(&document_id, delete_request, None)
            .await
        {
            Ok(response) => {
                if let Some(data) = response.data {
                    println!("✅ 块删除成功:");
                    println!("  - 文档版本: {}", data.document_revision_id);
                }
            }
            Err(e) => {
                eprintln!("❌ 删除块失败: {}", e);
            }
        }
    }

    println!("\n🎉 文档块操作演示完成！");
    println!("\n📋 演示总结:");
    println!("  ✅ 创建测试文档");
    println!("  ✅ 创建文本块");
    println!("  ✅ 添加标题和列表块");
    println!("  ✅ 获取块详细信息");
    println!("  ✅ 更新块内容");
    println!("  ✅ 批量更新多个块");
    println!("  ✅ 获取子块信息");
    println!("  ✅ 删除块");
    
    println!("\n💡 提示:");
    println!("  - 文档ID: {}", document_id);
    println!("  - 可以在飞书中查看文档的变化");
    println!("  - 块类型: 1=标题, 2=段落, 3=无序列表等");
    println!("  - 文档版本会随每次操作递增");

    Ok(())
}