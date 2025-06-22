use open_lark::{prelude::*, service::attendance::v1::models::ListShiftRequest};

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

    println!("=== 查询所有班次接口示例 ===\n");

    // 从环境变量获取分页大小
    let page_size: i32 = std::env::var("PAGE_SIZE")
        .unwrap_or_else(|_| "10".to_string())
        .parse()
        .unwrap_or(10)
        .min(20); // API最大限制为20

    println!("📋 分页查询班次，每页 {} 条记录", page_size);
    println!();

    let mut page_token: Option<String> = None;
    let mut page_num = 1;
    let mut total_count = 0;

    loop {
        let mut request = ListShiftRequest::new().page_size(page_size);

        if let Some(token) = &page_token {
            request = request.page_token(token.clone());
        }

        println!("🔍 查询第 {} 页...", page_num);

        match client.attendance.v1.shift.list(request, None).await {
            Ok(response) => {
                if response.success() {
                    if let Some(data) = response.data {
                        let shift_count = data.shift_list.len();
                        total_count += shift_count;

                        if shift_count == 0 {
                            println!("📭 本页没有班次数据");
                        } else {
                            println!("✅ 第 {} 页找到 {} 个班次:", page_num, shift_count);
                            println!();

                            for (index, shift) in data.shift_list.iter().enumerate() {
                                let global_index = (page_num - 1) * page_size as usize + index + 1;
                                println!("📋 班次 {} :", global_index);
                                println!("   班次ID: {}", shift.shift_id);
                                println!("   班次名称: {}", shift.shift_name);
                                println!("   打卡次数: {}", shift.punch_times);

                                let is_flexible = shift.is_flexible;
                                println!("   是否弹性打卡: {:?}", is_flexible);

                                if is_flexible.unwrap_or(false) {
                                    if let Some(flexible_minutes) = shift.flexible_minutes {
                                        println!("   弹性打卡时间: {} 分钟", flexible_minutes);
                                    }
                                }

                                if let Some(allow_outside_apply) = shift.allow_outside_apply {
                                    println!("   是否允许在家办公: {}", allow_outside_apply);
                                    if allow_outside_apply {
                                        if let Some(limit) = shift.outside_apply_limit {
                                            println!("   在家办公限制: {} 次", limit);
                                        }
                                    }
                                }

                                if let Some(create_time) = &shift.create_time {
                                    println!("   创建时间: {}", create_time);
                                }

                                if index < shift_count - 1 {
                                    println!("   ────────────────────────────");
                                }
                            }
                        }

                        println!();
                        println!("📊 分页信息:");
                        println!("   当前页: {}", page_num);
                        println!("   当前页记录数: {}", shift_count);
                        println!("   累计记录数: {}", total_count);
                        println!("   是否还有更多: {}", data.has_more);

                        if data.has_more {
                            page_token = data.page_token;
                            page_num += 1;
                            println!("   继续查询下一页...\n");

                            // 为了演示，可以在这里添加延迟
                            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
                        } else {
                            println!("   ✅ 所有班次查询完毕!");
                            break;
                        }
                    } else {
                        println!("⚠️ 响应成功但无数据");
                        break;
                    }
                } else {
                    println!("❌ 查询失败: {} - {}", response.code(), response.msg());
                    if let Some(err) = response.err() {
                        println!("   错误详情: {:?}", err);
                    }
                    break;
                }
            }
            Err(e) => {
                eprintln!("❌ 请求失败: {:?}", e);
                break;
            }
        }
    }

    println!("\n📈 查询总结:");
    println!("   总共查询了 {} 页", page_num);
    println!("   总共找到 {} 个班次", total_count);

    println!("\n💡 使用说明:");
    println!("   1. 设置环境变量 PAGE_SIZE 指定每页记录数 (默认10，最大20)");
    println!("   2. 运行: PAGE_SIZE=5 cargo run --example attendance_shift_list");
    println!("   3. 该示例会自动遍历所有页面直到查询完毕");

    println!("\n=== 示例结束 ===");
    Ok(())
}
