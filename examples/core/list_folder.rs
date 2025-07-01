use open_lark::core::trait_system::ExecutableBuilder;
/// 文件夹列表示例
///
/// 这个示例演示如何使用飞书SDK获取文件夹中的文件和子文件夹列表。
///
/// 使用方法：
/// cargo run --example list_folder
///
/// 环境变量：
/// APP_ID=your_app_id
/// APP_SECRET=your_app_secret
/// USER_ACCESS_TOKEN=your_user_access_token
/// FOLDER_TOKEN=target_folder_token (可选，默认列出根文件夹)
use open_lark::prelude::*;

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

    println!("📁 飞书文件夹列表示例");
    println!("{}", "=".repeat(50));

    // 获取目标文件夹token
    let folder_token = get_target_folder(&client).await?;

    // 列出文件夹内容
    list_folder_contents(&client, &folder_token).await?;

    // 演示分页获取
    list_folder_with_pagination(&client, &folder_token).await?;

    Ok(())
}

/// 获取目标文件夹token
async fn get_target_folder(client: &LarkClient) -> Result<String, Box<dyn std::error::Error>> {
    // 优先使用环境变量指定的文件夹
    if let Ok(folder_token) = std::env::var("FOLDER_TOKEN") {
        println!("📂 使用指定文件夹: {folder_token}");
        return Ok(folder_token);
    }

    println!("📂 获取根文件夹...");

    // 获取根文件夹
    match client.drive.v1.folder.get_root_folder_meta(None).await {
        Ok(response) => {
            if let Some(data) = &response.data {
                println!("✅ 根文件夹获取成功: {}", data.token);
                Ok(data.token.clone())
            } else {
                Err("无法获取根文件夹信息".into())
            }
        }
        Err(e) => {
            println!("❌ 获取根文件夹失败: {e:?}");
            Err(e.into())
        }
    }
}

/// 列出文件夹内容
async fn list_folder_contents(
    client: &LarkClient,
    folder_token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📋 获取文件夹内容...");
    println!("   文件夹Token: {folder_token}");

    // 使用增强Builder模式列出文件
    match open_lark::service::cloud_docs::drive::v1::folder::ListFilesRequest::builder()
        .folder_token(folder_token)
        .page_size(20)
        .order_by("created_time") // 按创建时间排序
        .direction("DESC") // 降序
        .execute(&client.drive.v1.folder)
        .await
    {
        Ok(response) => {
            if let Some(data) = &response.data {
                println!("✅ 文件列表获取成功!");
                println!("   总文件数: {}", data.files.len());
                println!("   是否有更多: {}", data.has_more);

                if !data.files.is_empty() {
                    println!("\n📄 文件和文件夹列表:");
                    for (index, file) in data.files.iter().enumerate() {
                        let file_type_icon = match file.file_type.as_str() {
                            "folder" => "📁",
                            "docx" => "📝",
                            "sheet" => "📊",
                            "bitable" => "🗃️",
                            "pdf" => "📄",
                            "image" => "🖼️",
                            "video" => "🎥",
                            "audio" => "🎵",
                            _ => "📄",
                        };

                        println!(
                            "   {}. {} {} ({})",
                            index + 1,
                            file_type_icon,
                            file.name,
                            file.file_type
                        );

                        println!("      Token: {}", file.token);
                        if let Some(created_time) = &file.created_time {
                            println!("      创建时间: {created_time}");
                        }
                        if let Some(modified_time) = &file.modified_time {
                            println!("      修改时间: {modified_time}");
                        }

                        if file.file_type != "folder" {
                            if let Some(size) = file.size {
                                println!("      文件大小: {}", format_file_size(size));
                            }
                        }

                        if let Some(owner) = &file.owner_id {
                            println!("      所有者: {owner}");
                        }

                        println!(); // 空行分隔
                    }
                } else {
                    println!("📭 文件夹为空");
                }

                if data.has_more {
                    println!("💡 提示: 还有更多文件可以通过分页获取");
                    if let Some(next_page_token) = &data.page_token {
                        println!("   下一页Token: {next_page_token}");
                    }
                }
            } else {
                println!("⚠️ 请求成功，但未返回文件数据");
            }
        }
        Err(e) => {
            println!("❌ 获取文件列表失败: {e:?}");
            println!("\n💡 常见错误解决方案:");
            println!("   1. 检查用户访问令牌权限");
            println!("   2. 确认文件夹Token是否正确");
            println!("   3. 验证是否有文件夹访问权限");
            return Err(e.into());
        }
    }

    Ok(())
}

/// 分页获取文件夹内容
async fn list_folder_with_pagination(
    client: &LarkClient,
    folder_token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📖 演示分页获取文件列表...");

    let mut page_token: Option<String> = None;
    let mut page_count = 0;
    let mut total_files = 0;

    loop {
        page_count += 1;
        println!("\n📄 获取第 {page_count} 页...");

        let mut request_builder =
            open_lark::service::cloud_docs::drive::v1::folder::ListFilesRequest::builder()
                .folder_token(folder_token)
                .page_size(5) // 较小的页面大小用于演示分页
                .order_by("modified_time")
                .direction("ASC");

        // 如果有分页token，添加到请求中
        if let Some(token) = &page_token {
            request_builder = request_builder.page_token(token);
        }

        match request_builder.execute(&client.drive.v1.folder).await {
            Ok(response) => {
                if let Some(data) = &response.data {
                    let page_files = data.files.len();
                    total_files += page_files;

                    println!("   本页文件数: {page_files}");
                    println!("   累计文件数: {total_files}");

                    // 显示本页文件名
                    for file in &data.files {
                        let file_type = match file.file_type.as_str() {
                            "folder" => "文件夹",
                            "docx" => "文档",
                            "sheet" => "表格",
                            "bitable" => "多维表格",
                            _ => "文件",
                        };
                        println!("     - {} ({})", file.name, file_type);
                    }

                    // 检查是否还有更多页面
                    if data.has_more {
                        page_token = data.page_token.clone();
                        println!("   → 还有更多页面，继续获取...");

                        // 为了演示，限制最大页数
                        if page_count >= 3 {
                            println!("   ⏹️ 演示限制：最多显示3页");
                            break;
                        }
                    } else {
                        println!("   ✅ 已获取所有文件");
                        break;
                    }
                } else {
                    println!("   ⚠️ 本页无数据");
                    break;
                }
            }
            Err(e) => {
                println!("   ❌ 第{page_count}页获取失败: {e:?}");
                break;
            }
        }
    }

    println!("\n📊 分页获取总结:");
    println!("   总页数: {page_count}");
    println!("   总文件数: {total_files}");

    Ok(())
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
