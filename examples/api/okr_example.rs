use open_lark::{
    prelude::*,
    service::okr::{
        models::{I18nText, PeriodStatus, ProgressRecordType},
        okr::UserOkrListRequest,
        period::{PeriodCreateRequest, PeriodListRequest},
        period_rule::PeriodRuleListRequest,
        progress_record::ProgressRecordCreateRequest,
        review::ReviewQueryRequest,
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

    println!("=== OKR 服务示例 ===\n");

    // 1. 创建 OKR 周期
    println!("1. 创建 OKR 周期");
    let period_request = PeriodCreateRequest {
        name: I18nText {
            zh_cn: Some("2024 Q1 OKR 周期".to_string()),
            en_us: Some("2024 Q1 OKR Period".to_string()),
            ja_jp: None,
        },
        start_time: 1704067200000, // 2024-01-01 的毫秒时间戳
        end_time: 1711900799000,   // 2024-03-31 的毫秒时间戳
        description: Some(I18nText {
            zh_cn: Some("2024年第一季度OKR目标管理周期".to_string()),
            en_us: Some("2024 Q1 OKR Management Period".to_string()),
            ja_jp: None,
        }),
        ..Default::default()
    };

    match client.okr.period.create_period(period_request, None).await {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 成功创建周期: ID={}", data.period.period_id);
            }
        }
        Err(e) => println!("❌ 创建周期失败: {e:?}"),
    }

    // 2. 查询周期列表
    println!("\n2. 查询 OKR 周期列表");
    let list_request = PeriodListRequest {
        status: Some(PeriodStatus::Active),
        page_size: Some(10),
        ..Default::default()
    };

    match client.okr.period.list_periods(list_request, None).await {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 查询到 {} 个周期", data.periods.items.len());
                for period in data.periods.items.iter().take(3) {
                    let name = period
                        .name
                        .as_ref()
                        .and_then(|n| n.zh_cn.as_ref())
                        .map(|s| s.as_str())
                        .unwrap_or("未命名");
                    println!(
                        "  - {}: {} ({:?} - {:?})",
                        period.period_id, name, period.start_time, period.end_time
                    );
                }
            }
        }
        Err(e) => println!("❌ 查询周期失败: {e:?}"),
    }

    // 3. 查询周期规则
    println!("\n3. 查询周期规则");
    let rule_request = PeriodRuleListRequest {
        page_size: Some(5),
        ..Default::default()
    };

    match client
        .okr
        .period_rule
        .list_period_rules(rule_request, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 查询到 {} 个周期规则", data.rules.items.len());
                for rule in data.rules.items.iter().take(2) {
                    println!("  - 规则ID: {}", rule.rule_id);
                }
            }
        }
        Err(e) => println!("❌ 查询周期规则失败: {e:?}"),
    }

    // 4. 查询用户 OKR
    println!("\n4. 查询用户 OKR 列表");
    let okr_request = UserOkrListRequest {
        user_id: Some("your_user_id".to_string()), // 需要替换为真实的用户ID
        period_id: None,
        page_size: Some(10),
        ..Default::default()
    };

    match client.okr.okr.list_user_okrs(okr_request, None).await {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 查询到 {} 个 OKR", data.okrs.items.len());
                for okr in data.okrs.items.iter().take(2) {
                    println!("  - OKR ID: {}", okr.okr_id);
                    if let Some(objectives) = &okr.objectives {
                        println!("    目标数量: {}", objectives.len());
                    }
                }
            }
        }
        Err(e) => println!("❌ 查询 OKR 失败: {e:?}"),
    }

    // 5. 创建进展记录
    println!("\n5. 创建进展记录");
    let progress_request = ProgressRecordCreateRequest {
        okr_id: "your_okr_id".to_string(), // 需要替换为真实的OKR ID
        content: "本周完成了用户登录模块的开发，目标完成度达到 60%".to_string(),
        record_type: Some(ProgressRecordType::Detail),
        progress_rate: Some(60.0),
        attachment_ids: None,
    };

    match client
        .okr
        .progress_record
        .create_progress_record(progress_request, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!(
                    "✅ 成功创建进展记录: ID={}",
                    data.progress_record.progress_id
                );
            }
        }
        Err(e) => println!("❌ 创建进展记录失败: {e:?}"),
    }

    // 6. 查询复盘信息
    println!("\n6. 查询复盘信息");
    let review_request = ReviewQueryRequest {
        period_id: Some("your_period_id".to_string()), // 需要替换为真实的周期ID
        min_score: Some(3.0),
        page_size: Some(10),
        ..Default::default()
    };

    match client.okr.review.query_reviews(review_request, None).await {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 查询到 {} 个复盘记录", data.reviews.items.len());
                for review in data.reviews.items.iter().take(2) {
                    println!("  - 复盘ID: {}, 评分: {:?}", review.review_id, review.score);
                }
            }
        }
        Err(e) => println!("❌ 查询复盘失败: {e:?}"),
    }

    println!("\n=== OKR 服务示例完成 ===");
    println!("注意: 某些示例需要替换为真实的 ID 才能正常运行");

    Ok(())
}
