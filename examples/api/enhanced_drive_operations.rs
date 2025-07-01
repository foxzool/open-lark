// 使用增强Builder模式进行云空间操作的示例
//
// 这个示例展示了新的 .execute() 方法如何简化API调用
//
// 运行方式：
// cargo run --example enhanced_drive_operations
//
// 环境变量要求：
// APP_ID=your_app_id
// APP_SECRET=your_app_secret

use open_lark::{core::trait_system::ExecutableBuilder, prelude::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 读取环境变量
    dotenvy::dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("请设置 APP_ID 环境变量");
    let app_secret = std::env::var("APP_SECRET").expect("请设置 APP_SECRET 环境变量");

    // 创建Lark客户端
    let client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .with_enable_token_cache(true)
        .build();

    println!("🚀 演示增强Builder模式在云空间操作中的应用");
    println!("{}", "=".repeat(60));

    // 获取根文件夹元数据
    println!("\n📁 获取根文件夹信息:");
    match client.drive.v1.folder.get_root_folder_meta(None).await {
        Ok(response) => {
            if let Some(data) = &response.data {
                println!("✅ 根文件夹Token: {}", data.token);

                // 使用新的增强Builder模式列出根文件夹中的文件
                println!("\n📋 使用增强Builder列出根文件夹中的文件:");

                // 新的增强方式 - 一行流畅的调用
                match open_lark::service::cloud_docs::drive::v1::folder::ListFilesRequest::builder()
                    .folder_token(&data.token)
                    .page_size(10)
                    .order_by("created_time")
                    .direction("DESC")
                    .execute(&client.drive.v1.folder)
                    .await
                {
                    Ok(files_response) => {
                        if let Some(files_data) = &files_response.data {
                            println!("✅ 成功获取文件列表，共 {} 个文件", files_data.files.len());
                            for (i, file) in files_data.files.iter().enumerate() {
                                println!("   {}. {} ({})", i + 1, file.name, file.file_type);
                            }
                        } else {
                            println!("❌ 响应数据为空");
                        }
                    }
                    Err(e) => println!("❌ 获取文件列表失败: {e}"),
                }
            } else {
                println!("❌ 根文件夹数据为空");
            }
        }
        Err(e) => println!("❌ 获取根文件夹信息失败: {e}"),
    }

    // 对比传统方式的调用量
    println!("\n📊 代码简化对比:");
    println!("传统方式需要:");
    println!("  1. 创建Request::builder()");
    println!("  2. 设置参数");
    println!("  3. 调用.build()");
    println!("  4. 调用service.method(request, option)");
    println!("  5. 等待结果");
    println!();
    println!("增强方式只需要:");
    println!("  1. 创建Request::builder()");
    println!("  2. 设置参数");
    println!("  3. 调用.execute(&service)");
    println!("  4. 等待结果");
    println!("减少了一个步骤，API更加流畅！");

    println!("\n📤 演示文件上传的增强Builder模式:");

    // 创建示例文件数据
    let sample_content = "这是一个使用增强Builder模式上传的测试文件！\n时间: {}";
    let current_time = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S");
    let file_content = sample_content
        .replace("{}", &current_time.to_string())
        .to_string();
    let file_bytes = file_content.into_bytes();

    println!("准备上传文件内容 ({} 字节)", file_bytes.len());

    // 注意：实际上传需要有效的父文件夹token，这里只展示API调用方式
    println!("使用增强Builder的上传调用方式:");
    println!("```rust");
    println!("let result = UploadAllRequest::builder()");
    println!("    .file_name(\"enhanced_builder_test.txt\")");
    println!("    .parent_type(\"explorer\")");
    println!("    .parent_node(\"your_folder_token\")");
    println!("    .size(file_bytes.len() as i32)");
    println!("    .file(file_bytes)");
    println!("    .execute(&client.drive.v1.files)  // 直接执行！");
    println!("    .await?;");
    println!("```");

    println!("\n📥 演示文件下载的增强Builder模式:");
    println!("使用增强Builder的下载调用方式:");
    println!("```rust");
    println!("let file_data = DownloadRequest::builder()");
    println!("    .file_token(\"your_file_token\")");
    println!("    .execute(&client.drive.v1.files)  // 直接执行！");
    println!("    .await?;");
    println!("```");

    println!("\n🎯 增强Builder模式的核心优势:");
    println!("1. ✨ 更少的样板代码");
    println!("2. 🔗 更流畅的方法链");
    println!("3. 🛡️  完全的类型安全");
    println!("4. 🔄 100% 向后兼容");
    println!("5. 💡 更好的IDE支持和自动完成");
    println!("6. ⚡ 零性能开销");

    println!("\n📋 架构说明:");
    println!("- execute() 方法是对现有API的语法糖封装");
    println!("- 底层仍使用相同的ApiRequest和Transport架构");
    println!("- 保持了Command Pattern的设计纯粹性");
    println!("- 无需任何代码生成，维护成本极低");

    Ok(())
}
