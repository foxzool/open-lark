use dotenvy::dotenv;
use log::error;
use open_lark::{
    prelude::*,
    service::admin::models::{
        BadgeCreateRequest, BadgeGetRequest, BadgeGrantCreateRequest, BadgeGrantUser,
        BadgeListRequest, DepartmentDataReportRequest, PasswordResetRequest, UserDataReportRequest,
    },
};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("APP_ID environment variable not set");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET environment variable not set");

    let client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .with_enable_token_cache(true)
        .build();

    println!("🎯 飞书管理后台API演示");
    println!("================================");

    // 1. 密码管理演示
    println!("\n1. 密码管理演示");
    println!("-------------------------------");

    let password_request = PasswordResetRequest {
        user_id: "example_user_id".to_string(),
        password: "NewPassword123".to_string(),
    };

    match client
        .admin
        .password
        .reset_password(password_request, None)
        .await
    {
        Ok(response) => {
            println!("✅ 密码重置成功");
            println!("响应: {response:?}");
        }
        Err(err) => {
            error!("❌ 密码重置失败: {err:?}");
        }
    }

    // 2. 数据报表管理演示
    println!("\n2. 数据报表管理演示");
    println!("-------------------------------");

    // 获取部门维度数据报表
    let dept_report_request = DepartmentDataReportRequest {
        start_date: "2024-01-01".to_string(),
        end_date: "2024-01-31".to_string(),
        department_id_type: Some("open_department_id".to_string()),
        department_id: Some("example_dept_id".to_string()),
        page_size: Some(10),
        page_token: None,
    };

    match client
        .admin
        .data_report
        .get_department_data_report(dept_report_request, None)
        .await
    {
        Ok(response) => {
            println!("✅ 获取部门数据报表成功");
            println!("报表数据: {:?}", response.data);
        }
        Err(err) => {
            error!("❌ 获取部门数据报表失败: {err:?}");
        }
    }

    // 获取用户维度数据报表
    let user_report_request = UserDataReportRequest {
        start_date: "2024-01-01".to_string(),
        end_date: "2024-01-31".to_string(),
        user_id_type: Some("open_id".to_string()),
        user_ids: Some(vec!["user1".to_string(), "user2".to_string()]),
        department_id_type: None,
        department_id: None,
        page_size: Some(10),
        page_token: None,
    };

    match client
        .admin
        .data_report
        .get_user_data_report(user_report_request, None)
        .await
    {
        Ok(response) => {
            println!("✅ 获取用户数据报表成功");
            println!("报表数据: {:?}", response.data);
        }
        Err(err) => {
            error!("❌ 获取用户数据报表失败: {err:?}");
        }
    }

    // 3. 企业勋章管理演示
    println!("\n3. 企业勋章管理演示");
    println!("-------------------------------");

    // 创建勋章
    let mut i18n_name = HashMap::new();
    i18n_name.insert("zh_cn".to_string(), "优秀员工".to_string());
    i18n_name.insert("en_us".to_string(), "Excellent Employee".to_string());

    let badge_create_request = BadgeCreateRequest {
        name: "优秀员工勋章".to_string(),
        description: Some("表彰优秀员工的勋章".to_string()),
        detail_description: Some("该勋章用于表彰在工作中表现突出的员工".to_string()),
        show_detail_time: Some(true),
        image_key: Some("example_image_key".to_string()),
        i18n_name: Some(i18n_name),
        i18n_description: None,
        i18n_detail_description: None,
    };

    let badge_id = match client
        .admin
        .badge
        .create_badge(badge_create_request, None)
        .await
    {
        Ok(response) => {
            println!("✅ 创建勋章成功");
            let badge_id = response
                .data
                .as_ref()
                .and_then(|data| data.badge.badge_id.clone())
                .unwrap_or_default();
            println!("勋章ID: {badge_id}");
            badge_id
        }
        Err(err) => {
            error!("❌ 创建勋章失败: {err:?}");
            "example_badge_id".to_string()
        }
    };

    // 获取勋章列表
    let badge_list_request = BadgeListRequest {
        page_size: Some(10),
        page_token: None,
        name: Some("优秀".to_string()),
    };

    match client
        .admin
        .badge
        .list_badges(badge_list_request, None)
        .await
    {
        Ok(response) => {
            println!("✅ 获取勋章列表成功");
            if let Some(data) = &response.data {
                if let Some(badges) = &data.page_response.items {
                    println!("勋章数量: {}", badges.len());
                    for badge in badges {
                        if let Some(name) = &badge.name {
                            println!("- 勋章: {name}");
                        }
                    }
                }
            }
        }
        Err(err) => {
            error!("❌ 获取勋章列表失败: {err:?}");
        }
    }

    // 获取勋章详情
    let badge_get_request = BadgeGetRequest {
        badge_id: badge_id.clone(),
    };

    match client.admin.badge.get_badge(badge_get_request, None).await {
        Ok(response) => {
            println!("✅ 获取勋章详情成功");
            if let Some(data) = &response.data {
                println!("勋章信息: {:?}", data.badge);
            }
        }
        Err(err) => {
            error!("❌ 获取勋章详情失败: {err:?}");
        }
    }

    // 4. 勋章授予名单管理演示
    println!("\n4. 勋章授予名单管理演示");
    println!("-------------------------------");

    // 创建授予名单
    let badge_grant_users = vec![
        BadgeGrantUser {
            user_id: "user1".to_string(),
            user_id_type: Some("open_id".to_string()),
            reason: Some("工作表现优秀".to_string()),
            grant_time: Some("2024-01-15".to_string()),
        },
        BadgeGrantUser {
            user_id: "user2".to_string(),
            user_id_type: Some("open_id".to_string()),
            reason: Some("创新能力突出".to_string()),
            grant_time: Some("2024-01-16".to_string()),
        },
    ];

    let badge_grant_create_request = BadgeGrantCreateRequest {
        badge_id: badge_id.clone(),
        name: "2024年第一季度优秀员工".to_string(),
        description: Some("2024年第一季度表现优秀的员工名单".to_string()),
        user_list: badge_grant_users,
        effective_time: Some("2024-01-01T00:00:00Z".to_string()),
        expiry_time: Some("2024-12-31T23:59:59Z".to_string()),
        time_zone: Some("Asia/Shanghai".to_string()),
    };

    match client
        .admin
        .badge_grant
        .create_badge_grant(badge_grant_create_request, None)
        .await
    {
        Ok(response) => {
            println!("✅ 创建授予名单成功");
            if let Some(data) = &response.data {
                println!("授予名单信息: {:?}", data.grant);
            }
        }
        Err(err) => {
            error!("❌ 创建授予名单失败: {err:?}");
        }
    }

    println!("\n🎉 管理后台API演示完成!");
    Ok(())
}
