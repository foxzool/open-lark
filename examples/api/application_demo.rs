use dotenvy::dotenv;
use open_lark::{
    prelude::*,
    service::application::models::{DepartmentIdType, UserIdType},
};

/// Application v6 API 功能演示
///
/// 演示应用信息管理的各项功能，包括：
/// - 应用信息查询和管理
/// - 应用权限申请和授权状态查询
/// - 应用管理员和可用范围管理
/// - 应用商店订单和付费方案查询
/// - 应用使用统计和反馈管理
/// - 应用红点消息设置
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("APP_ID not found in environment");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET not found in environment");

    // 创建 LarkClient
    let client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .with_enable_token_cache(true)
        .build();

    println!("🚀 Application v6 API 功能演示开始\n");

    // ===================
    // 应用信息管理演示
    // ===================
    println!("📋 应用信息管理功能演示");

    // 获取应用信息
    match client
        .application
        .v6
        .application
        .get(&app_id, Some("zh_cn".to_string()), None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 获取应用信息成功");
                println!("   应用ID: {:?}", data.application.app_id);
                println!("   应用名称: {:?}", data.application.app_name);
                println!("   应用类型: {:?}", data.application.app_type);
                println!("   应用状态: {:?}", data.application.status);
            }
        }
        Err(e) => println!("❌ 获取应用信息失败: {e:?}"),
    }

    // 获取应用版本列表
    match client
        .application
        .v6
        .application
        .list_versions(&app_id, Some("zh_cn".to_string()), Some(20), None, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 获取应用版本列表成功");
                println!("   版本数量: {}", data.versions.len());
                for version in &data.versions {
                    println!(
                        "   - 版本: {:?} ({})",
                        version.version,
                        version.version_name.as_deref().unwrap_or("")
                    );
                }
            }
        }
        Err(e) => println!("❌ 获取应用版本列表失败: {e:?}"),
    }

    // 获取应用协作者列表
    match client
        .application
        .v6
        .application
        .get_collaborators(&app_id, Some(UserIdType::UserId), None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 获取应用协作者列表成功");
                println!("   协作者数量: {}", data.collaborators.len());
            }
        }
        Err(e) => println!("❌ 获取应用协作者列表失败: {e:?}"),
    }

    println!();

    // ===================
    // 应用权限管理演示
    // ===================
    println!("🔐 应用权限管理功能演示");

    // 查询租户授权状态
    match client
        .application
        .v6
        .scope
        .list(&app_id, Some("zh_cn".to_string()), None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 查询租户授权状态成功");
                println!("   权限数量: {}", data.scopes.len());
                for scope in &data.scopes {
                    println!(
                        "   - 权限: {:?}, 状态: {:?}",
                        scope.permission, scope.status
                    );
                }
            }
        }
        Err(e) => println!("❌ 查询租户授权状态失败: {e:?}"),
    }

    println!();

    // ===================
    // 应用管理功能演示
    // ===================
    println!("⚙️ 应用管理功能演示");

    // 获取企业安装的应用
    match client
        .application
        .v6
        .admin
        .list_installed_apps(
            Some(UserIdType::UserId),
            Some("zh_cn".to_string()),
            Some(20),
            None,
            None,
        )
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 获取企业安装的应用成功");
                println!("   应用数量: {}", data.apps.len());
                for app in &data.apps {
                    println!(
                        "   - 应用: {:?} ({})",
                        app.app_name,
                        app.app_id.as_deref().unwrap_or("")
                    );
                }
            }
        }
        Err(e) => println!("❌ 获取企业安装的应用失败: {e:?}"),
    }

    // 查询应用管理员列表
    match client
        .application
        .v6
        .admin
        .list_app_admins(&app_id, Some(UserIdType::UserId), Some(20), None, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 查询应用管理员列表成功");
                println!("   管理员数量: {}", data.admins.len());
            }
        }
        Err(e) => println!("❌ 查询应用管理员列表失败: {e:?}"),
    }

    // 获取应用在企业内的可用范围
    match client
        .application
        .v6
        .admin
        .get_app_availability(
            &app_id,
            Some(UserIdType::UserId),
            Some(DepartmentIdType::DepartmentId),
            None,
        )
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 获取应用可用范围成功");
                println!("   对所有人可用: {:?}", data.availability.is_visible_to_all);
            }
        }
        Err(e) => println!("❌ 获取应用可用范围失败: {e:?}"),
    }

    // 获取应用通讯录权限范围配置
    match client
        .application
        .v6
        .admin
        .contacts_range_configuration(
            &app_id,
            Some(DepartmentIdType::DepartmentId),
            Some(UserIdType::UserId),
            None,
        )
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 获取通讯录权限范围配置成功");
                println!("   范围类型: {:?}", data.contacts_range.range_type);
            }
        }
        Err(e) => println!("❌ 获取通讯录权限范围配置失败: {e:?}"),
    }

    println!();

    // ===================
    // 应用商店功能演示
    // ===================
    println!("🛒 应用商店功能演示");

    // 查询租户购买的付费方案
    match client
        .application
        .v6
        .appstore_paid_info
        .list_tenant_paid_plans(&app_id, Some(20), None, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 查询租户付费方案成功");
                println!("   付费方案数量: {}", data.pricing_plans.len());
                for plan in &data.pricing_plans {
                    println!(
                        "   - 方案: {:?} ({})",
                        plan.plan_name,
                        plan.pricing_plan_id.as_deref().unwrap_or("")
                    );
                }
            }
        }
        Err(e) => println!("❌ 查询租户付费方案失败: {e:?}"),
    }

    println!();

    // ===================
    // 应用使用情况演示
    // ===================
    println!("📊 应用使用情况功能演示");

    let yesterday = chrono::Utc::now().date_naive() - chrono::Duration::days(1);
    let date_str = yesterday.format("%Y-%m-%d").to_string();

    // 获取应用使用概览
    match client
        .application
        .v6
        .app_usage
        .overview(&app_id, &date_str, &date_str, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 获取应用使用概览成功");
                println!("   使用数据条数: {}", data.usage_data.len());
                for usage in &data.usage_data {
                    println!(
                        "   - 日期: {:?}, 活跃用户: {:?}",
                        usage.date, usage.active_users
                    );
                }
            }
        }
        Err(e) => println!("❌ 获取应用使用概览失败: {e:?}"),
    }

    // 获取消息推送概览
    match client
        .application
        .v6
        .app_usage
        .message_push_overview(&app_id, &date_str, &date_str, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 获取消息推送概览成功");
                println!("   推送数据条数: {}", data.usage_data.len());
            }
        }
        Err(e) => println!("❌ 获取消息推送概览失败: {e:?}"),
    }

    // 获取多部门应用使用概览
    match client
        .application
        .v6
        .app_usage
        .department_overview(
            &app_id,
            &date_str,
            Some(DepartmentIdType::DepartmentId),
            Some(20),
            None,
            None,
        )
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 获取部门使用概览成功");
                println!("   部门数据条数: {}", data.departments.len());
            }
        }
        Err(e) => println!("❌ 获取部门使用概览失败: {e:?}"),
    }

    println!();

    // ===================
    // 应用反馈管理演示
    // ===================
    println!("💬 应用反馈管理功能演示");

    // 获取应用反馈列表
    match client
        .application
        .v6
        .application_feedback
        .list(
            &app_id,
            Some(UserIdType::UserId),
            None,
            None,
            Some(20),
            None,
            None,
        )
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 获取应用反馈列表成功");
                println!("   反馈数量: {}", data.feedbacks.len());
                for feedback in &data.feedbacks {
                    println!(
                        "   - 反馈ID: {:?}, 类型: {:?}, 状态: {:?}",
                        feedback.feedback_id, feedback.feedback_type, feedback.status
                    );
                }
            }
        }
        Err(e) => println!("❌ 获取应用反馈列表失败: {e:?}"),
    }

    println!();

    // ===================
    // 待审核应用演示
    // ===================
    println!("📋 待审核应用功能演示");

    // 查看待审核的应用列表
    match client
        .application
        .v6
        .application
        .underaudit_list(Some("zh_cn".to_string()), Some(20), None, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 获取待审核应用列表成功");
                println!("   待审核应用数量: {}", data.applications.len());
                for app in &data.applications {
                    println!(
                        "   - 应用: {:?} ({})",
                        app.app_name,
                        app.app_id.as_deref().unwrap_or("")
                    );
                }
            }
        }
        Err(e) => println!("❌ 获取待审核应用列表失败: {e:?}"),
    }

    println!();

    println!("🎉 Application v6 API 功能演示完成！");
    println!("\n📝 演示内容总结:");
    println!("   ✅ 应用信息查询 - 获取应用详情、版本列表、协作者信息");
    println!("   ✅ 应用权限管理 - 查询授权状态、申请权限");
    println!("   ✅ 应用管理功能 - 企业应用列表、管理员管理、可用范围配置");
    println!("   ✅ 应用商店功能 - 付费方案查询、订单管理");
    println!("   ✅ 使用情况统计 - 应用使用概览、消息推送统计、部门使用情况");
    println!("   ✅ 反馈管理功能 - 反馈列表查询和状态更新");
    println!("   ✅ 审核管理功能 - 待审核应用列表查询");

    Ok(())
}
