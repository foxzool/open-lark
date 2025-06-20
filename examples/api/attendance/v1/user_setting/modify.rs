#![allow(clippy::field_reassign_with_default)]

use dotenvy::dotenv;
use open_lark::{prelude::LarkClient, service::attendance::v1::models::ModifyUserSettingRequest};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID must be set");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET must be set");

    let client = LarkClient::builder(&app_id, &app_secret).build();

    // 构建修改用户人脸识别信息请求
    let mut req = ModifyUserSettingRequest::default();
    req.employee_type = "employee_id".to_string();
    req.user_id = "employee_123".to_string(); // 用户ID
    req.face_key_open = Some(true); // 开启人脸识别打卡
    req.face_live_need_action = Some(true); // 开启活体检测
    req.face_downgrade = Some(false); // 关闭人脸识别降级

    println!("发送修改用户人脸识别信息请求...");
    println!("用户ID: {}", req.user_id);
    println!("开启人脸识别: {:?}", req.face_key_open);
    println!("开启活体检测: {:?}", req.face_live_need_action);

    match client.attendance.v1.user_setting.modify(req, None).await {
        Ok(resp) => {
            println!("✅ 修改用户人脸识别信息成功!");
            if let Some(data) = resp.data {
                let user_setting = &data.user_setting;
                println!("👤 用户设置信息:");
                println!("  用户ID: {}", user_setting.user_id);
                if let Some(face_key_open) = user_setting.face_key_open {
                    println!(
                        "  人脸识别打卡: {}",
                        if face_key_open {
                            "已开启"
                        } else {
                            "已关闭"
                        }
                    );
                }
                if let Some(face_key) = &user_setting.face_key {
                    println!("  人脸识别照片Key: {}", face_key);
                }
                if let Some(face_live_need_action) = user_setting.face_live_need_action {
                    println!(
                        "  活体检测: {}",
                        if face_live_need_action {
                            "已开启"
                        } else {
                            "已关闭"
                        }
                    );
                }
                if let Some(face_downgrade) = user_setting.face_downgrade {
                    println!(
                        "  人脸识别降级: {}",
                        if face_downgrade {
                            "已开启"
                        } else {
                            "已关闭"
                        }
                    );
                }
                if let Some(create_time) = &user_setting.create_time {
                    println!("  创建时间: {}", create_time);
                }
                if let Some(update_time) = &user_setting.update_time {
                    println!("  更新时间: {}", update_time);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 修改用户人脸识别信息失败: {:?}", e);
        }
    }

    Ok(())
}
