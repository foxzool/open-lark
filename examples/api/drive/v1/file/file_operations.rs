use dotenv::dotenv;
use open_lark::prelude::*;
use open_lark::service::drive::v1::file::{
    CreateFileRequest, GetFileMetaRequest, SearchFilesRequest, DeleteFileRequest,
    CopyFileRequest, CreateFileShortcutRequest,
};
use std::env;
use tracing::info;

/// 文件操作综合示例
///
/// 演示文件的创建、搜索、复制、删除等操作
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

    info!("开始文件操作演示...");

    // 获取根目录token
    let root_token = match client.drive.v1.folder.get_root_folder_meta(None).await {
        Ok(response) => {
            if let Some(data) = response.data {
                data.token
            } else {
                eprintln!("❌ 获取根目录失败：没有返回数据");
                return Ok(());
            }
        }
        Err(e) => {
            eprintln!("❌ 获取根目录失败: {}", e);
            return Ok(());
        }
    };

    println!("📁 根目录Token: {}", root_token);

    // 1. 创建一个新文档
    println!("\n📝 创建新文档...");
    let doc_name = format!("测试文档_{}", chrono::Utc::now().timestamp());
    let create_request = CreateFileRequest::new(
        doc_name.clone(),
        "docx", // 飞书文档类型
        root_token.clone()
    );

    let doc_token = match client.drive.v1.file.create_file(create_request, None).await {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 创建文档成功:");
                println!("  - 文档名称: {}", doc_name);
                println!("  - 文档Token: {}", data.token);
                println!("  - 文档URL: {}", data.url);
                data.token
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

    // 2. 获取文档元数据
    println!("\n📊 获取文档元数据...");
    let meta_request = GetFileMetaRequest::new(vec![(doc_token.clone(), "docx".to_string())]);
    
    match client.drive.v1.file.get_file_meta(meta_request, None).await {
        Ok(response) => {
            if let Some(data) = response.data {
                for meta in data.metas {
                    println!("✅ 文档元数据:");
                    println!("  - Token: {}", meta.doc_token);
                    println!("  - 类型: {}", meta.doc_type);
                    println!("  - 标题: {}", meta.title);
                    println!("  - 拥有者ID: {}", meta.owner_id);
                    println!("  - 创建时间: {}", meta.create_time);
                    println!("  - 更新时间: {}", meta.update_time);
                    if let Some(url) = meta.url {
                        println!("  - URL: {}", url);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 获取文档元数据失败: {}", e);
        }
    }

    // 3. 复制文档
    println!("\n📋 复制文档...");
    let copy_name = format!("{}_副本", doc_name);
    let copy_request = CopyFileRequest::new(
        doc_token.clone(),
        copy_name.clone(),
        root_token.clone()
    );

    let copied_doc_token = match client.drive.v1.file.copy_file(copy_request, None).await {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 复制文档成功:");
                println!("  - 副本名称: {}", copy_name);
                println!("  - 副本Token: {}", data.token);
                println!("  - 副本URL: {}", data.url);
                Some(data.token)
            } else {
                eprintln!("❌ 复制文档失败：没有返回数据");
                None
            }
        }
        Err(e) => {
            eprintln!("❌ 复制文档失败: {}", e);
            None
        }
    };

    // 4. 创建快捷方式
    println!("\n🔗 创建文档快捷方式...");
    let shortcut_name = format!("{}_快捷方式", doc_name);
    let shortcut_request = CreateFileShortcutRequest::new(
        "docx",
        doc_token.clone(),
        shortcut_name.clone(),
        root_token.clone()
    );

    let shortcut_token = match client.drive.v1.file.create_file_shortcut(shortcut_request, None).await {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 创建快捷方式成功:");
                println!("  - 快捷方式名称: {}", shortcut_name);
                println!("  - 快捷方式Token: {}", data.token);
                println!("  - 快捷方式URL: {}", data.url);
                Some(data.token)
            } else {
                eprintln!("❌ 创建快捷方式失败：没有返回数据");
                None
            }
        }
        Err(e) => {
            eprintln!("❌ 创建快捷方式失败: {}", e);
            None
        }
    };

    // 5. 搜索文件
    println!("\n🔍 搜索文件...");
    let search_request = SearchFilesRequest::new("测试")
        .with_count(10);

    match client.drive.v1.file.search_files(search_request, None).await {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 搜索结果 (关键词: '测试'):");
                for (i, file) in data.files.iter().enumerate() {
                    println!("  文件 {}:", i + 1);
                    println!("    - 名称: {}", file.name);
                    println!("    - Token: {}", file.token);
                    println!("    - 类型: {}", file.file_type);
                    println!("    - 拥有者ID: {}", file.owner_id);
                    println!("    - URL: {}", file.url);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 搜索文件失败: {}", e);
        }
    }

    // 6. 清理：删除创建的文件
    println!("\n🧹 清理创建的文件...");

    // 删除快捷方式
    if let Some(token) = shortcut_token {
        let delete_request = DeleteFileRequest::new(token);
        match client.drive.v1.file.delete_file(delete_request, None).await {
            Ok(_) => println!("✅ 快捷方式删除成功"),
            Err(e) => eprintln!("❌ 删除快捷方式失败: {}", e),
        }
    }

    // 删除副本
    if let Some(token) = copied_doc_token {
        let delete_request = DeleteFileRequest::new(token);
        match client.drive.v1.file.delete_file(delete_request, None).await {
            Ok(_) => println!("✅ 副本文档删除成功"),
            Err(e) => eprintln!("❌ 删除副本失败: {}", e),
        }
    }

    // 删除原文档
    let delete_request = DeleteFileRequest::new(doc_token);
    match client.drive.v1.file.delete_file(delete_request, None).await {
        Ok(_) => println!("✅ 原文档删除成功"),
        Err(e) => eprintln!("❌ 删除原文档失败: {}", e),
    }

    println!("\n🎉 文件操作演示完成！");

    Ok(())
}