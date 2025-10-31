//! Approval模块使用示例
//!
//! 演示如何使用approval模块进行审批管理操作，包括：
//! - 审批实例管理
//! - 审批任务处理
//! - 审批定义配置

use open_lark::prelude::*;
use open_lark::service::approval::v4::{
    ApprovalBaseResponse, ApprovalInstance, CreateApprovalRequest, CreateApprovalResponse,
    CreateInstanceRequest, CreateInstanceResponse, ProcessTaskResponse,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    env_logger::init();

    println!("🚀 Approval模块示例演示");

    // 创建客户端（这里使用测试配置）
    let client = LarkClient::builder("test_app_id", "test_app_secret")
        .with_app_type(AppType::SelfBuild)
        .build();

    println!("✅ 客户端创建成功");

    // 演示创建审批实例
    println!("\n📋 创建审批实例");
    let create_instance_request = CreateInstanceRequest {
        approval_code: "approval_leave_001".to_string(),
        form: Some(serde_json::json!({
            "leave_type": "年假",
            "start_date": "2024-01-15",
            "end_date": "2024-01-17",
            "reason": "家庭聚会"
        })),
        user_id: Some("user_12345".to_string()),
        user_id_type: Some("open_id".to_string()),
        department_id: None,
        department_id_type: None,
        uuid: None,
    };

    match client
        .approval
        .v4
        .instance
        .create(&create_instance_request)
        .await
    {
        Ok(response) => {
            println!("✅ 审批实例创建成功");
            if let Some(data) = response.data {
                println!("   实例ID: {}", data.instance_code);
                println!("   实例UUID: {}", data.uuid);
            }
        }
        Err(e) => {
            println!("❌ 审批实例创建失败: {}", e);
        }
    }

    // 演示获取审批实例详情
    println!("\n📋 获取审批实例详情");
    match client
        .approval
        .v4
        .instance
        .get("inst_001", Some("open_id"))
        .await
    {
        Ok(response) => {
            println!("✅ 审批实例详情获取成功");
            if let Some(data) = response.data {
                println!("   实例编码: {}", data.instance_code);
                println!("   审批定义: {}", data.approval_name.unwrap_or_default());
                println!("   发起人: {:?}", data.initiator.unwrap_or_default().name);
                println!("   审批状态: {:?}", data.status);
                println!("   创建时间: {:?}", data.create_time);
                if let Some(form_data) = data.form_data {
                    println!(
                        "   表单数据: {}",
                        serde_json::to_string_pretty(&form_data).unwrap_or_default()
                    );
                }
            }
        }
        Err(e) => {
            println!("❌ 审批实例详情获取失败: {}", e);
        }
    }

    // 演示同意审批任务
    println!("\n📋 同意审批任务");
    match client
        .approval
        .v4
        .task
        .approve(
            "task_001",
            Some("同意请假申请".to_string()),
            Some("open_id"),
        )
        .await
    {
        Ok(response) => {
            println!("✅ 审批任务同意成功");
            if let Some(data) = response.data {
                println!("   任务ID: {}", data.task_id);
                println!("   处理结果: {}", data.success);
                println!("   消息: {:?}", data.message);
            }
        }
        Err(e) => {
            println!("❌ 审批任务同意失败: {}", e);
        }
    }

    // 演示创建审批定义
    println!("\n📋 创建审批定义");
    let create_approval_request = CreateApprovalRequest {
        approval_name: "报销审批".to_string(),
        description: Some("员工费用报销审批流程".to_string()),
        form: None,
        process: None,
        permissions: None,
    };

    match client
        .approval
        .v4
        .approval
        .create(&create_approval_request)
        .await
    {
        Ok(response) => {
            println!("✅ 审批定义创建成功");
            if let Some(data) = response.data {
                println!("   审批编码: {}", data.approval_code);
                println!("   审批名称: {}", data.approval_name);
            }
        }
        Err(e) => {
            println!("❌ 审批定义创建失败: {}", e);
        }
    }

    println!("\n🎉 Approval模块示例演示完成！");
    Ok(())
}
