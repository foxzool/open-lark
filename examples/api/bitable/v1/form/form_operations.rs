use open_lark::{
    core::config::{AppType, Config},
    service::bitable::v1::form::{
        GetFormRequest, ListFormQuestionRequest, PatchFormMetaRequest, PatchFormQuestionRequest,
    },
    LarkClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenvy::dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("APP_ID environment variable not set");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET environment variable not set");

    // 创建客户端
    let client = LarkClient::builder(app_id, app_secret)
        .with_app_type(AppType::SelfBuild)
        .with_enable_token_cache(true)
        .build();

    // 配置必要参数
    let app_token = "bascnmBA*****yGehy8";
    let form_id = "vewxxxxxx"; // 表单ID需要从实际的多维表格中获取

    // 1. 获取表单元数据
    println!("1. 获取表单元数据...");

    let get_request = GetFormRequest::builder()
        .app_token(app_token)
        .form_id(form_id)
        .build();

    match client.bitable.v1.form.get(get_request, None).await {
        Ok(get_response) => {
            let form = &get_response.data.form;
            println!("表单信息:");
            println!("  - 名称: {}", form.name);
            println!("  - ID: {}", form.form_id);
            println!("  - 描述: {:?}", form.description);
            println!("  - 允许重复提交: {}", form.allow_resubmit);
            println!("  - 是否分享: {}", form.shared);
            println!("  - 分享URL: {:?}", form.shared_url);
            println!("  - 状态: {}", form.status);
        }
        Err(e) => {
            println!("获取表单元数据失败: {:?}", e);
            println!("请确保表单ID正确，或者在多维表格中创建一个表单");
            return Ok(());
        }
    }

    // 2. 列出表单问题
    println!("\n2. 列出表单问题...");

    let list_request = ListFormQuestionRequest::builder()
        .app_token(app_token)
        .form_id(form_id)
        .page_size(20)
        .build();

    match client.bitable.v1.form.list(list_request, None).await {
        Ok(list_response) => {
            println!("表单问题列表:");
            for question in &list_response.data.items {
                println!("  - {}: {} (必填: {})", 
                    question.title, 
                    question.question_id, 
                    question.required
                );
                if let Some(desc) = &question.description {
                    println!("    描述: {}", desc);
                }
                println!("    类型: {}, 字段ID: {}", question.question_type, question.field_id);
            }
            println!("总计 {} 个问题", list_response.data.total);

            // 3. 更新第一个问题（如果存在）
            if !list_response.data.items.is_empty() {
                let first_question = &list_response.data.items[0];
                println!("\n3. 更新表单问题: {}", first_question.title);

                let patch_question_request = PatchFormQuestionRequest::builder()
                    .app_token(app_token)
                    .form_id(form_id)
                    .question_id(&first_question.question_id)
                    .title(format!("{} (已更新)", first_question.title))
                    .description("这是一个通过API更新的问题描述")
                    .required(!first_question.required) // 切换必填状态
                    .build();

                match client
                    .bitable
                    .v1
                    .form
                    .patch(patch_question_request, None)
                    .await
                {
                    Ok(patch_response) => {
                        println!("问题更新成功:");
                        println!("  - 新标题: {}", patch_response.data.question.title);
                        println!("  - 新描述: {:?}", patch_response.data.question.description);
                        println!("  - 必填状态: {}", patch_response.data.question.required);
                    }
                    Err(e) => {
                        println!("更新问题失败: {:?}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("列出表单问题失败: {:?}", e);
        }
    }

    // 4. 更新表单元数据
    println!("\n4. 更新表单元数据...");

    let patch_meta_request = PatchFormMetaRequest::builder()
        .app_token(app_token)
        .form_id(form_id)
        .name("通过API更新的表单")
        .description("这是一个通过API更新的表单描述")
        .allow_resubmit(true)
        .show_submit_button(true)
        .submit_button_text("提交表单")
        .shared(true)
        .need_login(false)
        .status("enabled")
        .build();

    match client
        .bitable
        .v1
        .form
        .patch_meta(patch_meta_request, None)
        .await
    {
        Ok(patch_meta_response) => {
            println!("表单元数据更新成功:");
            let updated_form = &patch_meta_response.data.form;
            println!("  - 新名称: {}", updated_form.name);
            println!("  - 新描述: {:?}", updated_form.description);
            println!("  - 允许重复提交: {}", updated_form.allow_resubmit);
            println!("  - 显示提交按钮: {}", updated_form.show_submit_button);
            println!("  - 提交按钮文本: {:?}", updated_form.submit_button_text);
            println!("  - 是否分享: {}", updated_form.shared);
            println!("  - 需要登录: {}", updated_form.need_login);
            println!("  - 状态: {}", updated_form.status);
        }
        Err(e) => {
            println!("更新表单元数据失败: {:?}", e);
        }
    }

    // 5. 再次获取表单元数据，验证更新结果
    println!("\n5. 再次获取表单元数据（验证更新结果）...");

    let verify_request = GetFormRequest::builder()
        .app_token(app_token)
        .form_id(form_id)
        .build();

    match client.bitable.v1.form.get(verify_request, None).await {
        Ok(verify_response) => {
            let form = &verify_response.data.form;
            println!("验证更新结果:");
            println!("  - 当前名称: {}", form.name);
            println!("  - 当前描述: {:?}", form.description);
            println!("  - 当前状态: {}", form.status);
        }
        Err(e) => {
            println!("验证更新结果失败: {:?}", e);
        }
    }

    println!("\n表单操作示例完成！");
    println!("注意：此示例需要一个包含表单的多维表格，请确保提供正确的表单ID");
    Ok(())
}