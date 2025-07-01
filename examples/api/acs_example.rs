use open_lark::{
    prelude::*,
    service::acs::{
        access_record::AccessRecordListRequest,
        device::DeviceListRequest,
        models::{DeviceStatus, RuleStatus, UserStatus, UserType},
        rule_external::{DeviceBindRequest, RuleCreateOrUpdateRequest},
        user::{FaceImageUploadRequest, UserListRequest, UserPatchRequest},
        visitor::VisitorCreateRequest,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("请设置 APP_ID 环境变量");
    let app_secret = std::env::var("APP_SECRET").expect("请设置 APP_SECRET 环境变量");

    let client = LarkClient::builder(&app_id, &app_secret)
        .with_enable_token_cache(true)
        .build();

    println!("=== 智能门禁服务示例 ===\n");

    // 1. 获取用户列表
    println!("1. 获取门禁用户列表");
    let user_list_request = UserListRequest {
        page_size: Some(10),
        user_type: Some(UserType::Employee),
        status: Some(UserStatus::Active),
        ..Default::default()
    };

    match client.acs.user.list_users(user_list_request, None).await {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 查询到 {} 个用户", data.users.items.len());
                for user in data.users.items.iter().take(3) {
                    println!(
                        "  - 用户ID: {}, 姓名: {}, 类型: {:?}",
                        user.user_id, user.name, user.user_type
                    );
                    if let Some(department) = &user.department {
                        println!("    部门: {department}");
                    }
                }
            }
        }
        Err(e) => println!("❌ 查询用户列表失败: {e:?}"),
    }

    // 2. 修改用户信息
    println!("\n2. 修改用户信息");
    let patch_request = UserPatchRequest {
        name: Some("张三（更新）".to_string()),
        status: Some(UserStatus::Active),
        department: Some("技术部".to_string()),
        phone: Some("13800138000".to_string()),
        ..Default::default()
    };

    match client
        .acs
        .user
        .patch_user("test_user_id", patch_request, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 成功修改用户信息: {}", data.user.name);
                println!("   部门: {:?}", data.user.department);
                println!("   电话: {:?}", data.user.phone);
            }
        }
        Err(e) => println!("❌ 修改用户信息失败: {e:?}"),
    }

    // 3. 上传人脸图片
    println!("\n3. 上传用户人脸图片");
    // 注意：这里使用示例的 base64 数据，实际使用时应该是真实的人脸图片数据
    let sample_face_image = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChwGA60e6kgAAAABJRU5ErkJggg==";

    let upload_request = FaceImageUploadRequest {
        image_content: sample_face_image.to_string(),
        image_format: Some("png".to_string()),
    };

    match client
        .acs
        .user
        .upload_face_image("test_user_id", upload_request, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 成功上传人脸图片: {}", data.success);
                if let Some(image_id) = data.image_id {
                    println!("   图片ID: {image_id}");
                }
            }
        }
        Err(e) => println!("❌ 上传人脸图片失败: {e:?}"),
    }

    // 4. 创建权限组
    println!("\n4. 创建门禁权限组");
    let rule_request = RuleCreateOrUpdateRequest {
        rule_id: None,
        name: "技术部门禁组".to_string(),
        description: Some("技术部员工门禁权限组".to_string()),
        status: Some(RuleStatus::Active),
        user_ids: Some(vec!["test_user_id".to_string()]),
        start_time: Some(1672531200000), // 2023-01-01
        end_time: Some(1704067199000),   // 2023-12-31
    };

    match client
        .acs
        .rule_external
        .create_or_update_rule(rule_request, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 成功创建权限组: {}", data.rule_external.name);
                println!("   权限组ID: {}", data.rule_external.rule_id);
                println!("   状态: {:?}", data.rule_external.status);

                let rule_id = data.rule_external.rule_id;

                // 5. 设备绑定权限组
                println!("\n5. 设备绑定权限组");
                let bind_request = DeviceBindRequest {
                    device_id: "device_001".to_string(),
                    rule_ids: vec![rule_id.clone()],
                    operation: Some("bind".to_string()),
                };

                match client
                    .acs
                    .rule_external
                    .bind_device(bind_request, None)
                    .await
                {
                    Ok(response) => {
                        if let Some(data) = response.data {
                            println!("✅ 成功绑定设备到权限组");
                            println!("   设备ID: {}", data.device_id);
                            println!("   权限组ID列表: {:?}", data.rule_ids);
                        }
                    }
                    Err(e) => println!("❌ 设备绑定权限组失败: {e:?}"),
                }
            }
        }
        Err(e) => println!("❌ 创建权限组失败: {e:?}"),
    }

    // 6. 添加访客
    println!("\n6. 添加访客");
    let visitor_request = VisitorCreateRequest {
        name: "李四".to_string(),
        phone: Some("13900139000".to_string()),
        company: Some("合作伙伴公司".to_string()),
        purpose: Some("商务洽谈".to_string()),
        host_user_id: Some("test_user_id".to_string()),
        start_time: Some(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64,
        ),
        end_time: Some(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64
                + 3600000,
        ), // 1小时后
        device_ids: Some(vec!["device_001".to_string()]),
    };

    match client
        .acs
        .visitor
        .create_visitor(visitor_request, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 成功添加访客: {}", data.visitor.name);
                println!("   访客ID: {}", data.visitor.visitor_id);
                println!("   访问目的: {:?}", data.visitor.purpose);
                println!("   接待人: {:?}", data.visitor.host_name);
            }
        }
        Err(e) => println!("❌ 添加访客失败: {e:?}"),
    }

    // 7. 获取门禁设备列表
    println!("\n7. 获取门禁设备列表");
    let device_request = DeviceListRequest {
        page_size: Some(10),
        status: Some(DeviceStatus::Online),
        ..Default::default()
    };

    match client.acs.device.list_devices(device_request, None).await {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 查询到 {} 个设备", data.devices.items.len());
                for device in data.devices.items.iter().take(3) {
                    println!(
                        "  - 设备ID: {}, 名称: {}, 状态: {:?}",
                        device.device_id, device.name, device.status
                    );
                    if let Some(location) = &device.location {
                        println!("    位置: {location}");
                    }
                }
            }
        }
        Err(e) => println!("❌ 查询设备列表失败: {e:?}"),
    }

    // 8. 获取门禁访问记录
    println!("\n8. 获取门禁访问记录");
    let record_request = AccessRecordListRequest {
        page_size: Some(10),
        start_time: Some(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64
                - 86400000,
        ), // 24小时前
        end_time: Some(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64,
        ),
        ..Default::default()
    };

    match client
        .acs
        .access_record
        .list_access_records(record_request, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 查询到 {} 条访问记录", data.access_records.items.len());
                for record in data.access_records.items.iter().take(3) {
                    println!(
                        "  - 记录ID: {}, 用户: {:?}, 设备: {}",
                        record.record_id, record.user_name, record.device_id
                    );
                    println!(
                        "    访问方式: {:?}, 结果: {:?}",
                        record.access_method, record.result
                    );
                    println!("    访问时间: {} (时间戳)", record.access_time);
                }
            }
        }
        Err(e) => println!("❌ 查询访问记录失败: {e:?}"),
    }

    println!("\n=== 智能门禁服务示例完成 ===");
    println!("注意:");
    println!("1. 本示例使用测试数据，实际使用时请替换为真实的用户ID、设备ID等信息");
    println!("2. 人脸图片上传需要使用真实的人脸图片 base64 数据");
    println!("3. 门禁系统涉及物理安全，请确保严格的权限控制和审计");
    println!("4. 访客管理需要配合实际的门禁设备和管理流程");

    Ok(())
}
