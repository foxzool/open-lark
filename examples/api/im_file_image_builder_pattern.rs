/// IM文件和图片服务Builder模式示例
///
/// 这个示例展示了飞书IM文件和图片服务的现代化Builder模式使用方法，
/// 包括文件上传下载、图片上传下载等操作的统一API调用方式。
use dotenvy::dotenv;
use open_lark::{
    client::LarkClient,
    core::{constants::AppType, trait_system::ExecutableBuilder},
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

    println!("=== 飞书IM文件和图片服务Builder模式示例 ===\\n");

    // ==========================================
    // 文件服务Builder模式示例
    // ==========================================
    println!("📁 文件服务Builder模式示例");
    println!("展示: 文件上传、下载的Builder模式用法\\n");

    // 示例文件数据（实际使用时从文件读取）
    let test_file_data = b"Hello, this is a test file content!".to_vec();

    // 方式1: 传统API调用
    println!("📋 传统方式文件上传:");
    match client
        .im
        .v1
        .file
        .create("txt", "test_file.txt", test_file_data.clone(), None)
        .await
    {
        Ok(response) => {
            println!("✅ 传统方式文件上传成功");
            println!("   文件Key: {}", response.file_key);

            // 使用文件key下载文件
            println!("\\n📋 传统方式文件下载:");
            match client.im.v1.file.get(&response.file_key, None).await {
                Ok(file_data) => {
                    println!("✅ 传统方式文件下载成功");
                    println!("   文件大小: {} 字节", file_data.data.len());
                }
                Err(e) => {
                    println!("❌ 传统方式文件下载失败: {e}");
                }
            }
        }
        Err(e) => {
            println!("❌ 传统方式文件上传失败: {e}");
            println!("   这可能是因为权限问题或测试环境限制");
        }
    }

    println!();

    // 方式2: 现代Builder模式
    println!("🏗️  现代Builder模式文件操作:");

    // 文件上传Builder
    let upload_result = client
        .im
        .v1
        .file
        .upload_builder()
        .file_type("txt")
        .file_name("builder_test_file.txt")
        .file_data(test_file_data.clone())
        .execute(&client.im.v1.file)
        .await;

    match upload_result {
        Ok(response) => {
            println!("✅ Builder模式文件上传成功");
            println!("   文件Key: {}", response.file_key);

            // 文件下载Builder
            println!("\\n🏗️  Builder模式文件下载:");
            let download_result = client
                .im
                .v1
                .file
                .download_builder()
                .file_key(&response.file_key)
                .execute(&client.im.v1.file)
                .await;

            match download_result {
                Ok(file_data) => {
                    println!("✅ Builder模式文件下载成功");
                    println!("   文件大小: {} 字节", file_data.data.len());

                    // 验证文件内容
                    if file_data.data == test_file_data {
                        println!("✅ 文件内容验证通过");
                    } else {
                        println!("⚠️  文件内容验证失败");
                    }
                }
                Err(e) => {
                    println!("❌ Builder模式文件下载失败: {e}");
                }
            }
        }
        Err(e) => {
            println!("❌ Builder模式文件上传失败: {e}");
            println!("   这可能是因为权限问题或测试环境限制");
        }
    }

    println!();

    // ==========================================
    // 图片服务Builder模式示例
    // ==========================================
    println!("🖼️  图片服务Builder模式示例");
    println!("展示: 图片上传、下载的Builder模式用法\\n");

    // 示例图片数据（实际使用时从图片文件读取）
    let test_image_data = create_test_image_data();

    // 方式1: 传统API调用
    println!("📋 传统方式图片上传:");
    match client
        .im
        .v1
        .image
        .create("png", test_image_data.clone(), None)
        .await
    {
        Ok(response) => {
            println!("✅ 传统方式图片上传成功");
            println!("   图片Key: {}", response.image_key);

            // 使用图片key下载图片
            println!("\\n📋 传统方式图片下载:");
            match client.im.v1.image.get(&response.image_key, None).await {
                Ok(image_data) => {
                    println!("✅ 传统方式图片下载成功");
                    println!("   图片大小: {} 字节", image_data.data.len());
                }
                Err(e) => {
                    println!("❌ 传统方式图片下载失败: {e}");
                }
            }
        }
        Err(e) => {
            println!("❌ 传统方式图片上传失败: {e}");
            println!("   这可能是因为权限问题或测试环境限制");
        }
    }

    println!();

    // 方式2: 现代Builder模式
    println!("🏗️  现代Builder模式图片操作:");

    // 图片上传Builder
    let image_upload_result = client
        .im
        .v1
        .image
        .upload_builder()
        .image_type("png")
        .image_data(test_image_data.clone())
        .execute(&client.im.v1.image)
        .await;

    match image_upload_result {
        Ok(response) => {
            println!("✅ Builder模式图片上传成功");
            println!("   图片Key: {}", response.image_key);

            // 图片下载Builder
            println!("\\n🏗️  Builder模式图片下载:");
            let image_download_result = client
                .im
                .v1
                .image
                .download_builder()
                .image_key(&response.image_key)
                .execute(&client.im.v1.image)
                .await;

            match image_download_result {
                Ok(image_data) => {
                    println!("✅ Builder模式图片下载成功");
                    println!("   图片大小: {} 字节", image_data.data.len());

                    // 验证图片数据
                    if image_data.data == test_image_data {
                        println!("✅ 图片内容验证通过");
                    } else {
                        println!("⚠️  图片内容验证失败");
                    }
                }
                Err(e) => {
                    println!("❌ Builder模式图片下载失败: {e}");
                }
            }
        }
        Err(e) => {
            println!("❌ Builder模式图片上传失败: {e}");
            println!("   这可能是因为权限问题或测试环境限制");
        }
    }

    println!();

    // ==========================================
    // Builder模式高级用法
    // ==========================================
    println!("⚡ Builder模式高级用法");
    println!("展示: 条件构建、链式调用、批量操作\\n");

    // 批量文件上传示例
    let test_files = [
        ("txt", "batch_file_1.txt", b"Content of file 1".to_vec()),
        ("txt", "batch_file_2.txt", b"Content of file 2".to_vec()),
        ("txt", "batch_file_3.txt", b"Content of file 3".to_vec()),
    ];

    println!("📦 批量文件上传演示:");
    for (i, (file_type, file_name, file_data)) in test_files.iter().enumerate() {
        let builder = client
            .im
            .v1
            .file
            .upload_builder()
            .file_type(file_type)
            .file_name(file_name)
            .file_data(file_data.clone());

        match builder.execute(&client.im.v1.file).await {
            Ok(response) => {
                println!("✅ 批量文件 {} 上传成功: {}", i + 1, response.file_key);
            }
            Err(e) => {
                println!("❌ 批量文件 {} 上传失败: {}", i + 1, e);
            }
        }
    }

    println!();

    // ==========================================
    // 错误处理最佳实践
    // ==========================================
    println!("🛡️  错误处理最佳实践");
    println!("展示: 统一错误处理、详细错误信息、重试策略\\n");

    // 故意创建一个可能失败的请求（空文件数据）
    let error_demo_result = client
        .im
        .v1
        .file
        .upload_builder()
        .file_type("txt")
        .file_name("error_test.txt")
        .file_data(Vec::new()) // 空文件可能导致错误
        .execute(&client.im.v1.file)
        .await;

    match error_demo_result {
        Ok(response) => {
            println!("✅ 意外成功: {}", response.file_key);
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
                        403 => {
                            println!("   💡 建议: 权限不足，检查应用权限配置");
                        }
                        413 => {
                            println!("   💡 建议: 文件太大，请压缩后重试");
                        }
                        415 => {
                            println!("   💡 建议: 不支持的文件类型");
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
    println!("📚 IM文件和图片服务Builder模式最佳实践:");
    println!("1. 🔄 文件/图片操作推荐使用Builder模式，支持链式调用");
    println!("2. 🔧 Builder模式适用于复杂参数配置和可选参数");
    println!("3. 🛡️  统一错误处理提供详细的操作失败诊断");
    println!("4. ⚡ 类型安全保证确保文件类型和数据格式正确");
    println!("5. 🎯 批量操作时Builder模式提高代码复用性");
    println!("6. 🔍 使用.execute()方法获得一致的异步执行体验");
    println!("7. 📝 实际使用时注意文件大小限制和支持的文件类型");

    Ok(())
}

/// 创建测试图片数据（简单的PNG格式头部）
fn create_test_image_data() -> Vec<u8> {
    // 这是一个最小的PNG文件头部，实际使用时应该从真实图片文件读取
    vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG 签名
        0x00, 0x00, 0x00, 0x0D, // IHDR 长度
        0x49, 0x48, 0x44, 0x52, // IHDR
        0x00, 0x00, 0x00, 0x01, // 宽度: 1
        0x00, 0x00, 0x00, 0x01, // 高度: 1
        0x08, 0x02, 0x00, 0x00, 0x00, // 位深度、颜色类型等
        0x90, 0x77, 0x53, 0xDE, // CRC
        0x00, 0x00, 0x00, 0x00, // IEND 长度
        0x49, 0x45, 0x4E, 0x44, // IEND
        0xAE, 0x42, 0x60, 0x82, // CRC
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use open_lark::core::constants::AppType;

    #[test]
    fn test_file_upload_builder_creation() {
        let client = LarkClient::builder("test_app_id", "test_app_secret")
            .with_app_type(AppType::SelfBuild)
            .build();

        // 测试文件上传Builder创建
        let builder = client.im.v1.file.upload_builder();
        let request = builder
            .file_type("txt")
            .file_name("test.txt")
            .file_data(b"test content".to_vec())
            .build_unvalidated();

        assert_eq!(request.file_type, "txt");
        assert_eq!(request.file_name, "test.txt");
        assert_eq!(request.file_data, b"test content".to_vec());
    }

    #[test]
    fn test_file_download_builder_creation() {
        let client = LarkClient::builder("test_app_id", "test_app_secret")
            .with_app_type(AppType::SelfBuild)
            .build();

        // 测试文件下载Builder创建
        let builder = client.im.v1.file.download_builder();
        let file_key = builder.file_key("test_file_key").build();

        assert_eq!(file_key, "test_file_key");
    }

    #[test]
    fn test_image_upload_builder_creation() {
        let client = LarkClient::builder("test_app_id", "test_app_secret")
            .with_app_type(AppType::SelfBuild)
            .build();

        // 测试图片上传Builder创建
        let builder = client.im.v1.image.upload_builder();
        let (image_type, image_data) = builder
            .image_type("png")
            .image_data(create_test_image_data())
            .build();

        assert_eq!(image_type, "png");
        assert!(!image_data.is_empty());
    }

    #[test]
    fn test_image_download_builder_creation() {
        let client = LarkClient::builder("test_app_id", "test_app_secret")
            .with_app_type(AppType::SelfBuild)
            .build();

        // 测试图片下载Builder创建
        let builder = client.im.v1.image.download_builder();
        let image_key = builder.image_key("test_image_key").build();

        assert_eq!(image_key, "test_image_key");
    }

    #[test]
    fn test_test_image_data_creation() {
        let image_data = create_test_image_data();

        // 验证PNG文件签名
        assert_eq!(
            &image_data[0..8],
            &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]
        );
        assert!(image_data.len() > 20); // 确保有基本的PNG结构
    }
}
