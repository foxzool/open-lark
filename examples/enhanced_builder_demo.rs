// 增强Builder模式演示
//
// 这个示例展示了新增的execute()方法，提供更流畅的API体验
//
// 使用方法：
// cargo run --example enhanced_builder_demo

use open_lark::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("需要设置 APP_ID 环境变量");
    let app_secret = std::env::var("APP_SECRET").expect("需要设置 APP_SECRET 环境变量");

    // 创建客户端
    let _client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .build();

    println!("🚀 演示增强Builder模式的使用方法");
    println!();

    // 演示1: 传统方式 vs 新的增强方式
    println!("📁 示例 1: 列出文件夹中的文件");
    println!("{}", "=".repeat(50));

    // 传统方式 (仍然支持)
    println!("传统方式:");
    println!("```rust");
    println!("let req = ListFilesRequest::builder()");
    println!("    .folder_token(\"folder123\")");
    println!("    .page_size(10)");
    println!("    .build();");
    println!("let result = service.list_files(req, None).await?;");
    println!("```");
    println!();

    // 新的增强方式
    println!("增强方式:");
    println!("```rust");
    println!("let result = ListFilesRequest::builder()");
    println!("    .folder_token(\"folder123\")");
    println!("    .page_size(10)");
    println!("    .execute(&service)");
    println!("    .await?;");
    println!("```");
    println!();

    // 实际运行 (如果有有效的folder token的话)
    // let result = ListFilesRequest::builder()
    // .folder_token("your_folder_token_here")
    // .page_size(10)
    // .execute(&client.drive().folder())
    // .await?;
    // println!("文件列表: {:?}", result);

    println!("📤 示例 2: 上传文件");
    println!("{}", "=".repeat(50));

    // 传统方式
    println!("传统方式:");
    println!("```rust");
    println!("let req = UploadAllRequest::builder()");
    println!("    .file_name(\"test.txt\")");
    println!("    .parent_type(\"explorer\")");
    println!("    .parent_node(\"folder_token\")");
    println!("    .size(file_data.len() as i32)");
    println!("    .file(file_data)");
    println!("    .build();");
    println!("let result = service.upload_all(req, None).await?;");
    println!("```");
    println!();

    // 新的增强方式
    println!("增强方式:");
    println!("```rust");
    println!("let result = UploadAllRequest::builder()");
    println!("    .file_name(\"test.txt\")");
    println!("    .parent_type(\"explorer\")");
    println!("    .parent_node(\"folder_token\")");
    println!("    .size(file_data.len() as i32)");
    println!("    .file(file_data)");
    println!("    .execute(&service)");
    println!("    .await?;");
    println!("```");
    println!();

    println!("📥 示例 3: 下载文件");
    println!("{}", "=".repeat(50));

    // 传统方式
    println!("传统方式:");
    println!("```rust");
    println!("let req = DownloadRequest::builder()");
    println!("    .file_token(\"file_token_here\")");
    println!("    .build();");
    println!("let result = service.download(req, None).await?;");
    println!("```");
    println!();

    // 新的增强方式
    println!("增强方式:");
    println!("```rust");
    println!("let result = DownloadRequest::builder()");
    println!("    .file_token(\"file_token_here\")");
    println!("    .execute(&service)");
    println!("    .await?;");
    println!("```");
    println!();

    println!("✨ 新Builder模式的优势:");
    println!("- 减少样板代码 (不需要单独的 .build() 和 service.method() 调用)");
    println!("- 更流畅的API体验");
    println!("- 保持完全向后兼容");
    println!("- 类型安全和IDE自动完成支持");
    println!("- 支持带选项的执行: .execute_with_options(&service, option)");

    Ok(())
}
