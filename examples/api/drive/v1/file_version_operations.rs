use dotenv::dotenv;
use open_lark::prelude::*;
use open_lark::service::drive::v1::file::CreateFileRequest;
use open_lark::service::drive::v1::file_version::{
    CreateVersionRequest, DeleteVersionRequest, GetVersionRequest, ListVersionsRequest,
};
use std::env;
use tracing::info;

/// 文件版本管理示例
///
/// 演示文档版本的创建、查询、删除等操作
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

    info!("开始文件版本管理演示...");

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

    // 1. 首先创建一个测试文档
    println!("\n📝 创建测试文档...");
    let doc_name = format!("版本管理测试文档_{}", chrono::Utc::now().timestamp());
    let create_request = CreateFileRequest::new(doc_name.clone(), "docx", root_token.clone());

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

    // 2. 创建文档版本
    println!("\n📋 创建文档版本...");
    let version_name = "v1.0 - 初始版本";
    let create_version_request = CreateVersionRequest::new(
        doc_token.clone(),
        version_name,
        "docx" // 文档类型
    );

    let version_id = match client
        .drive
        .v1
        .file_version
        .create_version(create_version_request, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 创建版本成功:");
                println!("  - 版本ID: {}", data.version_id);
                println!("  - 版本名称: {}", data.name);
                println!("  - 创建时间: {}", data.create_time);
                println!("  - 创建者ID: {}", data.creator_id);
                data.version_id
            } else {
                eprintln!("❌ 创建版本失败：没有返回数据");
                return Ok(());
            }
        }
        Err(e) => {
            eprintln!("❌ 创建版本失败: {}", e);
            return Ok(());
        }
    };

    // 3. 再创建一个版本
    println!("\n📋 创建第二个文档版本...");
    let version2_name = "v2.0 - 更新版本";
    let create_version2_request = CreateVersionRequest::new(
        doc_token.clone(),
        version2_name,
        "docx"
    );

    let version2_id = match client
        .drive
        .v1
        .file_version
        .create_version(create_version2_request, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 创建第二个版本成功:");
                println!("  - 版本ID: {}", data.version_id);
                println!("  - 版本名称: {}", data.name);
                data.version_id
            } else {
                eprintln!("❌ 创建第二个版本失败：没有返回数据");
                return Ok(());
            }
        }
        Err(e) => {
            eprintln!("❌ 创建第二个版本失败: {}", e);
            return Ok(());
        }
    };

    // 4. 获取版本列表
    println!("\n📜 获取文档版本列表...");
    let list_request = ListVersionsRequest::new(doc_token.clone()).with_page_size(10);

    match client
        .drive
        .v1
        .file_version
        .list_versions(list_request, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 获取版本列表成功:");
                println!("  - 是否还有更多: {}", data.has_more);
                println!("  - 版本数量: {}", data.items.len());
                
                for (i, version) in data.items.iter().enumerate() {
                    println!("  版本 {}:", i + 1);
                    println!("    - ID: {}", version.version_id);
                    println!("    - 名称: {}", version.name);
                    println!("    - 创建时间: {}", version.create_time);
                    println!("    - 创建者ID: {}", version.creator_id);
                    println!("    - 状态: {}", version.status);
                    if let Some(parent_id) = &version.parent_version_id {
                        println!("    - 父版本ID: {}", parent_id);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 获取版本列表失败: {}", e);
        }
    }

    // 5. 获取特定版本信息
    println!("\n🔍 获取特定版本信息...");
    let get_version_request = GetVersionRequest::new(doc_token.clone(), version_id.clone());

    match client
        .drive
        .v1
        .file_version
        .get_version(get_version_request, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                let version = data.version;
                println!("✅ 获取版本信息成功:");
                println!("  - 版本ID: {}", version.version_id);
                println!("  - 版本名称: {}", version.name);
                println!("  - 文档Token: {}", version.obj_token);
                println!("  - 文档类型: {}", version.obj_type);
                println!("  - 创建者ID: {}", version.creator_id);
                println!("  - 创建时间: {}", version.create_time);
                println!("  - 状态: {}", version.status);
            }
        }
        Err(e) => {
            eprintln!("❌ 获取版本信息失败: {}", e);
        }
    }

    // 6. 删除一个版本
    println!("\n🗑️  删除第一个版本...");
    let delete_version_request = DeleteVersionRequest::new(doc_token.clone(), version_id.clone());

    match client
        .drive
        .v1
        .file_version
        .delete_version(delete_version_request, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                if data.success {
                    println!("✅ 删除版本成功");
                } else {
                    println!("❌ 删除版本失败");
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 删除版本失败: {}", e);
        }
    }

    // 7. 再次获取版本列表，验证删除结果
    println!("\n📜 验证删除后的版本列表...");
    let list_request2 = ListVersionsRequest::new(doc_token.clone());

    match client
        .drive
        .v1
        .file_version
        .list_versions(list_request2, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 验证版本列表:");
                println!("  - 剩余版本数量: {}", data.items.len());
                
                for (i, version) in data.items.iter().enumerate() {
                    println!("  剩余版本 {}:", i + 1);
                    println!("    - ID: {}", version.version_id);
                    println!("    - 名称: {}", version.name);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 验证版本列表失败: {}", e);
        }
    }

    // 8. 清理：删除测试文档
    println!("\n🧹 清理测试文档...");
    let delete_file_request = open_lark::service::drive::v1::file::DeleteFileRequest::new(doc_token);
    match client
        .drive
        .v1
        .file
        .delete_file(delete_file_request, None)
        .await
    {
        Ok(_) => println!("✅ 测试文档删除成功"),
        Err(e) => eprintln!("❌ 删除测试文档失败: {}", e),
    }

    println!("\n🎉 文件版本管理演示完成！");
    println!("\n📋 演示总结:");
    println!("  ✅ 创建测试文档");
    println!("  ✅ 创建文档版本");
    println!("  ✅ 获取版本列表");
    println!("  ✅ 获取特定版本信息");
    println!("  ✅ 删除版本");
    println!("  ✅ 验证删除结果");
    println!("  ✅ 清理测试文档");

    Ok(())
}