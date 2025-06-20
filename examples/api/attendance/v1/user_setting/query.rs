#![allow(clippy::field_reassign_with_default)]

use dotenvy::dotenv;
use open_lark::{prelude::LarkClient, service::attendance::v1::models::QueryUserSettingRequest};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID must be set");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET must be set");

    let client = LarkClient::builder(&app_id, &app_secret).build();

    // 构建批量查询用户人脸识别信息请求
    let mut req = QueryUserSettingRequest::default();
    req.employee_type = "employee_id".to_string();
    req.user_ids = vec![
        "employee_123".to_string(),
        "employee_456".to_string(),
        "employee_789".to_string(),
    ];

    println!("发送批量查询用户人脸识别信息请求...");
    println!("查询用户数量: {}", req.user_ids.len());
    println!("用户ID列表: {:?}", req.user_ids);

    match client.attendance.v1.user_setting.query(req, None).await {
        Ok(resp) => {
            println!("✅ 批量查询用户人脸识别信息成功!");
            if let Some(data) = resp.data {
                println!("找到 {} 个用户的设置信息", data.user_setting_list.len());

                for (index, user_setting) in data.user_setting_list.iter().enumerate() {
                    println!("{}. 👤 用户设置信息:", index + 1);
                    println!("   用户ID: {}", user_setting.user_id);

                    if let Some(face_key_open) = user_setting.face_key_open {
                        println!(
                            "   人脸识别打卡: {}",
                            if face_key_open {
                                "已开启"
                            } else {
                                "已关闭"
                            }
                        );
                    }

                    if let Some(face_key) = &user_setting.face_key {
                        println!("   人脸识别照片Key: {}", face_key);
                    } else {
                        println!("   人脸识别照片Key: 未设置");
                    }

                    if let Some(face_live_need_action) = user_setting.face_live_need_action {
                        println!(
                            "   活体检测: {}",
                            if face_live_need_action {
                                "已开启"
                            } else {
                                "已关闭"
                            }
                        );
                    }

                    if let Some(face_downgrade) = user_setting.face_downgrade {
                        println!(
                            "   人脸识别降级: {}",
                            if face_downgrade {
                                "已开启"
                            } else {
                                "已关闭"
                            }
                        );
                    }

                    if let Some(create_time) = &user_setting.create_time {
                        println!("   创建时间: {}", create_time);
                    }

                    if let Some(update_time) = &user_setting.update_time {
                        println!("   更新时间: {}", update_time);
                    }

                    println!("   ---");
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 批量查询用户人脸识别信息失败: {:?}", e);
        }
    }

    Ok(())
}
