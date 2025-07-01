use dotenvy::dotenv;
use open_lark::prelude::*;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID 环境变量未设置");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET 环境变量未设置");

    let client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .with_enable_token_cache(true)
        .build();

    println!("🎧 开始服务台模块演示...");

    // 1. 客服功能管理示例
    println!("\n👥 客服功能管理示例");
    demo_agent_management(&client).await?;

    // 2. 客服工作日程示例
    println!("\n📅 客服工作日程示例");
    demo_agent_schedule(&client).await?;

    // 3. 客服技能管理示例
    println!("\n🎯 客服技能管理示例");
    demo_agent_skill(&client).await?;

    // 4. 工单管理示例
    println!("\n🎫 工单管理示例");
    demo_ticket_management(&client).await?;

    println!("\n✅ 服务台模块演示完成!");
    Ok(())
}

/// 客服功能管理示例
async fn demo_agent_management(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    use open_lark::service::helpdesk::{models::UserIdType, v1::agent::*};

    let agent_id = "agent_example_id";

    // 更新客服信息
    let update_request = UpdateAgentRequest {
        status: Some("online".to_string()),
        agent_name: Some("示例客服".to_string()),
    };

    match client
        .helpdesk
        .v1
        .agent
        .patch(agent_id, update_request, Some(UserIdType::OpenId), None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("  ✅ 客服信息更新成功");
                if let Some(name) = data.agent.agent_name {
                    println!("    📱 客服名称: {name}");
                }
                if let Some(status) = data.agent.status {
                    println!("    📊 客服状态: {status:?}");
                }
            }
        }
        Err(e) => {
            println!("  ❌ 客服信息更新失败: {e:?}");
        }
    }

    // 获取客服邮箱
    match client
        .helpdesk
        .v1
        .agent
        .agent_email(agent_id, Some(UserIdType::OpenId), None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("  ✅ 客服邮箱获取成功: {}", data.agent_email);
            }
        }
        Err(e) => {
            println!("  ❌ 客服邮箱获取失败: {e:?}");
        }
    }

    Ok(())
}

/// 客服工作日程示例
async fn demo_agent_schedule(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    use open_lark::service::helpdesk::{models::UserIdType, v1::agent_schedule::*};

    let agent_id = "agent_example_id";

    // 创建客服工作日程
    let create_request = CreateAgentScheduleRequest {
        start_time: "2024-01-01 09:00:00".to_string(),
        end_time: "2024-01-01 18:00:00".to_string(),
        repeat_type: Some("daily".to_string()),
    };

    match client
        .helpdesk
        .v1
        .agent_schedule
        .create(agent_id, create_request, Some(UserIdType::OpenId), None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("  ✅ 工作日程创建成功");
                if let Some(schedule_id) = data.schedule.schedule_id {
                    println!("    🆔 日程ID: {schedule_id}");
                }
                if let Some(start_time) = data.schedule.start_time {
                    println!("    ⏰ 开始时间: {start_time}");
                }
                if let Some(end_time) = data.schedule.end_time {
                    println!("    ⏰ 结束时间: {end_time}");
                }
            }
        }
        Err(e) => {
            println!("  ❌ 工作日程创建失败: {e:?}");
        }
    }

    // 查询全部客服工作日程
    match client
        .helpdesk
        .v1
        .agent_schedule
        .list(agent_id, Some(UserIdType::OpenId), None, Some(10), None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!(
                    "  ✅ 工作日程查询成功，找到 {} 个日程",
                    data.schedules.len()
                );
                for schedule in data.schedules.iter() {
                    if let Some(schedule_id) = &schedule.schedule_id {
                        println!("    📅 日程ID: {schedule_id}");
                    }
                }
            }
        }
        Err(e) => {
            println!("  ❌ 工作日程查询失败: {e:?}");
        }
    }

    Ok(())
}

/// 客服技能管理示例
async fn demo_agent_skill(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    use open_lark::service::helpdesk::{models::UserIdType, v1::agent_skill::*};

    // 创建客服技能
    let create_request = CreateAgentSkillRequest {
        skill_name: "技术支持".to_string(),
        description: Some("负责技术相关问题的解答".to_string()),
    };

    match client
        .helpdesk
        .v1
        .agent_skill
        .create(create_request, Some(UserIdType::OpenId), None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("  ✅ 客服技能创建成功");
                if let Some(skill_name) = data.skill.skill_name {
                    println!("    🎯 技能名称: {skill_name}");
                }
                if let Some(description) = data.skill.description {
                    println!("    📝 技能描述: {description}");
                }
            }
        }
        Err(e) => {
            println!("  ❌ 客服技能创建失败: {e:?}");
        }
    }

    // 查询全部客服技能
    match client
        .helpdesk
        .v1
        .agent_skill
        .list(Some(UserIdType::OpenId), None, Some(10), None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("  ✅ 客服技能查询成功，找到 {} 个技能", data.skills.len());
                for skill in data.skills.iter() {
                    if let Some(skill_name) = &skill.skill_name {
                        println!("    🎯 技能: {skill_name}");
                    }
                }
            }
        }
        Err(e) => {
            println!("  ❌ 客服技能查询失败: {e:?}");
        }
    }

    Ok(())
}

/// 工单管理示例
async fn demo_ticket_management(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    use open_lark::service::helpdesk::{models::UserIdType, v1::ticket::*};

    // 创建服务台对话
    let start_service_request = StartServiceRequest {
        open_id: "ou_example_user_open_id".to_string(),
        helpdesk_id: "helpdesk_example_id".to_string(),
        description: Some("用户遇到登录问题，需要技术支持".to_string()),
    };

    match client
        .helpdesk
        .v1
        .ticket
        .start_service(start_service_request, Some(UserIdType::OpenId), None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("  ✅ 服务台对话创建成功");
                println!("    💬 聊天群ID: {}", data.chat_id);
                if let Some(ticket) = data.ticket {
                    if let Some(ticket_id) = ticket.ticket_id {
                        println!("    🎫 工单ID: {ticket_id}");
                    }
                    if let Some(title) = ticket.title {
                        println!("    📋 工单标题: {title}");
                    }
                }
            }
        }
        Err(e) => {
            println!("  ❌ 服务台对话创建失败: {e:?}");
        }
    }

    // 查询全部工单详情
    match client
        .helpdesk
        .v1
        .ticket
        .list(Some(UserIdType::OpenId), None, Some(10), None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("  ✅ 工单查询成功，找到 {} 个工单", data.tickets.len());
                for ticket in data.tickets.iter() {
                    if let Some(ticket_id) = &ticket.ticket_id {
                        println!("    🎫 工单ID: {ticket_id}");
                    }
                    if let Some(status) = &ticket.status {
                        println!("    📊 状态: {status:?}");
                    }
                    if let Some(priority) = &ticket.priority {
                        println!("    🚨 优先级: {priority:?}");
                    }
                }
            }
        }
        Err(e) => {
            println!("  ❌ 工单查询失败: {e:?}");
        }
    }

    Ok(())
}
