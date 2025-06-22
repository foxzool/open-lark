use open_lark::{
    service::bitable::v1::app_table_field::{
        AppTableFieldProperty, AppTableFieldPropertyOption, CreateFieldRequest, DeleteFieldRequest,
        FieldType, ListFieldRequest, UpdateFieldRequest,
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
        .with_enable_token_cache(true)
        .build();

    // 配置必要参数
    let app_token = "bascnmBA*****yGehy8";
    let table_id = "tblsRc9GRRXKqhvW";

    // 1. 新增文本字段
    println!("1. 新增文本字段...");

    let text_property = AppTableFieldProperty::text();

    let create_request = CreateFieldRequest::builder()
        .app_token(app_token)
        .table_id(table_id)
        .field_name("文本字段示例")
        .field_type(FieldType::Text)
        .property(text_property)
        .build();

    let create_response = client
        .bitable
        .v1
        .app_table_field
        .create(create_request, None)
        .await?;
    println!("创建成功: {:?}", create_response.data);
    let field_id = create_response.data.field.field_id.clone();

    // 2. 新增单选字段
    println!("\n2. 新增单选字段...");

    let single_select_options = vec![
        AppTableFieldPropertyOption {
            name: "待办".to_string(),
            id: None,
            color: Some(1),
        },
        AppTableFieldPropertyOption {
            name: "进行中".to_string(),
            id: None,
            color: Some(2),
        },
        AppTableFieldPropertyOption {
            name: "已完成".to_string(),
            id: None,
            color: Some(3),
        },
    ];

    let select_property = AppTableFieldProperty::single_select(single_select_options);

    let single_select_request = CreateFieldRequest::builder()
        .app_token(app_token)
        .table_id(table_id)
        .field_name("任务状态")
        .field_type(FieldType::SingleSelect)
        .property(select_property)
        .build();

    let single_select_response = client
        .bitable
        .v1
        .app_table_field
        .create(single_select_request, None)
        .await?;
    println!("单选字段创建成功: {:?}", single_select_response.data);
    let single_select_field_id = single_select_response.data.field.field_id.clone();

    // 3. 新增数字字段
    println!("\n3. 新增数字字段...");

    let number_property = AppTableFieldProperty::number(Some("0.00".to_string()));

    let number_request = CreateFieldRequest::builder()
        .app_token(app_token)
        .table_id(table_id)
        .field_name("预算金额")
        .field_type(FieldType::Number)
        .property(number_property)
        .build();

    let number_response = client
        .bitable
        .v1
        .app_table_field
        .create(number_request, None)
        .await?;
    println!("数字字段创建成功: {:?}", number_response.data);
    let number_field_id = number_response.data.field.field_id.clone();

    // 4. 新增货币字段
    println!("\n4. 新增货币字段...");

    let currency_property = AppTableFieldProperty::currency("CNY".to_string());

    let currency_request = CreateFieldRequest::builder()
        .app_token(app_token)
        .table_id(table_id)
        .field_name("实际花费")
        .field_type(FieldType::Number)
        .property(currency_property)
        .build();

    let currency_response = client
        .bitable
        .v1
        .app_table_field
        .create(currency_request, None)
        .await?;
    println!("货币字段创建成功: {:?}", currency_response.data);
    let currency_field_id = currency_response.data.field.field_id.clone();

    // 5. 新增进度字段
    println!("\n5. 新增进度字段...");

    let progress_property = AppTableFieldProperty::progress(0.0, 100.0, true);

    let progress_request = CreateFieldRequest::builder()
        .app_token(app_token)
        .table_id(table_id)
        .field_name("完成进度")
        .field_type(FieldType::Number)
        .property(progress_property)
        .build();

    let progress_response = client
        .bitable
        .v1
        .app_table_field
        .create(progress_request, None)
        .await?;
    println!("进度字段创建成功: {:?}", progress_response.data);
    let progress_field_id = progress_response.data.field.field_id.clone();

    // 6. 列出所有字段
    println!("\n6. 列出所有字段...");

    let list_request = ListFieldRequest::builder()
        .app_token(app_token)
        .table_id(table_id)
        .page_size(20)
        .build();

    let list_response = client
        .bitable
        .v1
        .app_table_field
        .list(list_request, None)
        .await?;
    println!("字段列表:");
    for field in &list_response.data.items {
        println!(
            "  - {}: {} ({})",
            field.field_name, field.field_id, field.r#type as u16
        );
    }
    println!("总计 {} 个字段", list_response.data.total);

    // 7. 更新文本字段
    println!("\n7. 更新文本字段...");

    let update_request = UpdateFieldRequest::builder()
        .app_token(app_token)
        .table_id(table_id)
        .field_id(&field_id)
        .field_name("更新后的文本字段")
        .build();

    let update_response = client
        .bitable
        .v1
        .app_table_field
        .update(update_request, None)
        .await?;
    println!("字段更新成功: {:?}", update_response.data);

    // 8. 删除创建的字段（清理资源）
    println!("\n8. 清理创建的测试字段...");

    let test_fields = vec![
        (&field_id, "文本字段"),
        (&single_select_field_id, "单选字段"),
        (&number_field_id, "数字字段"),
        (&currency_field_id, "货币字段"),
        (&progress_field_id, "进度字段"),
    ];

    for (field_id, field_name) in test_fields {
        let delete_request = DeleteFieldRequest::builder()
            .app_token(app_token)
            .table_id(table_id)
            .field_id(field_id)
            .build();

        match client
            .bitable
            .v1
            .app_table_field
            .delete(delete_request, None)
            .await
        {
            Ok(delete_response) => {
                println!("删除{}成功: {:?}", field_name, delete_response.data);
            }
            Err(e) => {
                println!("删除{}失败: {:?}", field_name, e);
            }
        }
    }

    println!("\n字段操作示例完成！");
    Ok(())
}
