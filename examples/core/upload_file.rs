use open_lark::core::trait_system::ExecutableBuilder;
/// 文件上传示例
///
/// 这个示例演示如何使用飞书SDK上传文件到云空间。
///
/// 使用方法：
/// cargo run --example upload_file
///
/// 环境变量：
/// APP_ID=your_app_id
/// APP_SECRET=your_app_secret
/// USER_ACCESS_TOKEN=your_user_access_token
/// FOLDER_TOKEN=target_folder_token (可选，默认使用根文件夹)
use open_lark::prelude::*;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenvy::dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("APP_ID environment variable not set");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET environment variable not set");
    let _user_access_token = std::env::var("USER_ACCESS_TOKEN")
        .expect("USER_ACCESS_TOKEN environment variable not set (required for file operations)");

    // 创建客户端（文件操作需要用户访问令牌）
    let client = LarkClient::builder(&app_id, &app_secret)
        .with_enable_token_cache(true)
        .build();

    println!("📤 飞书文件上传示例");
    println!("{}", "=".repeat(50));

    // 获取根文件夹信息
    let folder_token = get_upload_folder(&client).await?;

    // 创建测试文件并上传
    upload_text_file(&client, &folder_token).await?;

    Ok(())
}

/// 获取上传目标文件夹token
async fn get_upload_folder(client: &LarkClient) -> Result<String, Box<dyn std::error::Error>> {
    // 优先使用环境变量指定的文件夹
    if let Ok(folder_token) = std::env::var("FOLDER_TOKEN") {
        println!("📁 使用指定文件夹: {}", folder_token);
        return Ok(folder_token);
    }

    println!("📁 获取根文件夹信息...");

    // 获取根文件夹元数据
    match client.drive.v1.folder.get_root_folder_meta(None).await {
        Ok(response) => {
            if let Some(data) = &response.data {
                println!("✅ 根文件夹获取成功!");
                println!("   文件夹Token: {}", data.token);
                Ok(data.token.clone())
            } else {
                Err("无法获取根文件夹信息".into())
            }
        }
        Err(e) => {
            println!("❌ 获取根文件夹失败: {:?}", e);
            println!("\n💡 常见错误解决方案:");
            println!("   1. 确保设置了有效的 USER_ACCESS_TOKEN");
            println!("   2. 检查用户访问令牌的权限");
            println!("   3. 确认应用有云空间访问权限");
            Err(e.into())
        }
    }
}

/// 上传文本文件
async fn upload_text_file(
    client: &LarkClient,
    folder_token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📝 创建测试文件并上传...");

    // 创建测试文件内容
    let file_name = format!(
        "飞书SDK测试文件_{}.txt",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs()
    );

    let file_content = format!(
        "飞书SDK文件上传测试\n\
        ==================\n\
        上传时间: {}\n\
        SDK版本: open-lark\n\
        文件编码: UTF-8\n\
        \n\
        这是一个由飞书Rust SDK自动生成的测试文件。\n\
        如果您看到这个文件，说明文件上传功能工作正常！\n\
        \n\
        技术细节:\n\
        - 使用增强Builder模式\n\
        - 支持二进制文件上传\n\
        - 自动计算文件大小\n\
        - 包含完整错误处理\n",
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    );

    let file_data = file_content.as_bytes().to_vec();

    println!("   文件名: {}", file_name);
    println!("   文件大小: {} 字节", file_data.len());
    println!("   目标文件夹: {}", folder_token);

    // 使用增强Builder模式上传文件
    match open_lark::service::cloud_docs::drive::v1::files::UploadAllRequest::builder()
        .file_name(&file_name)
        .parent_type("explorer")
        .parent_node(folder_token)
        .size(file_data.len() as i32)
        .file(file_data)
        .execute(&client.drive.v1.files)
        .await
    {
        Ok(response) => {
            if let Some(data) = &response.data {
                println!("✅ 文件上传成功!");
                println!("   文件Token: {}", data.file_token);

                println!("   💡 文件已上传，可通过Drive API获取详细信息");

                println!("\n💡 提示: 您可以在飞书云空间中查看上传的文件");
            } else {
                println!("⚠️ 上传请求成功，但未返回文件信息");
            }
        }
        Err(e) => {
            println!("❌ 文件上传失败: {:?}", e);
            println!("\n💡 常见错误解决方案:");
            println!("   1. 检查用户访问令牌权限");
            println!("   2. 确认目标文件夹存在且有写入权限");
            println!("   3. 验证文件名格式是否正确");
            println!("   4. 检查文件大小是否超出限制");
            return Err(e.into());
        }
    }

    Ok(())
}

/// 上传本地文件示例（注释掉的代码，可以解除注释使用）
#[allow(dead_code)]
async fn upload_local_file(
    client: &LarkClient,
    folder_token: &str,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📁 上传本地文件: {}", file_path);

    // 读取本地文件
    let file_data = fs::read(file_path)?;
    let file_name = std::path::Path::new(file_path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("unknown_file")
        .to_string();

    println!("   文件名: {}", file_name);
    println!("   文件大小: {} 字节", file_data.len());

    match open_lark::service::cloud_docs::drive::v1::files::UploadAllRequest::builder()
        .file_name(&file_name)
        .parent_type("explorer")
        .parent_node(folder_token)
        .size(file_data.len() as i32)
        .file(file_data)
        .execute(&client.drive.v1.files)
        .await
    {
        Ok(response) => {
            if let Some(data) = &response.data {
                println!("✅ 本地文件上传成功!");
                println!("   文件Token: {}", data.file_token);
            }
        }
        Err(e) => {
            println!("❌ 本地文件上传失败: {:?}", e);
            return Err(e.into());
        }
    }

    Ok(())
}
