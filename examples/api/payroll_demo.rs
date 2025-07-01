use std::collections::HashMap;

use dotenvy::dotenv;
use open_lark::{prelude::*, service::payroll::models::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    let app_id = std::env::var("APP_ID").expect("APP_ID environment variable is required");
    let app_secret =
        std::env::var("APP_SECRET").expect("APP_SECRET environment variable is required");

    // 创建客户端
    let client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .with_enable_token_cache(true)
        .build();

    println!("🏢 飞书发薪系统 API 演示");
    println!("====================================");

    // 1. 查询薪资组列表
    println!("\n📋 1. 查询薪资组列表");
    match query_paygroups(&client).await {
        Ok(_) => println!("✅ 薪资组查询成功"),
        Err(e) => println!("❌ 薪资组查询失败: {e}"),
    }

    // 2. 查询算薪项列表
    println!("\n💰 2. 查询算薪项列表");
    match query_acct_items(&client).await {
        Ok(_) => println!("✅ 算薪项查询成功"),
        Err(e) => println!("❌ 算薪项查询失败: {e}"),
    }

    // 3. 查询发薪活动列表
    println!("\n📊 3. 查询发薪活动列表");
    match query_payment_activities(&client).await {
        Ok(_) => println!("✅ 发薪活动查询成功"),
        Err(e) => println!("❌ 发薪活动查询失败: {e}"),
    }

    // 4. 查询发薪明细
    println!("\n📄 4. 查询发薪明细");
    match query_payment_details(&client).await {
        Ok(_) => println!("✅ 发薪明细查询成功"),
        Err(e) => println!("❌ 发薪明细查询失败: {e}"),
    }

    // 5. 查询外部数据源配置
    println!("\n🔗 5. 查询外部数据源配置");
    match query_datasources(&client).await {
        Ok(_) => println!("✅ 外部数据源查询成功"),
        Err(e) => println!("❌ 外部数据源查询失败: {e}"),
    }

    // 6. 创建外部算薪数据
    println!("\n📝 6. 创建外部算薪数据");
    match create_datasource_records(&client).await {
        Ok(_) => println!("✅ 外部算薪数据创建成功"),
        Err(e) => println!("❌ 外部算薪数据创建失败: {e}"),
    }

    // 7. 查询成本分摊方案
    println!("\n📈 7. 查询成本分摊方案");
    match query_cost_allocation_plans(&client).await {
        Ok(_) => println!("✅ 成本分摊方案查询成功"),
        Err(e) => println!("❌ 成本分摊方案查询失败: {e}"),
    }

    // 8. 查询成本分摊报表
    println!("\n📋 8. 查询成本分摊报表");
    match query_cost_allocation_reports(&client).await {
        Ok(_) => println!("✅ 成本分摊报表查询成功"),
        Err(e) => println!("❌ 成本分摊报表查询失败: {e}"),
    }

    // 9. 封存发薪活动演示
    println!("\n🗃️ 9. 封存发薪活动演示");
    match archive_payment_activity(&client).await {
        Ok(_) => println!("✅ 发薪活动封存演示成功"),
        Err(e) => println!("❌ 发薪活动封存演示失败: {e}"),
    }

    println!("\n🎉 发薪系统 API 演示完成！");

    Ok(())
}

/// 查询薪资组列表
async fn query_paygroups(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    let request = PaygroupListRequest {
        page_size: Some(10),
        page_token: None,
        status: Some("active".to_string()),
    };

    let response = client
        .payroll
        .paygroup
        .list_paygroups(request, None)
        .await?;

    if let Some(data) = &response.data {
        println!("  📊 薪资组总数: {}", data.paygroups.items.len());
        for (i, paygroup) in data.paygroups.items.iter().enumerate() {
            println!("  [{:2}] 薪资组: {}", i + 1, paygroup.paygroup_id);
            println!("       名称: {:?}", paygroup.paygroup_name.zh_cn);
            println!("       类型: {}", paygroup.paygroup_type);
            println!("       状态: {}", paygroup.status);
            if let Some(count) = paygroup.employee_count {
                println!("       员工数: {count}");
            }
            if let Some(desc) = &paygroup.description {
                if let Some(desc_text) = &desc.zh_cn {
                    println!("       描述: {desc_text}");
                }
            }
            println!();
        }
    }

    Ok(())
}

/// 查询算薪项列表
async fn query_acct_items(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    let request = AcctItemListRequest {
        page_size: Some(20),
        page_token: None,
        item_type: Some("income".to_string()),
        paygroup_id: None,
        status: Some("active".to_string()),
    };

    let response = client
        .payroll
        .acct_item
        .list_acct_items(request, None)
        .await?;

    if let Some(data) = &response.data {
        println!("  📊 算薪项总数: {}", data.acct_items.items.len());
        for (i, item) in data.acct_items.items.iter().enumerate() {
            println!("  [{:2}] 算薪项: {}", i + 1, item.acct_item_id);
            println!("       名称: {:?}", item.item_name.zh_cn);
            println!("       类型: {}", item.item_type);
            if let Some(category) = &item.category {
                println!("       分类: {category}");
            }
            println!("       个税相关: {}", item.tax_related);
            println!("       社保相关: {}", item.social_security_related);
            println!("       状态: {}", item.status);
            println!();
        }
    }

    Ok(())
}

/// 查询发薪活动列表
async fn query_payment_activities(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    let request = PaymentActivityListRequest {
        page_size: Some(10),
        page_token: None,
        status: Some("active".to_string()),
        paygroup_id: None,
        period_start: Some("2024-01-01".to_string()),
        period_end: Some("2024-12-31".to_string()),
    };

    let response = client
        .payroll
        .payment_activity
        .list_activities(request, None)
        .await?;

    if let Some(data) = &response.data {
        println!("  📊 发薪活动总数: {}", data.activities.items.len());
        for (i, activity) in data.activities.items.iter().enumerate() {
            println!("  [{:2}] 发薪活动: {}", i + 1, activity.payment_activity_id);
            println!("       名称: {:?}", activity.activity_name.zh_cn);
            println!("       状态: {}", activity.status);
            println!("       薪资组: {}", activity.paygroup_id);
            println!(
                "       周期: {} ~ {}",
                activity.period_start, activity.period_end
            );
            if let Some(count) = activity.employee_count {
                println!("       员工数: {count}");
            }
            if let Some(amount) = &activity.total_amount {
                println!(
                    "       总金额: {} {}",
                    amount,
                    activity.currency.as_deref().unwrap_or("CNY")
                );
            }
            println!();
        }
    }

    Ok(())
}

/// 查询发薪明细
async fn query_payment_details(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    // 注意：这里使用示例活动ID，实际使用时需要先查询到真实的活动ID
    let activity_id = "demo_activity_123";

    let request = PaymentDetailListRequest {
        payment_activity_id: activity_id.to_string(),
        page_size: Some(10),
        page_token: None,
        employee_id: None,
        user_id_type: Some("open_id".to_string()),
        department_id_type: Some("open_department_id".to_string()),
    };

    match client
        .payroll
        .payment_detail
        .list_details(request, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = &response.data {
                println!("  📊 发薪明细总数: {}", data.details.items.len());
                for (i, detail) in data.details.items.iter().enumerate() {
                    println!("  [{:2}] 员工: {}", i + 1, detail.employee_id);
                    if let Some(name) = &detail.employee_name {
                        println!("       姓名: {:?}", name.zh_cn);
                    }
                    if let Some(number) = &detail.employee_number {
                        println!("       工号: {number}");
                    }
                    if let Some(total) = &detail.total_amount {
                        println!(
                            "       总金额: {} {}",
                            total,
                            detail.currency.as_deref().unwrap_or("CNY")
                        );
                    }
                    println!("       发薪项目数: {}", detail.payment_items.len());
                    for (j, item) in detail.payment_items.iter().enumerate() {
                        println!(
                            "         [{}] {}: {} {}",
                            j + 1,
                            item.acct_item_name
                                .as_ref()
                                .and_then(|n| n.zh_cn.as_ref())
                                .unwrap_or(&item.acct_item_id),
                            item.amount,
                            item.currency.as_deref().unwrap_or("CNY")
                        );
                    }
                    println!();
                }
            }
        }
        Err(e) => {
            println!("  ⚠️ 发薪明细查询失败 (可能是示例活动ID不存在): {e}");
        }
    }

    Ok(())
}

/// 查询外部数据源配置
async fn query_datasources(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    let request = DatasourceListRequest {
        page_size: Some(10),
        page_token: None,
        status: Some("active".to_string()),
    };

    let response = client
        .payroll
        .datasource
        .list_datasources(request, None)
        .await?;

    if let Some(data) = &response.data {
        println!("  📊 数据源总数: {}", data.datasources.items.len());
        for (i, datasource) in data.datasources.items.iter().enumerate() {
            println!("  [{:2}] 数据源: {}", i + 1, datasource.datasource_id);
            println!("       名称: {:?}", datasource.datasource_name.zh_cn);
            println!("       类型: {}", datasource.datasource_type);
            println!("       状态: {}", datasource.status);
            println!("       字段数: {}", datasource.field_configs.len());
            for (j, field) in datasource.field_configs.iter().enumerate() {
                println!(
                    "         [{}] {}: {} ({})",
                    j + 1,
                    field.field_id,
                    field.field_name.zh_cn.as_deref().unwrap_or(""),
                    field.field_type
                );
            }
            println!();
        }
    }

    Ok(())
}

/// 创建外部算薪数据演示
async fn create_datasource_records(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    // 注意：这里使用示例数据源ID和员工ID，实际使用时需要使用真实的ID
    let datasource_id = "demo_datasource_123";
    let employee_id = "demo_employee_456";

    let mut field_values = HashMap::new();
    field_values.insert(
        "base_salary".to_string(),
        serde_json::Value::Number(serde_json::Number::from(10000)),
    );
    field_values.insert(
        "overtime_hours".to_string(),
        serde_json::Value::Number(serde_json::Number::from(20)),
    );
    field_values.insert(
        "performance_bonus".to_string(),
        serde_json::Value::Number(serde_json::Number::from(2000)),
    );

    let record = DatasourceRecord {
        record_id: None,
        employee_id: employee_id.to_string(),
        field_values,
        payment_period: "2024-01".to_string(),
        created_time: None,
        updated_time: None,
    };

    let request = DatasourceRecordSaveRequest {
        datasource_id: datasource_id.to_string(),
        employee_id: employee_id.to_string(),
        user_id_type: Some("open_id".to_string()),
        records: vec![record],
        payment_period: "2024-01".to_string(),
    };

    match client
        .payroll
        .datasource_record
        .save_records(request, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = &response.data {
                println!("  ✅ 保存结果: {}", data.success);
                if let Some(count) = data.record_count {
                    println!("  📊 成功保存记录数: {count}");
                }
                if let Some(failed) = &data.failed_records {
                    println!("  ❌ 失败记录数: {}", failed.len());
                    for fail in failed {
                        println!(
                            "    员工: {} - 原因: {}",
                            fail.employee_id, fail.error_message
                        );
                    }
                }
            }
        }
        Err(e) => {
            println!("  ⚠️ 外部算薪数据创建失败 (可能是示例ID不存在): {e}");
        }
    }

    Ok(())
}

/// 查询成本分摊方案
async fn query_cost_allocation_plans(
    client: &LarkClient,
) -> Result<(), Box<dyn std::error::Error>> {
    let request = CostAllocationPlanListRequest {
        page_size: Some(10),
        page_token: None,
        status: Some("active".to_string()),
        plan_type: None,
    };

    let response = client
        .payroll
        .cost_allocation_plan
        .list_plans(request, None)
        .await?;

    if let Some(data) = &response.data {
        println!("  📊 成本分摊方案总数: {}", data.plans.items.len());
        for (i, plan) in data.plans.items.iter().enumerate() {
            println!("  [{:2}] 方案: {}", i + 1, plan.plan_id);
            println!("       名称: {:?}", plan.plan_name.zh_cn);
            println!("       类型: {}", plan.plan_type);
            println!("       状态: {}", plan.status);
            if let Some(effective) = &plan.effective_date {
                println!("       生效日期: {effective}");
            }
            println!("       分摊规则数: {}", plan.allocation_rules.len());
            for (j, rule) in plan.allocation_rules.iter().enumerate() {
                println!(
                    "         [{}] {}: {}% -> {}",
                    j + 1,
                    rule.rule_name.zh_cn.as_deref().unwrap_or(&rule.rule_id),
                    rule.allocation_ratio * 100.0,
                    rule.target_cost_center_id
                );
            }
            println!();
        }
    }

    Ok(())
}

/// 查询成本分摊报表
async fn query_cost_allocation_reports(
    client: &LarkClient,
) -> Result<(), Box<dyn std::error::Error>> {
    let request = CostAllocationReportListRequest {
        start_date: "2024-01-01".to_string(),
        end_date: "2024-03-31".to_string(),
        cost_center_id: None,
        department_id: None,
        page_size: Some(10),
        page_token: None,
        report_type: Some("monthly".to_string()),
    };

    let response = client
        .payroll
        .cost_allocation_report
        .list_reports(request, None)
        .await?;

    if let Some(data) = &response.data {
        println!("  📊 成本分摊报表总数: {}", data.reports.items.len());
        for (i, report) in data.reports.items.iter().enumerate() {
            println!("  [{:2}] 报表: {}", i + 1, report.report_id);
            println!("       成本中心: {}", report.cost_center_id);
            if let Some(name) = &report.cost_center_name {
                println!("       中心名称: {:?}", name.zh_cn);
            }
            println!("       员工数: {}", report.employee_count);
            println!("       总成本: {} {}", report.total_cost, report.currency);
            println!(
                "       统计周期: {} ~ {}",
                report.period_start, report.period_end
            );
            println!("       分摊明细数: {}", report.allocation_details.len());
            for (j, detail) in report.allocation_details.iter().enumerate() {
                println!(
                    "         [{}] {}: {} ({} 人)",
                    j + 1,
                    detail
                        .acct_item_name
                        .as_ref()
                        .and_then(|n| n.zh_cn.as_ref())
                        .unwrap_or(&detail.acct_item_id),
                    detail.allocated_amount,
                    detail.employee_count
                );
            }
            println!();
        }
    }

    Ok(())
}

/// 封存发薪活动演示
async fn archive_payment_activity(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    // 注意：这里使用示例活动ID，实际使用时需要使用真实的ID
    let activity_id = "demo_activity_for_archive";

    let request = PaymentActivityArchiveRequest {
        payment_activity_id: activity_id.to_string(),
        archive_reason: Some("发薪完成，数据确认无误，进行封存归档".to_string()),
    };

    match client
        .payroll
        .payment_activity
        .archive_activity(request, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = &response.data {
                println!("  ✅ 封存结果: {}", data.success);
                if let Some(time) = &data.archived_time {
                    println!("  📅 封存时间: {time}");
                }
                if let Some(msg) = &data.message {
                    println!("  💬 消息: {msg}");
                }
            }
        }
        Err(e) => {
            println!("  ⚠️ 发薪活动封存失败 (可能是示例活动ID不存在): {e}");
        }
    }

    Ok(())
}
