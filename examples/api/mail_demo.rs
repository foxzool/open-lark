use open_lark::{
    prelude::*,
    service::mail::{
        models::{MailAddress, MailBody, UserIdType},
        v1::{
            contact::CreateContactRequest,
            event::SubscribeEventRequest,
            folder::{CreateFolderRequest, UpdateFolderRequest},
            mail_group::CreateMailGroupRequest,
            message::SendMessageRequest,
            rule::CreateRuleRequest,
        },
    },
};

/// Mail v1 API 演示程序
///
/// 展示邮箱管理系统的主要功能：
/// - 邮箱文件夹管理（创建、修改、列出、删除）
/// - 用户邮件管理（发送、获取、列出）
/// - 邮件附件处理
/// - 事件订阅管理
/// - 收信规则管理
/// - 邮箱联系人管理
/// - 邮件组管理
/// - 公共邮箱管理等
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("APP_ID must be set");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET must be set");

    let client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .with_enable_token_cache(true)
        .build();

    println!("=== 邮箱服务演示 ===");

    // 示例用户邮箱ID（实际使用时需要替换为真实的用户邮箱ID）
    let user_mailbox_id = "example_user_mailbox_id";

    // 1. 邮箱文件夹管理演示
    println!("\n1. 邮箱文件夹管理演示");

    // 创建邮箱文件夹
    println!("创建邮箱文件夹...");
    let create_folder_req = CreateFolderRequest {
        folder_name: "测试文件夹".to_string(),
        parent_folder_id: None,
    };

    match client
        .mail
        .v1
        .folder
        .create(
            user_mailbox_id,
            create_folder_req,
            Some(UserIdType::OpenId),
            None,
        )
        .await
    {
        Ok(response) => {
            println!("✓ 文件夹创建成功: {:?}", response.data);

            if let Some(folder_data) = &response.data {
                if let Some(folder_id) = &folder_data.folder.folder_id {
                    // 修改文件夹
                    println!("修改邮箱文件夹...");
                    let update_folder_req = UpdateFolderRequest {
                        folder_name: Some("修改后的文件夹".to_string()),
                        parent_folder_id: None,
                    };

                    match client
                        .mail
                        .v1
                        .folder
                        .patch(
                            user_mailbox_id,
                            folder_id,
                            update_folder_req,
                            Some(UserIdType::OpenId),
                            None,
                        )
                        .await
                    {
                        Ok(response) => println!("✓ 文件夹修改成功: {:?}", response.data),
                        Err(e) => println!("✗ 文件夹修改失败: {e:?}"),
                    }

                    // 删除文件夹
                    println!("删除邮箱文件夹...");
                    match client
                        .mail
                        .v1
                        .folder
                        .delete(user_mailbox_id, folder_id, Some(UserIdType::OpenId), None)
                        .await
                    {
                        Ok(_) => println!("✓ 文件夹删除成功"),
                        Err(e) => println!("✗ 文件夹删除失败: {e:?}"),
                    }
                }
            }
        }
        Err(e) => println!("✗ 文件夹创建失败: {e:?}"),
    }

    // 列出邮箱文件夹
    println!("列出邮箱文件夹...");
    match client
        .mail
        .v1
        .folder
        .list(
            user_mailbox_id,
            Some(20),
            None,
            Some(UserIdType::OpenId),
            None,
        )
        .await
    {
        Ok(response) => println!(
            "✓ 获取文件夹列表成功: {} 个文件夹",
            response.data.map(|d| d.folders.len()).unwrap_or(0)
        ),
        Err(e) => println!("✗ 获取文件夹列表失败: {e:?}"),
    }

    // 2. 用户邮件管理演示
    println!("\n2. 用户邮件管理演示");

    // 发送邮件
    println!("发送邮件...");
    let send_message_req = SendMessageRequest {
        to: vec![MailAddress {
            email: Some("recipient@example.com".to_string()),
            name: Some("测试收件人".to_string()),
        }],
        cc: None,
        bcc: None,
        subject: "测试邮件主题".to_string(),
        body: MailBody {
            text: Some("这是一封测试邮件的文本内容".to_string()),
            html: Some("<p>这是一封测试邮件的HTML内容</p>".to_string()),
        },
        attachment_ids: None,
    };

    match client
        .mail
        .v1
        .message
        .send(
            user_mailbox_id,
            send_message_req,
            Some(UserIdType::OpenId),
            None,
        )
        .await
    {
        Ok(response) => {
            println!("✓ 邮件发送成功，邮件ID: {:?}", response.data);

            if let Some(message_data) = &response.data {
                // 获取邮件详情
                println!("获取邮件详情...");
                match client
                    .mail
                    .v1
                    .message
                    .get(
                        user_mailbox_id,
                        &message_data.message_id,
                        Some(UserIdType::OpenId),
                        None,
                    )
                    .await
                {
                    Ok(response) => println!("✓ 获取邮件详情成功: {:?}", response.data),
                    Err(e) => println!("✗ 获取邮件详情失败: {e:?}"),
                }
            }
        }
        Err(e) => println!("✗ 邮件发送失败: {e:?}"),
    }

    // 列出邮件
    println!("列出邮件...");
    match client
        .mail
        .v1
        .message
        .list(
            user_mailbox_id,
            None,
            Some(20),
            None,
            Some(UserIdType::OpenId),
            None,
        )
        .await
    {
        Ok(response) => println!(
            "✓ 获取邮件列表成功: {} 封邮件",
            response.data.map(|d| d.messages.len()).unwrap_or(0)
        ),
        Err(e) => println!("✗ 获取邮件列表失败: {e:?}"),
    }

    // 3. 邮箱联系人管理演示
    println!("\n3. 邮箱联系人管理演示");

    // 创建邮箱联系人
    println!("创建邮箱联系人...");
    let create_contact_req = CreateContactRequest {
        name: "测试联系人".to_string(),
        email: "contact@example.com".to_string(),
        remark: Some("这是一个测试联系人".to_string()),
    };

    match client
        .mail
        .v1
        .contact
        .create(
            user_mailbox_id,
            create_contact_req,
            Some(UserIdType::OpenId),
            None,
        )
        .await
    {
        Ok(response) => println!("✓ 联系人创建成功: {:?}", response.data),
        Err(e) => println!("✗ 联系人创建失败: {e:?}"),
    }

    // 列出邮箱联系人
    println!("列出邮箱联系人...");
    match client
        .mail
        .v1
        .contact
        .list(
            user_mailbox_id,
            Some(20),
            None,
            Some(UserIdType::OpenId),
            None,
        )
        .await
    {
        Ok(response) => println!(
            "✓ 获取联系人列表成功: {} 个联系人",
            response.data.map(|d| d.contacts.len()).unwrap_or(0)
        ),
        Err(e) => println!("✗ 获取联系人列表失败: {e:?}"),
    }

    // 4. 收信规则管理演示
    println!("\n4. 收信规则管理演示");

    // 创建收信规则
    println!("创建收信规则...");
    let create_rule_req = CreateRuleRequest {
        rule_name: "测试规则".to_string(),
        enabled: Some(true),
        conditions: vec![serde_json::json!({
            "field": "subject",
            "operator": "contains",
            "value": "测试"
        })],
        actions: vec![serde_json::json!({
            "action_type": "move_to_folder",
            "parameters": {
                "folder_id": "test_folder_id"
            }
        })],
    };

    match client
        .mail
        .v1
        .rule
        .create(
            user_mailbox_id,
            create_rule_req,
            Some(UserIdType::OpenId),
            None,
        )
        .await
    {
        Ok(response) => println!("✓ 收信规则创建成功: {:?}", response.data),
        Err(e) => println!("✗ 收信规则创建失败: {e:?}"),
    }

    // 列出收信规则
    println!("列出收信规则...");
    match client
        .mail
        .v1
        .rule
        .list(
            user_mailbox_id,
            Some(20),
            None,
            Some(UserIdType::OpenId),
            None,
        )
        .await
    {
        Ok(response) => println!(
            "✓ 获取收信规则列表成功: {} 条规则",
            response.data.map(|d| d.rules.len()).unwrap_or(0)
        ),
        Err(e) => println!("✗ 获取收信规则列表失败: {e:?}"),
    }

    // 5. 邮件组管理演示
    println!("\n5. 邮件组管理演示");

    // 创建邮件组
    println!("创建邮件组...");
    let create_mailgroup_req = CreateMailGroupRequest {
        name: "测试邮件组".to_string(),
        email: "test-group@example.com".to_string(),
        description: Some("这是一个测试邮件组".to_string()),
        allow_external_send: Some(false),
    };

    match client
        .mail
        .v1
        .mail_group
        .create(create_mailgroup_req, Some(UserIdType::OpenId), None)
        .await
    {
        Ok(response) => {
            println!("✓ 邮件组创建成功: {:?}", response.data);

            if let Some(mailgroup_data) = &response.data {
                if let Some(mailgroup_id) = &mailgroup_data.mailgroup.mailgroup_id {
                    // 获取邮件组详情
                    println!("获取邮件组详情...");
                    match client
                        .mail
                        .v1
                        .mail_group
                        .get(mailgroup_id, Some(UserIdType::OpenId), None)
                        .await
                    {
                        Ok(response) => println!("✓ 获取邮件组详情成功: {:?}", response.data),
                        Err(e) => println!("✗ 获取邮件组详情失败: {e:?}"),
                    }
                }
            }
        }
        Err(e) => println!("✗ 邮件组创建失败: {e:?}"),
    }

    // 列出邮件组
    println!("列出邮件组...");
    match client
        .mail
        .v1
        .mail_group
        .list(Some(20), None, Some(UserIdType::OpenId), None)
        .await
    {
        Ok(response) => println!(
            "✓ 获取邮件组列表成功: {} 个邮件组",
            response.data.map(|d| d.mailgroups.len()).unwrap_or(0)
        ),
        Err(e) => println!("✗ 获取邮件组列表失败: {e:?}"),
    }

    // 6. 事件订阅管理演示
    println!("\n6. 事件订阅管理演示");

    // 订阅事件
    println!("订阅邮件事件...");
    let subscribe_event_req = SubscribeEventRequest {
        event_type: "message_received".to_string(),
        callback_url: "https://example.com/webhook".to_string(),
    };

    match client
        .mail
        .v1
        .event
        .subscribe(
            user_mailbox_id,
            subscribe_event_req,
            Some(UserIdType::OpenId),
            None,
        )
        .await
    {
        Ok(response) => println!("✓ 事件订阅成功: {:?}", response.data),
        Err(e) => println!("✗ 事件订阅失败: {e:?}"),
    }

    // 获取订阅状态
    println!("获取订阅状态...");
    match client
        .mail
        .v1
        .event
        .subscription(user_mailbox_id, Some(UserIdType::OpenId), None)
        .await
    {
        Ok(response) => println!("✓ 获取订阅状态成功: {:?}", response.data),
        Err(e) => println!("✗ 获取订阅状态失败: {e:?}"),
    }

    println!("\n=== 邮箱服务演示完成 ===");
    println!("注意：本演示使用的是示例数据，实际使用时需要：");
    println!("1. 配置正确的APP_ID和APP_SECRET");
    println!("2. 使用真实的用户邮箱ID和邮件组ID");
    println!("3. 根据实际需求调整请求参数");
    println!("4. 处理API返回的错误和边界情况");

    Ok(())
}
