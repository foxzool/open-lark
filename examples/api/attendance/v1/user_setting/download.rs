#![allow(clippy::field_reassign_with_default)]

use dotenvy::dotenv;
use open_lark::{prelude::LarkClient, service::attendance::v1::models::DownloadUserPhotoRequest};
use std::{env, fs};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID must be set");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET must be set");

    let client = LarkClient::builder(&app_id, &app_secret).build();

    // 构建下载用户人脸识别照片请求
    let mut req = DownloadUserPhotoRequest::default();
    req.employee_type = "employee_id".to_string();
    req.user_id = "employee_123".to_string(); // 用户ID
    req.face_key = "face_photo_key_123".to_string(); // 人脸识别照片Key（从上传接口或查询接口获得）

    println!("发送下载用户人脸识别照片请求...");
    println!("用户ID: {}", req.user_id);
    println!("照片Key: {}", req.face_key);

    match client
        .attendance
        .v1
        .user_setting
        .download_photo(req, None)
        .await
    {
        Ok(photo_data) => {
            println!("✅ 下载用户人脸识别照片成功!");
            println!("📸 照片数据大小: {} bytes", photo_data.len());

            // 保存照片到本地文件
            let output_path = "downloaded_face_photo.jpg";
            match fs::write(output_path, &photo_data) {
                Ok(_) => {
                    println!("💾 照片已保存到: {}", output_path);
                    println!("💡 提示: 你可以使用图片查看器打开这个文件查看下载的照片");
                }
                Err(e) => {
                    eprintln!("❌ 保存照片文件失败: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 下载用户人脸识别照片失败: {:?}", e);
            eprintln!("💡 提示: 请确保 face_key 正确，并且用户确实有人脸识别照片");
        }
    }

    Ok(())
}
