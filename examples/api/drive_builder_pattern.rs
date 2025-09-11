/// Drive服务Builder模式示例
///
/// 这个示例展示了飞书Drive云文档服务的新Builder模式使用方法，
/// 包括文件上传、下载等操作的现代化API调用方式。
use dotenvy::dotenv;
use open_lark::{
    client::LarkClient,
    core::{constants::AppType, trait_system::ExecutableBuilder},
    service::cloud_docs::drive::v1::files::UploadAllRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    // 创建客户端
    let client = LarkClient::builder(
        &std::env::var("APP_ID").expect("APP_ID is required"),
        &std::env::var("APP_SECRET").expect("APP_SECRET is required"),
    )
    .with_app_type(AppType::SelfBuild)
    .build();

    println!("=== 飞书Drive云文档服务Builder模式示例 ===\n");

    // ==========================================
    // 方式一: 传统API调用方式
    // ==========================================
    println!("📋 方式一: 传统API调用方式");
    println!("适用于: 现有代码迁移、简单文件操作\n");

    // 模拟文件内容
    let file_content = b"Hello, Lark Drive! This is a test file.".to_vec();

    let traditional_request = UploadAllRequest::builder()
        .file_name("test_traditional.txt")
        .parent_type("explorer")
        .parent_node("root") // 根目录，实际使用时需要真实的文件夹token
        .size(file_content.len() as i32)
        .file(file_content.clone())
        .build();

    match client
        .cloud_docs
        .drive
        .v1
        .files
        .upload_all(traditional_request, None)
        .await
    {
        Ok(response) => {
            println!("✅ 传统方式文件上传成功");
            println!("   文件Token: {}", response.file_token);
        }
        Err(e) => {
            println!("❌ 传统方式上传失败: {e}");
            println!("   这可能是因为权限问题或测试环境限制");
        }
    }

    println!();

    // ==========================================
    // 方式二: 现代Builder模式 (推荐)
    // ==========================================
    println!("🏗️  方式二: 现代Builder模式 (推荐)");
    println!("适用于: 新代码开发、复杂参数配置、链式调用\n");

    // 使用Builder模式构建上传请求
    let builder_result = client
        .cloud_docs
        .drive
        .v1
        .files
        .upload_all_builder()
        .file_name("test_builder.txt")
        .parent_type("explorer")
        .parent_node("root") // 实际使用时需要真实的文件夹token
        .size(file_content.len() as i32)
        .checksum("optional_checksum") // 可选的校验和
        .file(file_content.clone())
        .execute(&client.cloud_docs.drive.v1.files)
        .await;

    match builder_result {
        Ok(response) => {
            println!("✅ Builder模式文件上传成功");
            println!("   文件Token: {}", response.file_token);

            // 演示文件下载
            println!("\n📥 尝试下载刚上传的文件...");
            let download_result = client
                .cloud_docs
                .drive
                .v1
                .files
                .download_builder()
                .file_token(&response.file_token)
                .execute(&client.cloud_docs.drive.v1.files)
                .await;

            match download_result {
                Ok(binary_response) => {
                    println!("✅ 文件下载成功");
                    println!("   文件名: {}", binary_response.file_name);
                    println!("   下载内容长度: {} bytes", binary_response.body.len());

                    // 将下载的内容转换为字符串显示
                    if let Ok(content) = String::from_utf8(binary_response.body) {
                        println!("   文件内容: {content}");
                    }
                }
                Err(e) => {
                    println!("❌ 文件下载失败: {e}");
                }
            }
        }
        Err(e) => {
            println!("❌ Builder模式上传失败: {e}");
            println!("   这可能是因为权限问题或测试环境限制");
        }
    }

    println!();

    // ==========================================
    // 方式三: Builder模式的高级用法
    // ==========================================
    println!("⚡ 方式三: Builder模式高级用法");
    println!("展示: 条件构建、大文件处理、批量操作\n");

    // 模拟批量文件上传
    let files_to_upload = vec![
        ("document1.txt", "This is document 1 content"),
        ("document2.txt", "This is document 2 content"),
        ("document3.txt", "This is document 3 content"),
    ];

    for (filename, content) in files_to_upload {
        let file_bytes = content.as_bytes().to_vec();

        let mut upload_builder = client
            .cloud_docs
            .drive
            .v1
            .files
            .upload_all_builder()
            .file_name(filename)
            .parent_type("explorer")
            .parent_node("root")
            .size(file_bytes.len() as i32)
            .file(file_bytes.clone());

        // 条件性添加校验和 (针对较大文件)
        if file_bytes.len() > 50 {
            let checksum = format!("checksum_{filename}");
            upload_builder = upload_builder.checksum(checksum);
        }

        // 执行上传
        match upload_builder
            .execute(&client.cloud_docs.drive.v1.files)
            .await
        {
            Ok(response) => {
                println!("✅ 批量上传成功: {filename}");
                println!("   文件Token: {}", response.file_token);
            }
            Err(e) => {
                println!("❌ 批量上传失败 {filename}: {e}");
            }
        }
    }

    println!();

    // ==========================================
    // 错误处理最佳实践
    // ==========================================
    println!("🛡️  错误处理最佳实践");
    println!("展示: 统一错误处理、重试策略、文件验证\n");

    // 故意创建一个可能失败的请求（空文件名）
    let error_demo_result = client
        .cloud_docs
        .drive
        .v1
        .files
        .upload_all_builder()
        .file_name("") // 空文件名可能导致错误
        .parent_type("explorer")
        .parent_node("invalid_token") // 无效的父节点token
        .size(0)
        .file(vec![])
        .execute(&client.cloud_docs.drive.v1.files)
        .await;

    match error_demo_result {
        Ok(response) => {
            println!("✅ 意外成功: {}", response.file_token);
        }
        Err(e) => {
            println!("❌ 预期错误示例:");
            println!("   错误信息: {e}");

            // 使用新的错误处理方法
            use open_lark::core::error::LarkAPIError;
            match &e {
                LarkAPIError::APIError { code, msg, .. } => {
                    println!("   错误码: {code}");
                    println!("   错误消息: {msg}");

                    // 根据错误码决定处理策略
                    match *code {
                        429 => {
                            println!("   💡 建议: 请求频率过高，建议稍后重试");
                        }
                        403 => {
                            println!("   💡 建议: 权限不足，请检查应用权限配置");
                        }
                        400 => {
                            println!("   💡 建议: 请求参数错误，检查文件名和父节点token");
                        }
                        _ => {
                            println!("   💡 建议: 检查网络连接和API配置");
                        }
                    }
                }
                LarkAPIError::DataError(msg) => {
                    println!("   数据错误: {msg}");
                    println!("   💡 建议: 检查文件内容和格式");
                }
                _ => {
                    println!("   其他错误类型");
                    println!("   💡 建议: 查看详细日志获取更多信息");
                }
            }
        }
    }

    println!();

    // ==========================================
    // 最佳实践总结
    // ==========================================
    println!("📚 Drive服务Builder模式最佳实践:");
    println!("1. 🔄 文件上传推荐使用Builder模式，支持可选参数配置");
    println!("2. 🔧 条件性构建适用于不同文件类型和大小的处理");
    println!("3. 🛡️  统一错误处理提供详细的文件操作错误诊断");
    println!("4. ⚡ 链式调用使文件操作代码更简洁易读");
    println!("5. 🎯 批量操作时建议使用Builder模式提高代码复用性");
    println!("6. 🔍 使用.execute()方法获得一致的异步执行体验");
    println!("7. 📁 实际使用时记得使用真实的parent_node token");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use open_lark::service::cloud_docs::drive::v1::files::UploadAllRequest;

    #[test]
    fn test_drive_builder_pattern_creation() {
        let client = LarkClient::builder("test_app_id", "test_app_secret")
            .with_app_type(AppType::SelfBuild)
            .build();

        // 测试Builder创建
        let file_content = b"test content".to_vec();
        let builder = client.cloud_docs.drive.v1.files.upload_all_builder();

        let _request = builder
            .file_name("test.txt")
            .parent_type("explorer")
            .parent_node("test_token")
            .size(file_content.len() as i32)
            .checksum("test_checksum")
            .file(file_content.clone())
            .build();

        // Builder成功创建，由于字段是私有的，无法直接断言值
    }

    #[test]
    fn test_drive_traditional_pattern_creation() {
        let file_content = b"traditional content".to_vec();

        let _request = UploadAllRequest::builder()
            .file_name("traditional.txt")
            .parent_type("explorer")
            .parent_node("root")
            .size(file_content.len() as i32)
            .file(file_content.clone())
            .build();

        // 传统Builder模式成功创建，由于字段是私有的，无法直接断言值
    }
}
