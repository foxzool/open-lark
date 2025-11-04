//! 薪酬管理模块使用示例
//!
//! 演示如何使用compensation_management模块进行薪酬管理操作，包括：
//! - 薪酬档案管理
//! - 薪酬项目管理
//! - 薪酬指标管理
//! - 变更原因管理
//! - 社保管理
//! - 支付管理

use open_lark::prelude::*;
use open_lark::service::compensation_management;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    env_logger::init();

    println!("🚀 薪酬管理模块示例演示");

    // 创建客户端（这里使用测试配置）
    let client = LarkClient::builder("test_app_id", "test_app_secret")
        .with_app_type(AppType::SelfBuild)
        .build();

    println!("✅ 客户端创建成功");

    // 演示创建薪酬档案
    println!("\n📋 创建薪酬档案");
    let archive_request = compensation_management::v1::CreateSalaryArchiveRequest {
        user_id: "test_user_id".to_string(),
        salary_items: vec![
            compensation_management::v1::SalaryItem {
                item_id: "basic_salary".to_string(),
                amount: 10000.0,
                currency: "CNY".to_string(),
            },
        ],
        effective_date: "2024-01-01".to_string(),
        change_reason_id: Some("new_hire".to_string()),
        remarks: Some("新员工入职薪资".to_string()),
    };

    match client.compensation_management.v1.create_salary_archive(&archive_request).await {
        Ok(response) => {
            println!("✅ 薪酬档案创建成功");
            if let Some(data) = response.data {
                println!("   档案ID: {}", data.archive_id);
                println!("   用户ID: {}", data.user_id);
                println!("   生效日期: {}", data.effective_date);
            }
        }
        Err(e) => {
            println!("❌ 薪酬档案创建失败: {}", e);
        }
    }

    // 演示查询薪酬档案
    println!("\n📋 查询薪酬档案");
    let query_request = compensation_management::v1::QuerySalaryArchivesRequest {
        user_ids: Some(vec!["test_user_id".to_string()]),
        department_id: None,
        effective_date_start: Some("2024-01-01".to_string()),
        effective_date_end: Some("2024-12-31".to_string()),
        page_size: 20,
        page_token: None,
    };

    match client.compensation_management.v1.query_salary_archives(&query_request).await {
        Ok(response) => {
            println!("✅ 薪酬档案查询成功");
            if let Some(data) = response.data {
                println!("   找到 {} 个档案", data.archives.len());
                println!("   是否有更多: {}", data.has_more);
            }
        }
        Err(e) => {
            println!("❌ 薪酬档案查询失败: {}", e);
        }
    }

    // 演示创建薪酬项目
    println!("\n📋 创建薪酬项目");
    let item_request = compensation_management::v1::CreateCompensationItemRequest {
        name: "基本工资".to_string(),
        category_id: "salary_category".to_string(),
        description: Some("员工基本工资项目".to_string()),
        is_taxable: true,
        is_social_insurance_base: true,
    };

    match client.compensation_management.v1.create_compensation_item(&item_request).await {
        Ok(response) => {
            println!("✅ 薪酬项目创建成功");
            if let Some(data) = response.data {
                println!("   项目ID: {}", data.item_id);
                println!("   项目名称: {}", data.name);
                println!("   是否计税: {}", data.is_taxable);
            }
        }
        Err(e) => {
            println!("❌ 薪酬项目创建失败: {}", e);
        }
    }

    // 演示获取薪酬项目列表
    println!("\n📋 获取薪酬项目列表");
    let list_request = compensation_management::v1::ListCompensationItemsRequest {
        category_id: Some("salary_category".to_string()),
        page_size: 20,
        page_token: None,
    };

    match client.compensation_management.v1.list_compensation_items(&list_request).await {
        Ok(response) => {
            println!("✅ 薪酬项目列表获取成功");
            if let Some(data) = response.data {
                println!("   找到 {} 个项目", data.items.len());
                for item in &data.items {
                    println!("   - {}: {} ({})", item.item_id, item.name,
                        if item.is_taxable { "计税" } else { "不计税" });
                }
            }
        }
        Err(e) => {
            println!("❌ 薪酬项目列表获取失败: {}", e);
        }
    }

    // 演示创建薪酬指标
    println!("\n📋 创建薪酬指标");
    let indicator_request = compensation_management::v1::CreateCompensationIndicatorRequest {
        name: "月薪标准".to_string(),
        description: Some("员工月度薪资标准指标".to_string()),
        formula: "base_salary + allowance".to_string(),
        category: "salary_indicator".to_string(),
    };

    match client.compensation_management.v1.create_compensation_indicator(&indicator_request).await {
        Ok(response) => {
            println!("✅ 薪酬指标创建成功");
            if let Some(data) = response.data {
                println!("   指标ID: {}", data.indicator_id);
                println!("   指标名称: {}", data.name);
                println!("   计算公式: {}", data.formula);
            }
        }
        Err(e) => {
            println!("❌ 薪酬指标创建失败: {}", e);
        }
    }

    // 演示创建一次性支付
    println!("\n📋 创建一次性支付");
    let payment_request = compensation_management::v1::CreateLumpSumPaymentRequest {
        user_id: "test_user_id".to_string(),
        payment_items: vec![
            compensation_management::v1::PaymentItem {
                item_id: "bonus".to_string(),
                amount: 5000.0,
                currency: "CNY".to_string(),
            },
        ],
        payment_date: "2024-01-15".to_string(),
        reason: "年度奖金".to_string(),
    };

    match client.compensation_management.v1.create_lump_sum_payment(&payment_request).await {
        Ok(response) => {
            println!("✅ 一次性支付创建成功");
            if let Some(data) = response.data {
                println!("   支付ID: {}", data.payment_id);
                println!("   用户ID: {}", data.user_id);
                println!("   支付日期: {}", data.payment_date);
                println!("   支付状态: {}", data.status);
            }
        }
        Err(e) => {
            println!("❌ 一次性支付创建失败: {}", e);
        }
    }

    // 演示创建社保方案
    println!("\n📋 创建社保方案");
    let insurance_request = compensation_management::v1::CreateSocialInsurancePlanRequest {
        name: "北京市社保方案".to_string(),
        description: Some("北京市企业社保缴纳方案".to_string()),
        city: "北京".to_string(),
        effective_date: "2024-01-01".to_string(),
        items: vec![
            compensation_management::v1::SocialInsurancePlanItem {
                item_id: "pension".to_string(),
                company_rate: 0.16,
                personal_rate: 0.08,
                base_min: 5000.0,
                base_max: 30000.0,
            },
        ],
    };

    match client.compensation_management.v1.create_social_insurance_plan(&insurance_request).await {
        Ok(response) => {
            println!("✅ 社保方案创建成功");
            if let Some(data) = response.data {
                println!("   方案ID: {}", data.plan_id);
                println!("   方案名称: {}", data.name);
                println!("   适用城市: {}", data.city);
                println!("   生效日期: {}", data.effective_date);
            }
        }
        Err(e) => {
            println!("❌ 社保方案创建失败: {}", e);
        }
    }

    println!("\n🎉 薪酬管理模块示例演示完成！");
    Ok(())
}