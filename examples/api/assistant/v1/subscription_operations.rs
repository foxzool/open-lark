use open_lark::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let app_id = std::env::var("APP_ID").unwrap();
    let app_secret = std::env::var("APP_SECRET").unwrap();

    let client = LarkClient::builder(app_id, app_secret)
        .with_enable_token_cache(true)
        .build();

    // 测试文档token和类型
    let file_token = "doccnxxxxxx"; // 替换为实际的文档token
    let file_type = open_lark::service::assistant::v1::subscription::FileType::Doc;

    println!("=== 云文档助手订阅管理示例 ===\n");

    // 1. 获取订阅状态
    println!("--- 获取订阅状态 ---");
    let get_request = open_lark::service::assistant::v1::subscription::GetSubscriptionRequest::builder()
        .file_token(file_token)
        .as_doc()
        .build();

    match client.assistant.v1.subscription.get(get_request, None).await {
        Ok(response) => {
            println!("订阅状态获取成功:");
            println!("完整信息: {}", response.info_summary());
            
            let subscription = &response.subscription;
            println!("详细状态:");
            println!("- 状态: {} ({})", subscription.status.description(), subscription.status.color());
            println!("- 是否已订阅: {}", subscription.is_subscribed());
            println!("- 可否激活: {}", subscription.can_activate());
            
            if let Some(start_time) = subscription.start_time_formatted() {
                println!("- 开始时间: {}", start_time);
            }
            
            if let Some(end_time) = subscription.end_time_formatted() {
                println!("- 结束时间: {}", end_time);
            }
            
            if let Some(days) = subscription.duration_days() {
                println!("- 持续时间: {:.1} 天", days);
            }
            
            println!("- 摘要: {}", subscription.summary());
        }
        Err(e) => {
            println!("获取订阅状态失败: {:?}", e);
        }
    }

    // 2. 创建订阅 - 基础配置
    println!("\n--- 创建基础订阅 ---");
    let create_request = open_lark::service::assistant::v1::subscription::CreateSubscriptionRequest::builder()
        .file_token(file_token)
        .as_doc()
        .basic_subscription()
        .build();

    match client.assistant.v1.subscription.create(create_request, None).await {
        Ok(response) => {
            println!("基础订阅创建成功:");
            println!("完整信息: {}", response.info_summary());
            
            if let Some(create_time) = response.create_time_formatted() {
                println!("创建时间: {}", create_time);
            }
            
            if let Some(ref subscription_id) = response.subscription_id {
                println!("订阅ID: {}", subscription_id);
            }
        }
        Err(e) => {
            println!("创建基础订阅失败: {:?}", e);
        }
    }

    // 3. 创建订阅 - 高级配置
    println!("\n--- 创建高级订阅 ---");
    let premium_request = open_lark::service::assistant::v1::subscription::CreateSubscriptionRequest::builder()
        .file_token("sheetxxxxxx") // 使用不同的文档
        .as_sheet()
        .premium_subscription()
        .add_tag("重要文档")
        .add_tag("团队协作")
        .build();

    match client.assistant.v1.subscription.create(premium_request, None).await {
        Ok(response) => {
            println!("高级订阅创建成功:");
            println!("完整信息: {}", response.info_summary());
            println!("文档类型: {}", response.file_type_enum().chinese_name());
        }
        Err(e) => {
            println!("创建高级订阅失败: {:?}", e);
        }
    }

    // 4. 创建自定义订阅
    println!("\n--- 创建自定义订阅 ---");
    let custom_request = open_lark::service::assistant::v1::subscription::CreateSubscriptionRequest::builder()
        .file_token("bitablexxxxxx")
        .as_bitable()
        .high_priority()
        .notification_interval(600) // 10分钟
        .auto_renew(true)
        .add_tag("数据分析")
        .add_tag("实时监控")
        .build();

    match client.assistant.v1.subscription.create(custom_request, None).await {
        Ok(response) => {
            println!("自定义订阅创建成功:");
            println!("完整信息: {}", response.info_summary());
        }
        Err(e) => {
            println!("创建自定义订阅失败: {:?}", e);
        }
    }

    // 5. 更新订阅状态 - 激活
    println!("\n--- 激活订阅 ---");
    let activate_request = open_lark::service::assistant::v1::subscription::PatchSubscriptionRequest::builder()
        .file_token(file_token)
        .file_type(file_type.clone())
        .activate()
        .high_priority()
        .quick_notification()
        .build();

    match client.assistant.v1.subscription.patch(activate_request, None).await {
        Ok(response) => {
            println!("订阅激活成功:");
            println!("完整信息: {}", response.info_summary());
            println!("更新字段: {:?}", response.get_updated_fields());
            println!("是否更新成功: {}", response.is_updated());
        }
        Err(e) => {
            println!("激活订阅失败: {:?}", e);
        }
    }

    // 6. 更新订阅状态 - 使用构建器查看更改
    println!("\n--- 使用构建器更新订阅 ---");
    let mut patch_builder = open_lark::service::assistant::v1::subscription::PatchSubscriptionRequest::builder()
        .file_token(file_token)
        .file_type(file_type.clone())
        .notification_interval(1800) // 30分钟
        .add_tag("已优化")
        .remove_tag("旧标签")
        .auto_renew(false);

    println!("计划的更改: {}", patch_builder.changes_summary());

    let patch_request = patch_builder.build();
    
    match client.assistant.v1.subscription.patch(patch_request, None).await {
        Ok(response) => {
            println!("订阅更新成功:");
            println!("完整信息: {}", response.info_summary());
        }
        Err(e) => {
            println!("更新订阅失败: {:?}", e);
        }
    }

    // 7. 暂停订阅
    println!("\n--- 暂停订阅 ---");
    let pause_request = open_lark::service::assistant::v1::subscription::PatchSubscriptionRequest::builder()
        .file_token(file_token)
        .file_type(file_type.clone())
        .safe_pause()
        .build();

    match client.assistant.v1.subscription.patch(pause_request, None).await {
        Ok(response) => {
            println!("订阅暂停成功:");
            println!("完整信息: {}", response.info_summary());
            println!("订阅状态: {}", response.subscription.status.description());
        }
        Err(e) => {
            println!("暂停订阅失败: {:?}", e);
        }
    }

    // 8. 快速激活订阅
    println!("\n--- 快速激活订阅 ---");
    let quick_activate_request = open_lark::service::assistant::v1::subscription::PatchSubscriptionRequest::builder()
        .file_token(file_token)
        .file_type(file_type.clone())
        .quick_activate()
        .build();

    match client.assistant.v1.subscription.patch(quick_activate_request, None).await {
        Ok(response) => {
            println!("快速激活成功:");
            println!("完整信息: {}", response.info_summary());
        }
        Err(e) => {
            println!("快速激活失败: {:?}", e);
        }
    }

    // 9. 节能模式激活
    println!("\n--- 节能模式激活 ---");
    let eco_activate_request = open_lark::service::assistant::v1::subscription::PatchSubscriptionRequest::builder()
        .file_token(file_token)
        .file_type(file_type.clone())
        .eco_activate()
        .build();

    match client.assistant.v1.subscription.patch(eco_activate_request, None).await {
        Ok(response) => {
            println!("节能模式激活成功:");
            println!("完整信息: {}", response.info_summary());
        }
        Err(e) => {
            println!("节能模式激活失败: {:?}", e);
        }
    }

    // 10. 使用服务便捷方法
    println!("\n--- 使用服务便捷方法 ---");
    
    // 快速订阅文档
    println!("快速订阅文档...");
    match client.assistant.v1.subscription.quick_subscribe_doc("doc_new_xxxxx", None).await {
        Ok(response) => {
            println!("快速订阅文档成功: {}", response.info_summary());
        }
        Err(e) => {
            println!("快速订阅文档失败: {:?}", e);
        }
    }

    // 检查订阅状态
    match client.assistant.v1.subscription.is_subscribed(file_token, file_type.clone(), None).await {
        Ok(is_subscribed) => {
            println!("订阅状态检查: {}", if is_subscribed { "已订阅" } else { "未订阅" });
        }
        Err(e) => {
            println!("检查订阅状态失败: {:?}", e);
        }
    }

    // 快速激活订阅
    match client.assistant.v1.subscription.quick_activate(file_token, file_type.clone(), None).await {
        Ok(response) => {
            println!("服务快速激活成功: {}", response.info_summary());
        }
        Err(e) => {
            println!("服务快速激活失败: {:?}", e);
        }
    }

    // 11. 批量订阅
    println!("\n--- 批量订阅示例 ---");
    let files_to_subscribe = vec![
        ("doc1_xxxxx".to_string(), open_lark::service::assistant::v1::subscription::FileType::Doc),
        ("sheet1_xxxxx".to_string(), open_lark::service::assistant::v1::subscription::FileType::Sheet),
        ("wiki1_xxxxx".to_string(), open_lark::service::assistant::v1::subscription::FileType::Wiki),
    ];

    let batch_results = client.assistant.v1.subscription.batch_subscribe(files_to_subscribe, None).await;
    
    println!("批量订阅结果:");
    for (i, result) in batch_results.iter().enumerate() {
        match result {
            Ok(response) => {
                println!("  {}. 成功: {}", i + 1, response.info_summary());
            }
            Err(e) => {
                println!("  {}. 失败: {:?}", i + 1, e);
            }
        }
    }

    // 12. 文档类型功能展示
    println!("\n--- 文档类型功能展示 ---");
    let doc_type = open_lark::service::assistant::v1::subscription::FileType::Doc;
    let bitable_type = open_lark::service::assistant::v1::subscription::FileType::Bitable;
    let sheet_type = open_lark::service::assistant::v1::subscription::FileType::Sheet;
    let wiki_type = open_lark::service::assistant::v1::subscription::FileType::Wiki;

    println!("支持的文档类型:");
    for file_type in [&doc_type, &bitable_type, &sheet_type, &wiki_type] {
        println!("- {}: {} (支持助手: {})",
            file_type.to_string(),
            file_type.chinese_name(),
            if file_type.supports_assistant() { "是" } else { "否" }
        );
    }

    // 13. 订阅优先级和配置展示
    println!("\n--- 订阅优先级和配置展示 ---");
    use open_lark::service::assistant::v1::subscription::{SubscriptionPriority, SubscriptionConfig};
    
    let priorities = [
        SubscriptionPriority::Low,
        SubscriptionPriority::Normal,
        SubscriptionPriority::High,
        SubscriptionPriority::Urgent,
    ];

    println!("优先级列表:");
    for priority in priorities {
        println!("- {}: 值={}, 颜色={}",
            priority.description(),
            priority.value(),
            priority.color()
        );
    }

    // 创建不同配置示例
    let configs = vec![
        ("默认配置", SubscriptionConfig::default()),
        ("高频配置", SubscriptionConfig {
            enable_notification: Some(true),
            notification_interval: Some(300), // 5分钟
            priority: Some(SubscriptionPriority::High),
            auto_renew: Some(true),
            tags: Some(vec!["高频".to_string(), "重要".to_string()]),
            custom_properties: None,
        }),
        ("节能配置", SubscriptionConfig {
            enable_notification: Some(true),
            notification_interval: Some(21600), // 6小时
            priority: Some(SubscriptionPriority::Low),
            auto_renew: Some(false),
            tags: Some(vec!["节能".to_string()]),
            custom_properties: None,
        }),
    ];

    println!("\n配置示例:");
    for (name, config) in configs {
        println!("{}:", name);
        println!("  摘要: {}", config.summary());
        println!("  通知频率: {:.1} 小时", config.get_notification_interval_hours());
        println!("  是否高频: {}", config.is_high_frequency());
        println!("  标签: {:?}", config.get_tags());
    }

    Ok(())
}