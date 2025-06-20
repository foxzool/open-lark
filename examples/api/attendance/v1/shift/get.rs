use open_lark::{prelude::*, service::attendance::v1::models::GetShiftRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    env_logger::init();

    let app_id = std::env::var("APP_ID").expect("APP_ID is required");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET is required");

    // 创建客户端
    let client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .with_enable_token_cache(true)
        .build();

    println!("=== 按ID查询班次接口示例 ===\n");

    // 从环境变量获取要查询的班次ID
    let shift_id = std::env::var("SHIFT_ID").unwrap_or_else(|_| {
        println!("⚠️ 未设置 SHIFT_ID 环境变量，使用默认测试ID");
        println!("   请设置 SHIFT_ID 环境变量为实际的班次ID");
        "test_shift_id".to_string()
    });

    println!("🔍 查询班次 ID: {}", shift_id);

    let request = GetShiftRequest::new(&shift_id);

    match client.attendance.v1.shift.get(request, None).await {
        Ok(response) => {
            if response.success() {
                if let Some(shift) = response.data {
                    println!("✅ 班次查询成功!");
                    println!();
                    println!("📋 班次详情:");
                    println!("   班次ID: {}", shift.shift_id);
                    println!("   班次名称: {}", shift.shift_name);
                    println!("   打卡次数: {}", shift.punch_times);
                    println!("   是否弹性打卡: {:?}", shift.is_flexible);

                    if let Some(flexible_minutes) = shift.flexible_minutes {
                        println!("   弹性打卡时间: {} 分钟", flexible_minutes);
                    }

                    if let Some(no_need_off) = shift.no_need_off {
                        println!("   是否需要打下班卡: {}", !no_need_off);
                    }

                    if let Some(allow_outside_apply) = shift.allow_outside_apply {
                        println!("   是否允许在家办公: {}", allow_outside_apply);
                        if let Some(limit) = shift.outside_apply_limit {
                            println!("   在家办公限制: {} 次", limit);
                        }
                    }

                    if let Some(allow_face_punch) = shift.allow_face_punch {
                        println!("   是否开启人脸识别打卡: {}", allow_face_punch);
                    }

                    if let Some(punch_rules) = &shift.punch_time_rule {
                        println!("   打卡时间规则:");
                        for (i, rule) in punch_rules.iter().enumerate() {
                            println!(
                                "     规则{}: {}上班 {}下班",
                                i + 1,
                                rule.on_time,
                                rule.off_time
                            );
                            if let Some(late) = rule.late_minutes_as_late {
                                println!("       晚到{}分钟算迟到", late);
                            }
                            if let Some(early) = rule.early_minutes_as_early {
                                println!("       早退{}分钟算早退", early);
                            }
                        }
                    }

                    if let Some(create_time) = &shift.create_time {
                        println!("   创建时间: {}", create_time);
                    }
                    if let Some(update_time) = &shift.update_time {
                        println!("   更新时间: {}", update_time);
                    }
                } else {
                    println!("⚠️ 响应成功但无数据");
                }
            } else {
                println!("❌ 查询失败: {} - {}", response.code(), response.msg());
                if let Some(err) = response.err() {
                    println!("   错误详情: {:?}", err);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 请求失败: {:?}", e);
        }
    }

    println!("\n💡 使用说明:");
    println!("   1. 设置环境变量 SHIFT_ID 为要查询的班次ID");
    println!("   2. 运行: SHIFT_ID=your_shift_id cargo run --example attendance_shift_get");

    println!("\n=== 示例结束 ===");
    Ok(())
}
