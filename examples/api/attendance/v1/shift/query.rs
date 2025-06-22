use open_lark::{prelude::*, service::attendance::v1::models::QueryShiftRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    env_logger::init();

    let app_id = std::env::var("APP_ID").expect("APP_ID is required");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET is required");

    // 创建客户端
    let client = LarkClient::builder(&app_id, &app_secret)
        .with_enable_token_cache(true)
        .build();

    println!("=== 按名称查询班次接口示例 ===\n");

    // 从环境变量获取要查询的班次名称
    let shift_name = std::env::var("SHIFT_NAME").unwrap_or_else(|_| {
        println!("⚠️ 未设置 SHIFT_NAME 环境变量，使用默认名称");
        println!("   请设置 SHIFT_NAME 环境变量为实际的班次名称");
        "标准班次".to_string()
    });

    println!("🔍 搜索班次名称: \"{}\"", shift_name);

    let request = QueryShiftRequest::new("employee_id", &shift_name);

    match client.attendance.v1.shift.query(request, None).await {
        Ok(response) => {
            if response.success() {
                if let Some(shift) = response.data {
                    if shift.shift_id.is_empty() {
                        println!("📭 未找到匹配的班次");
                    } else {
                        println!("✅ 找到匹配的班次:");
                        println!();

                        println!("📋 班次信息:");
                        println!("   班次ID: {}", shift.shift_id);
                        println!("   班次名称: {}", shift.shift_name);
                        println!("   打卡次数: {}", shift.punch_times);
                        println!("   是否弹性打卡: {:?}", shift.is_flexible);

                        if let Some(flexible_minutes) = shift.flexible_minutes {
                            println!("   弹性打卡时间: {} 分钟", flexible_minutes);
                        }

                        if let Some(allow_outside_apply) = shift.allow_outside_apply {
                            println!("   是否允许在家办公: {}", allow_outside_apply);
                            if allow_outside_apply {
                                if let Some(limit) = shift.outside_apply_limit {
                                    println!("   在家办公限制: {} 次", limit);
                                }
                            }
                        }

                        if let Some(punch_rules) = &shift.punch_time_rule {
                            if !punch_rules.is_empty() {
                                println!("   打卡时间规则:");
                                for (i, rule) in punch_rules.iter().enumerate() {
                                    println!(
                                        "     规则{}: {}上班 {}下班",
                                        i + 1,
                                        rule.on_time,
                                        rule.off_time
                                    );
                                }
                            }
                        }

                        if let Some(create_time) = &shift.create_time {
                            println!("   创建时间: {}", create_time);
                        }
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
    println!("   1. 设置环境变量 SHIFT_NAME 为要搜索的班次名称");
    println!("   2. 运行: SHIFT_NAME=\"班次名称\" cargo run --example attendance_shift_query");
    println!("   3. 支持模糊匹配，可以搜索部分班次名称");

    println!("\n=== 示例结束 ===");
    Ok(())
}
