use dotenvy::dotenv;
use open_lark::{
    prelude::*,
    service::drive::v1::media::{
        BatchGetTmpDownloadUrlRequest, DownloadMediaRequest, UploadMediaRequest,
    },
};
use std::env;

/// 媒体操作示例
///
/// 演示素材的上传、下载和获取临时链接等操作
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从环境变量获取配置
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID 必须设置");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET 必须设置");
    let user_access_token = env::var("USER_ACCESS_TOKEN").expect("USER_ACCESS_TOKEN 必须设置");

    // 创建客户端
    let client = LarkClient::builder(&app_id, &app_secret)
        .with_enable_token_cache(true)
        .build();
    
    let option = RequestOption::builder()
        .user_access_token(user_access_token)
        .build();

    println!("开始媒体操作演示...");

    // 获取根目录token
    let root_token = match client.drive.v1.folder.get_root_folder_meta(Some(option.clone())).await {
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

    // 1. 创建一个简单的测试图片数据（一个小的PNG图片）
    // 这是一个1x1像素的透明PNG图片的二进制数据
    let test_image_data = vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44,
        0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00, 0x00, 0x1F,
        0x15, 0xC4, 0x89, 0x00, 0x00, 0x00, 0x0A, 0x49, 0x44, 0x41, 0x54, 0x78, 0x9C, 0x63, 0x00,
        0x01, 0x00, 0x00, 0x05, 0x00, 0x01, 0x0D, 0x0A, 0x2D, 0xB4, 0x00, 0x00, 0x00, 0x00, 0x49,
        0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
    ];

    // 2. 上传素材
    println!("\n📤 上传测试图片素材...");
    let image_name = format!("测试图片_{}.png", chrono::Utc::now().timestamp());
    let upload_request = UploadMediaRequest::builder()
        .file_name(image_name.clone())
        .parent_token(root_token.clone())
        .size(test_image_data.len() as i32)
        .file(test_image_data.clone())
        .build();

    let media_token = match client.drive.v1.media.upload_all(upload_request, None).await {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 素材上传成功:");
                println!("  - 素材名称: {}", image_name);
                println!("  - 素材Token: {}", data.file_token);
                data.file_token
            } else {
                eprintln!("❌ 上传素材失败：没有返回数据");
                return Ok(());
            }
        }
        Err(e) => {
            eprintln!("❌ 上传素材失败: {}", e);
            return Ok(());
        }
    };

    // 3. 获取素材临时下载链接
    println!("\n🔗 获取素材临时下载链接...");
    let tmp_url_request = BatchGetTmpDownloadUrlRequest::new(vec![media_token.clone()]);

    match client
        .drive
        .v1
        .media
        .batch_get_tmp_download_url(tmp_url_request, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 获取临时下载链接成功:");
                for url_info in data.tmp_download_urls {
                    println!("  - 素材Token: {}", url_info.file_token);
                    println!("  - 临时下载链接: {}", url_info.tmp_download_url);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 获取临时下载链接失败: {}", e);
        }
    }

    // 4. 下载素材
    println!("\n📥 下载素材...");
    let download_request = DownloadMediaRequest::new(media_token.clone());

    match client.drive.v1.media.download(download_request, None).await {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 素材下载成功:");
                println!("  - 下载数据大小: {} 字节", data.data.len());

                // 验证下载的数据是否与原始数据一致
                if data.data == test_image_data {
                    println!("  - ✅ 数据完整性验证通过");
                } else {
                    println!("  - ❌ 数据完整性验证失败");
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 下载素材失败: {}", e);
        }
    }

    // 5. 演示分片上传（对于大文件）
    println!("\n📦 演示分片上传过程（模拟）...");

    // 创建一个稍大的测试文件数据（重复上面的PNG数据）
    let large_file_data: Vec<u8> = test_image_data.repeat(100); // 约6KB
    let large_file_name = format!("大文件测试_{}.png", chrono::Utc::now().timestamp());

    println!("  - 模拟大文件名称: {}", large_file_name);
    println!("  - 模拟大文件大小: {} 字节", large_file_data.len());
    println!("  - 分片上传步骤:");
    println!("    1. 预上传：获取上传ID和分片信息");
    println!("    2. 分片上传：逐个上传文件块");
    println!("    3. 完成上传：合并所有分片");

    // 这里只是演示流程，实际的分片上传需要根据文件大小动态调整
    // 由于这是演示，我们不实际执行分片上传，只显示流程

    println!("\n🎉 媒体操作演示完成！");
    println!("\n📋 演示总结:");
    println!("  ✅ 素材上传");
    println!("  ✅ 获取临时下载链接");
    println!("  ✅ 素材下载");
    println!("  ✅ 数据完整性验证");
    println!("  📖 分片上传流程说明");

    Ok(())
}
