use std::env;

use dotenvy::dotenv;
use open_lark::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("Missing APP_ID environment variable");
    let app_secret = env::var("APP_SECRET").expect("Missing APP_SECRET environment variable");

    // 创建飞书客户端
    let client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .build();

    println!("🚀 飞书招聘 v1 API 示例");
    println!("{}", "=".repeat(50));

    // 1. 职位管理示例
    println!("\n📋 1. 获取职位列表");
    match client
        .hire
        .recruitment_config
        .job
        .list_jobs(Default::default(), None)
        .await
    {
        Ok(response) => {
            if let Some(data) = &response.data {
                println!("✅ 成功获取职位列表，共 {} 个职位", data.jobs.items.len());
                for job in &data.jobs.items {
                    println!("  - 职位: {} ({})", job.id, job.id);
                }
            } else {
                println!("✅ 职位列表API调用成功，但暂无数据");
            }
        }
        Err(e) => println!("❌ 获取职位列表失败: {e:?}"),
    }

    // 1.1 人才标签管理示例
    println!("\n🏷️  1.1 获取人才标签列表");
    match client
        .hire
        .recruitment_config
        .application
        .list_talent_tags(Default::default(), None)
        .await
    {
        Ok(response) => {
            if let Some(data) = &response.data {
                println!("✅ 获取人才标签成功，共 {} 个标签", data.tags.items.len());
                for tag in data.tags.items.iter().take(5) {
                    println!(
                        "  - 标签: {} ({})",
                        tag.name.as_deref().unwrap_or("未知"),
                        tag.tag_id
                    );
                }
            } else {
                println!("✅ 人才标签API调用成功，但暂无数据");
            }
        }
        Err(e) => println!("❌ 获取人才标签失败: {e:?}"),
    }

    // 2. 人才库管理示例
    println!("\n📚 2. 人才库管理");
    match client
        .hire
        .candidate_management
        .talent_pool
        .list_pools(Default::default(), None)
        .await
    {
        Ok(response) => {
            if let Some(data) = &response.data {
                println!(
                    "✅ 获取人才库列表成功，共 {} 个人才库",
                    data.pools.items.len()
                );
                for pool in &data.pools.items {
                    println!("  - 人才库: {} ({})", pool.id, pool.id);
                }
            } else {
                println!("✅ 人才库列表API调用成功，但暂无数据");
            }
        }
        Err(e) => println!("❌ 获取人才库列表失败: {e:?}"),
    }

    // 3. 内推管理示例
    println!("\n🔗 3. 内推信息管理");
    match client
        .hire
        .get_candidates
        .referral
        .list_referrals(Default::default(), None)
        .await
    {
        Ok(response) => {
            if let Some(data) = &response.data {
                println!(
                    "✅ 获取内推信息成功，共 {} 条记录",
                    data.referrals.items.len()
                );
                for referral in &data.referrals.items {
                    println!("  - 内推记录: {} 状态: {}", referral.id, referral.status);
                }
            } else {
                println!("✅ 内推信息API调用成功，但暂无数据");
            }
        }
        Err(e) => println!("❌ 获取内推信息失败: {e:?}"),
    }

    // 4. 附件管理示例
    println!("\n📎 4. 附件管理");
    match client
        .hire
        .attachment
        .list_attachments(Default::default(), None)
        .await
    {
        Ok(response) => {
            if let Some(data) = &response.data {
                println!(
                    "✅ 获取附件列表成功，共 {} 个附件",
                    data.attachments.items.len()
                );
                for attachment in data.attachments.items.iter().take(5) {
                    println!("  - 附件: {} ({})", attachment.name, attachment.file_type);
                }
            } else {
                println!("✅ 附件列表API调用成功，但暂无数据");
            }
        }
        Err(e) => println!("❌ 获取附件列表失败: {e:?}"),
    }

    // 5. 内推账户管理示例
    println!("\n💰 5. 内推账户管理");
    match client
        .hire
        .referral_account
        .list_accounts(Default::default(), None)
        .await
    {
        Ok(response) => {
            if let Some(data) = &response.data {
                println!(
                    "✅ 获取内推账户列表成功，共 {} 个账户",
                    data.accounts.items.len()
                );
                for account in data.accounts.items.iter().take(3) {
                    println!("  - 账户: {} 状态: {}", account.user_id, account.status);
                }
            } else {
                println!("✅ 内推账户列表API调用成功，但暂无数据");
            }
        }
        Err(e) => println!("❌ 获取内推账户列表失败: {e:?}"),
    }

    // 6. 笔试管理示例
    println!("\n📝 6. 笔试管理");
    match client
        .hire
        .ecological_docking
        .exam
        .list_papers(Default::default(), None)
        .await
    {
        Ok(response) => {
            if let Some(data) = &response.data {
                println!("✅ 获取试卷列表成功，共 {} 张试卷", data.papers.items.len());
                for paper in data.papers.items.iter().take(3) {
                    println!("  - 试卷: {} 题目数: {}", paper.id, paper.question_count);
                }
            } else {
                println!("✅ 试卷列表API调用成功，但暂无数据");
            }
        }
        Err(e) => println!("❌ 获取试卷列表失败: {e:?}"),
    }

    // 7. 背调管理示例
    println!("\n🔍 7. 背调管理");
    match client
        .hire
        .ecological_docking
        .background_check
        .list_orders(Default::default(), None)
        .await
    {
        Ok(response) => {
            if let Some(data) = &response.data {
                println!("✅ 获取背调订单成功，共 {} 个订单", data.orders.items.len());
                for order in data.orders.items.iter().take(3) {
                    println!("  - 订单: {} 状态: {}", order.id, order.status);
                }
            } else {
                println!("✅ 背调订单API调用成功，但暂无数据");
            }
        }
        Err(e) => println!("❌ 获取背调订单失败: {e:?}"),
    }

    println!("\n🎉 招聘系统 API 示例完成！");
    println!("{}", "=".repeat(50));
    println!("📖 这个示例展示了如何使用飞书招聘 v1 API 的各个模块：");
    println!("   • 职位管理 (job management)");
    println!("   • 人才库管理 (talent pool management)");
    println!("   • 内推管理 (referral management)");
    println!("   • 附件管理 (attachment management)");
    println!("   • 内推账户管理 (referral account management)");
    println!("   • 笔试管理 (exam management)");
    println!("   • 背调管理 (background check management)");
    println!("\n💡 注意: 某些 API 调用可能因为缺少权限或数据而返回错误，这是正常现象");
    println!("   在实际使用中，请确保配置了正确的权限范围和测试数据");
    println!("\n📋 相关权限范围:");
    println!("   • hire:job - 职位管理权限");
    println!("   • hire:candidate - 候选人管理权限");
    println!("   • hire:application - 投递管理权限");
    println!("   • hire:interview - 面试管理权限");
    println!("   • hire:offer - Offer管理权限");
    println!("   • hire:onboard - 入职管理权限");

    Ok(())
}
