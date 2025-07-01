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

    println!("🔐 开始审批系统演示...");

    // 1. 创建审批定义示例
    println!("\n📝 创建审批定义示例");
    demo_create_approval(&client).await?;

    // 2. 创建审批实例示例
    println!("\n📋 创建审批实例示例");
    demo_create_instance(&client).await?;

    // 3. 任务操作示例
    println!("\n✅ 审批任务操作示例");
    demo_task_operations(&client).await?;

    // 4. 文件上传示例
    println!("\n📎 文件上传示例");
    demo_file_upload(&client).await?;

    // 5. 评论操作示例
    println!("\n💬 评论操作示例");
    demo_comments(&client).await?;

    // 6. 三方审批示例
    println!("\n🔗 三方审批示例");
    demo_external_approval(&client).await?;

    // 7. 查询操作示例
    println!("\n🔍 查询操作示例");
    demo_search(&client).await?;

    println!("\n✅ 审批系统演示完成!");
    Ok(())
}

/// 创建审批定义示例
async fn demo_create_approval(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    use open_lark::service::approval::{models::*, v4::approval::*};

    let form_data = serde_json::json!([
        {
            "id": "reason",
            "name": "申请理由",
            "type": "textarea",
            "required": true
        },
        {
            "id": "amount",
            "name": "申请金额",
            "type": "number",
            "required": true
        }
    ]);

    let request = CreateApprovalRequest {
        approval_name: "费用申请".to_string(),
        description: Some("用于费用申请的审批流程".to_string()),
        form: Some(form_data),
        process: None,
        settings: None,
    };

    match client
        .approval
        .v4
        .approval
        .create(
            request,
            Some(UserIdType::OpenId),
            Some(DepartmentIdType::DepartmentId),
            None,
        )
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("  ✅ 审批定义创建成功: {}", data.approval_code);
            }
        }
        Err(e) => {
            println!("  ❌ 审批定义创建失败: {e:?}");
        }
    }

    Ok(())
}

/// 创建审批实例示例
async fn demo_create_instance(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    use open_lark::service::approval::{models::*, v4::instance::*};

    let form_data = serde_json::json!({
        "reason": "出差费用报销",
        "amount": 5000
    });

    let request = CreateInstanceRequest {
        approval_code: "APPROVAL_CODE_EXAMPLE".to_string(),
        form: Some(form_data),
        user_id: Some("ou_example_user_id".to_string()),
        department_id: None,
        uuid: None,
    };

    match client
        .approval
        .v4
        .instance
        .create(
            request,
            Some(UserIdType::OpenId),
            Some(DepartmentIdType::DepartmentId),
            None,
        )
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("  ✅ 审批实例创建成功: {}", data.instance_code);
            }
        }
        Err(e) => {
            println!("  ❌ 审批实例创建失败: {e:?}");
        }
    }

    Ok(())
}

/// 任务操作示例
async fn demo_task_operations(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    use open_lark::service::approval::{models::*, v4::task::*};

    let task_id = "TASK_ID_EXAMPLE";

    // 同意任务
    let approve_request = ApproveTaskRequest {
        comment: Some("同意申请".to_string()),
        form: None,
    };

    match client
        .approval
        .v4
        .task
        .approve(task_id, approve_request, Some(UserIdType::OpenId), None)
        .await
    {
        Ok(_) => {
            println!("  ✅ 任务同意操作成功");
        }
        Err(e) => {
            println!("  ❌ 任务同意操作失败: {e:?}");
        }
    }

    Ok(())
}

/// 文件上传示例
async fn demo_file_upload(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    let file_content = b"This is a test file content".to_vec();
    let file_name = "test_document.txt";

    match client
        .approval
        .v4
        .file
        .upload(file_name, file_content, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("  ✅ 文件上传成功: {:?}", data.file);
            }
        }
        Err(e) => {
            println!("  ❌ 文件上传失败: {e:?}");
        }
    }

    Ok(())
}

/// 评论操作示例
async fn demo_comments(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    use open_lark::service::approval::{models::*, v4::instance_comment::*};

    let instance_code = "INSTANCE_CODE_EXAMPLE";

    let request = CreateCommentRequest {
        content: "这是一个审批评论".to_string(),
        attachments: None,
    };

    match client
        .approval
        .v4
        .instance_comment
        .create(instance_code, request, Some(UserIdType::OpenId), None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("  ✅ 评论创建成功: {}", data.comment_id);
            }
        }
        Err(e) => {
            println!("  ❌ 评论创建失败: {e:?}");
        }
    }

    Ok(())
}

/// 三方审批示例
async fn demo_external_approval(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    use open_lark::service::approval::{models::*, v4::external_approval::*};

    let request = CreateExternalApprovalRequest {
        approval_name: "外部系统审批".to_string(),
        description: Some("连接外部审批系统".to_string()),
        external_url: "https://external-approval-system.com".to_string(),
        callback_url: Some("https://callback.example.com".to_string()),
        config: None,
    };

    match client
        .approval
        .v4
        .external_approval
        .create(
            request,
            Some(UserIdType::OpenId),
            Some(DepartmentIdType::DepartmentId),
            None,
        )
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("  ✅ 三方审批创建成功: {}", data.approval_code);
            }
        }
        Err(e) => {
            println!("  ❌ 三方审批创建失败: {e:?}");
        }
    }

    Ok(())
}

/// 查询操作示例
async fn demo_search(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    use open_lark::service::approval::{models::UserIdType, v4::search::*};

    // 查询实例列表
    let params = SearchInstanceParams {
        page_size: Some(10),
        approval_code: Some("APPROVAL_CODE_EXAMPLE".to_string()),
        ..Default::default()
    };

    match client
        .approval
        .v4
        .search
        .instances(Some(params), None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("  ✅ 实例查询成功，找到 {} 个实例", data.instances.len());
            }
        }
        Err(e) => {
            println!("  ❌ 实例查询失败: {e:?}");
        }
    }

    // 查询审批ID
    match client
        .approval
        .v4
        .search
        .approval_id(Some("费用申请"), Some(UserIdType::OpenId), None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!(
                    "  ✅ 审批ID查询成功，找到 {} 个审批",
                    data.approval_list.len()
                );
            }
        }
        Err(e) => {
            println!("  ❌ 审批ID查询失败: {e:?}");
        }
    }

    Ok(())
}
