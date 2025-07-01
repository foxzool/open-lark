use open_lark::core::trait_system::ExecutableBuilder;
/// 文件下载示例
///
/// 这个示例演示如何使用飞书SDK下载云空间中的文件。
///
/// 使用方法：
/// cargo run --example download_file
///
/// 环境变量：
/// APP_ID=your_app_id
/// APP_SECRET=your_app_secret
/// USER_ACCESS_TOKEN=your_user_access_token
/// FILE_TOKEN=target_file_token (可选，如果不提供会列出文件供选择)
use open_lark::prelude::*;
use std::{fs, io::Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenvy::dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("APP_ID environment variable not set");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET environment variable not set");
    let _user_access_token = std::env::var("USER_ACCESS_TOKEN")
        .expect("USER_ACCESS_TOKEN environment variable not set (required for file operations)");

    // 创建客户端
    let client = LarkClient::builder(&app_id, &app_secret)
        .with_enable_token_cache(true)
        .build();

    println!("📥 飞书文件下载示例");
    println!("{}", "=".repeat(50));

    // 获取目标文件token
    let file_token = get_target_file(&client).await?;

    // 下载文件
    download_file(&client, &file_token).await?;

    Ok(())
}

/// 获取目标文件token
async fn get_target_file(client: &LarkClient) -> Result<String, Box<dyn std::error::Error>> {
    // 优先使用环境变量指定的文件
    if let Ok(file_token) = std::env::var("FILE_TOKEN") {
        println!("📄 使用指定文件: {file_token}");
        return Ok(file_token);
    }

    println!("📄 未指定文件token，列出根文件夹中的文件供选择...");

    // 获取根文件夹
    let root_folder = match client.drive.v1.folder.get_root_folder_meta(None).await {
        Ok(response) => {
            if let Some(data) = response.data {
                data.token
            } else {
                return Err("无法获取根文件夹信息".into());
            }
        }
        Err(e) => return Err(e.into()),
    };

    // 列出根文件夹中的文件
    match open_lark::service::cloud_docs::drive::v1::folder::ListFilesRequest::builder()
        .folder_token(&root_folder)
        .page_size(10)
        .order_by("modified_time")
        .direction("DESC")
        .execute(&client.drive.v1.folder)
        .await
    {
        Ok(response) => {
            if let Some(data) = &response.data {
                let downloadable_files: Vec<_> = data
                    .files
                    .iter()
                    .filter(|file| file.file_type != "folder") // 过滤掉文件夹
                    .collect();

                if downloadable_files.is_empty() {
                    return Err("根文件夹中没有可下载的文件".into());
                }

                println!("\n📋 可下载的文件列表:");
                for (index, file) in downloadable_files.iter().enumerate() {
                    println!("   {}. {} ({})", index + 1, file.name, file.file_type);
                    println!("      Token: {}", file.token);
                    if let Some(size) = file.size {
                        println!("      大小: {}", format_file_size(size));
                    }
                    println!();
                }

                // 选择第一个文件进行演示
                let first_file = downloadable_files[0];
                println!("🎯 自动选择第一个文件进行下载演示: {}", first_file.name);
                Ok(first_file.token.clone())
            } else {
                Err("无法获取文件列表".into())
            }
        }
        Err(e) => {
            println!("❌ 获取文件列表失败: {e:?}");
            Err("请通过 FILE_TOKEN 环境变量指定要下载的文件".into())
        }
    }
}

/// 下载文件
async fn download_file(
    client: &LarkClient,
    file_token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n⬇️ 下载文件...");
    println!("   文件Token: {file_token}");

    // 使用增强Builder模式下载文件
    match open_lark::service::cloud_docs::drive::v1::files::DownloadRequest::builder()
        .file_token(file_token)
        .execute(&client.drive.v1.files)
        .await
    {
        Ok(response) => {
            println!("✅ 文件下载成功!");

            // 获取文件数据
            let file_data = &response.body;
            println!("   下载大小: {} 字节", file_data.len());

            // 生成本地文件名（使用时间戳避免冲突）
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs();
            let local_filename = format!("downloaded_file_{timestamp}.bin");

            // 保存到本地文件
            save_file_to_local(&local_filename, file_data).await?;

            // 尝试检测文件类型并提供更好的文件名
            detect_and_rename_file(&local_filename, file_data).await?;
        }
        Err(e) => {
            println!("❌ 文件下载失败: {e:?}");
            println!("\n💡 常见错误解决方案:");
            println!("   1. 检查用户访问令牌权限");
            println!("   2. 确认文件Token是否正确");
            println!("   3. 验证是否有文件下载权限");
            println!("   4. 检查文件是否存在且未被删除");
            return Err(e.into());
        }
    }

    Ok(())
}

/// 保存文件到本地
async fn save_file_to_local(filename: &str, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = fs::File::create(filename)?;
    file.write_all(data)?;
    file.flush()?;

    println!("   💾 文件已保存到: {filename}");
    println!(
        "   📁 当前目录: {}",
        std::env::current_dir()?.to_string_lossy()
    );

    Ok(())
}

/// 检测文件类型并重命名
async fn detect_and_rename_file(
    original_filename: &str,
    data: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    // 简单的文件类型检测（基于文件头）
    let (extension, file_type) = detect_file_type(data);

    if !extension.is_empty() {
        let new_filename = original_filename.replace(".bin", &format!(".{extension}"));

        // 重命名文件
        if let Err(e) = fs::rename(original_filename, &new_filename) {
            println!("   ⚠️ 重命名文件失败: {e}");
        } else {
            println!("   🔄 文件已重命名为: {new_filename}");
            println!("   📄 检测到文件类型: {file_type}");
        }
    }

    Ok(())
}

/// 简单的文件类型检测
fn detect_file_type(data: &[u8]) -> (String, String) {
    if data.len() < 4 {
        return ("".to_string(), "未知类型".to_string());
    }

    // 检查常见的文件头
    match &data[0..4] {
        [0x50, 0x4B, 0x03, 0x04] | [0x50, 0x4B, 0x05, 0x06] | [0x50, 0x4B, 0x07, 0x08] => {
            // ZIP格式（包括DOCX、XLSX等）
            if data.len() > 30 {
                // 进一步检测是否是Office文档
                let content = String::from_utf8_lossy(data);
                if content.contains("word/") {
                    return ("docx".to_string(), "Word文档".to_string());
                } else if content.contains("xl/") {
                    return ("xlsx".to_string(), "Excel表格".to_string());
                } else if content.contains("ppt/") {
                    return ("pptx".to_string(), "PowerPoint演示文稿".to_string());
                }
            }
            ("zip".to_string(), "ZIP压缩文件".to_string())
        }
        [0x25, 0x50, 0x44, 0x46] => ("pdf".to_string(), "PDF文档".to_string()),
        [0xFF, 0xD8, 0xFF, _] => ("jpg".to_string(), "JPEG图片".to_string()),
        [0x89, 0x50, 0x4E, 0x47] => ("png".to_string(), "PNG图片".to_string()),
        [0x47, 0x49, 0x46, 0x38] => ("gif".to_string(), "GIF图片".to_string()),
        _ => {
            // 检查是否是文本文件
            if data.iter().take(1024).all(|&b| b < 128 || b == 0) {
                // 可能是文本文件
                if data.starts_with(b"{") || data.starts_with(b"[") {
                    ("json".to_string(), "JSON文件".to_string())
                } else {
                    ("txt".to_string(), "文本文件".to_string())
                }
            } else {
                ("".to_string(), "未知类型".to_string())
            }
        }
    }
}

/// 格式化文件大小显示
fn format_file_size(size: i64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = size as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", size as i64, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}
