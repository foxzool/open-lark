#![allow(clippy::field_reassign_with_default)]

use dotenvy::dotenv;
use open_lark::{prelude::LarkClient, service::attendance::v1::models::UploadUserPhotoRequest};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID must be set");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET must be set");

    let client = LarkClient::builder(&app_id, &app_secret).build();

    // 读取示例照片文件（这里使用一个虚拟的照片数据）
    // 在实际使用中，你需要替换为真实的照片文件路径
    let photo_data = create_sample_photo_data();

    // 构建上传用户人脸识别照片请求
    let mut req = UploadUserPhotoRequest::default();
    req.employee_type = "employee_id".to_string();
    req.user_id = "employee_123".to_string(); // 用户ID
    req.photo_data = photo_data;
    req.photo_name = "face_photo.jpg".to_string();

    println!("发送上传用户人脸识别照片请求...");
    println!("用户ID: {}", req.user_id);
    println!("照片文件名: {}", req.photo_name);
    println!("照片数据大小: {} bytes", req.photo_data.len());

    match client
        .attendance
        .v1
        .user_setting
        .upload_photo(req, None)
        .await
    {
        Ok(resp) => {
            println!("✅ 上传用户人脸识别照片成功!");
            if let Some(data) = resp.data {
                println!("📸 上传结果:");
                println!("  人脸识别照片Key: {}", data.face_key);
                println!("  💡 提示: 你可以使用这个 face_key 在修改用户设置时设置人脸识别照片");
            }
        }
        Err(e) => {
            eprintln!("❌ 上传用户人脸识别照片失败: {:?}", e);
            eprintln!("💡 提示: 确保照片格式正确（JPEG/PNG）且大小适中");
        }
    }

    Ok(())
}

/// 创建示例照片数据（实际使用中应该读取真实的照片文件）
fn create_sample_photo_data() -> Vec<u8> {
    // 这里返回一个简单的示例数据
    // 在实际应用中，你应该使用 std::fs::read() 来读取真实的照片文件
    // 例如: std::fs::read("path/to/face_photo.jpg")?

    println!("⚠️  注意: 这是一个示例，使用了虚拟的照片数据");
    println!("   在实际使用中，请替换为真实的照片文件，例如:");
    println!("   let photo_data = std::fs::read(\"path/to/face_photo.jpg\")?;");

    // 返回一个最小的 JPEG 文件头作为示例
    vec![
        0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01, 0x01, 0x01, 0x00,
        0x48, 0x00, 0x48, 0x00, 0x00, 0xFF, 0xD9,
    ]
}

// 以下是读取真实照片文件的示例代码（注释掉）
// fn read_photo_from_file(file_path: &str) -> Result<Vec<u8>, std::io::Error> {
// std::fs::read(file_path)
// }
//
// 使用示例:
// let photo_data = read_photo_from_file("examples/assets/face_photo.jpg")?;
