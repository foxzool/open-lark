use dotenvy::dotenv;
use open_lark::{
    prelude::*,
    core::req_option::RequestOption,
    service::docs::v1::document::{CreateDocumentRequest, ListDocumentBlocksRequest},
};
use std::env;

/// 文档操作综合示例
///
/// 演示文档的创建、获取信息、获取内容等操作
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从环境变量获取配置
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID 必须设置");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET 必须设置");
    let user_access_token = env::var("USER_ACCESS_TOKEN").expect("USER_ACCESS_TOKEN 必须设置");

    // 创建客户端
    let client = LarkClient::builder(&app_id, &app_secret).build();
    
    // 创建RequestOption以传递用户访问令牌
    let option = Some(RequestOption::builder()
        .user_access_token(&user_access_token)
        .build());

    println!("开始文档操作演示...");

    // 1. 创建新文档
    println!("📝 创建新文档...");
    let doc_title = format!("API测试文档_{}", chrono::Utc::now().timestamp());
    let create_request = CreateDocumentRequest::new(doc_title.clone());

    let document_id = match client.docs.v1.document.create(create_request, option).await {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 文档创建成功:");
                println!("  - 文档ID: {}", data.document_id);
                println!("  - 文档标题: {}", data.title);
                println!("  - 版本ID: {}", data.document_revision_id);
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

    // 2. 获取文档基本信息
    println!("\n📋 获取文档基本信息...");
    match client.docs.v1.document.get(&document_id, option).await {
        Ok(response) => {
            if let Some(data) = response.data {
                let doc = data.document;
                println!("✅ 文档信息:");
                println!("  - 文档ID: {}", doc.document_id);
                println!("  - 文档标题: {}", doc.title);
                println!("  - 版本ID: {}", doc.document_revision_id);
                println!("  - 创建时间: {}", doc.create_time);
                println!("  - 更新时间: {}", doc.update_time);
                println!("  - 创建者ID: {}", doc.creator_id);
                println!("  - 最后编辑者ID: {}", doc.last_editor_id);
            }
        }
        Err(e) => {
            eprintln!("❌ 获取文档信息失败: {}", e);
        }
    }

    // 3. 获取文档纯文本内容
    println!("\n📄 获取文档纯文本内容...");
    match client
        .docs
        .v1
        .document
        .get_raw_content(&document_id, option)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 文档纯文本内容:");
                if data.content.is_empty() {
                    println!("  - 内容: (空文档)");
                } else {
                    println!("  - 内容: {}", data.content);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 获取文档内容失败: {}", e);
        }
    }

    // 4. 获取文档所有块
    println!("\n🧱 获取文档所有块...");
    let list_request = ListDocumentBlocksRequest::new(&document_id).with_page_size(50);

    match client
        .docs
        .v1
        .document
        .list_blocks(list_request, option)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 文档块信息:");
                println!("  - 是否还有更多: {}", data.has_more);
                println!("  - 块数量: {}", data.items.len());

                for (i, block) in data.items.iter().enumerate() {
                    println!("  块 {}:", i + 1);
                    println!("    - 块ID: {}", block.block_id);
                    println!("    - 父块ID: {}", block.parent_id);
                    println!("    - 块类型: {}", block.block_type);
                    println!("    - 块索引: {}", block.index);
                    println!("    - 子块数量: {}", block.children.len());
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 获取文档块失败: {}", e);
        }
    }

    // 5. 演示文档转换功能（如果需要）
    println!("\n🔄 演示文档转换功能...");
    match client
        .docs
        .v1
        .document
        .convert_to_docx(&document_id, option)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 文档转换成功:");
                println!("  - 新文档ID: {}", data.document_id);
                println!("  - 版本ID: {}", data.document_revision_id);
            }
        }
        Err(e) => {
            // 转换可能会失败，这是正常的，因为新文档可能已经是新格式
            println!("ℹ️  文档转换信息: {}", e);
        }
    }

    println!("\n🎉 文档操作演示完成！");
    println!("\n📋 演示总结:");
    println!("  ✅ 创建新文档");
    println!("  ✅ 获取文档基本信息");
    println!("  ✅ 获取文档纯文本内容");
    println!("  ✅ 获取文档所有块");
    println!("  ✅ 演示文档转换功能");

    println!("\n💡 提示:");
    println!("  - 创建的文档ID: {}", document_id);
    println!("  - 可以在飞书中查看和编辑这个文档");
    println!("  - 可以使用文档块API进一步操作文档内容");

    Ok(())
}
