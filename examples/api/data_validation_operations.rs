use open_lark::{
    core::trait_system::ExecutableBuilder,
    prelude::*,
    service::cloud_docs::sheets::v3::data_validation::{
        create::{DataValidationRule, SetDataValidationRequest},
        delete::DeleteDataValidationRequest,
        query::QueryDataValidationsRequest,
        update::UpdateDataValidationRequest,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("请在.env文件中设置APP_ID");
    let app_secret = std::env::var("APP_SECRET").expect("请在.env文件中设置APP_SECRET");

    let client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .with_enable_token_cache(true)
        .build();

    let spreadsheet_token = "your_spreadsheet_token";
    let sheet_id = "your_sheet_id";

    println!("🚀 飞书表格数据校验操作示例");
    println!("{}", "=".repeat(50));

    // 示例1: 创建下拉列表数据校验
    println!("\n📝 创建下拉列表数据校验...");
    let validation_rule = DataValidationRule::dropdown(
        "A1:A10",
        vec![
            "选项1".to_string(),
            "选项2".to_string(),
            "选项3".to_string(),
        ],
    )
    .with_input_message("请选择一个选项")
    .with_error_message("输入的值不在选项列表中")
    .with_strict(true);

    match SetDataValidationRequest::builder()
        .spreadsheet_token(spreadsheet_token)
        .sheet_id(sheet_id)
        .data_validation(validation_rule)
        .execute(&client.sheets.v3.spreadsheet_sheet)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 数据校验创建成功:");
                println!("  📋 数据校验ID: {}", data.data_validation_id);
                println!("  🔧 校验类型: {}", data.data_validation.condition_type);
                println!("  📍 应用范围: {}", data.data_validation.range);

                let data_validation_id = data.data_validation_id.clone();

                // 示例2: 更新数据校验
                println!("\n🔄 更新数据校验...");
                let updated_rule = DataValidationRule::dropdown(
                    "A1:A15",
                    vec![
                        "新选项1".to_string(),
                        "新选项2".to_string(),
                        "新选项3".to_string(),
                        "新选项4".to_string(),
                    ],
                )
                .with_input_message("请从更新后的选项中选择")
                .with_error_message("输入无效，请选择列表中的选项");

                match UpdateDataValidationRequest::builder()
                    .spreadsheet_token(spreadsheet_token)
                    .sheet_id(sheet_id)
                    .data_validation_id(&data_validation_id)
                    .data_validation(updated_rule)
                    .execute(&client.sheets.v3.spreadsheet_sheet)
                    .await
                {
                    Ok(update_response) => {
                        if let Some(update_data) = update_response.data {
                            println!("✅ 数据校验更新成功:");
                            println!(
                                "  🔧 更新后的校验类型: {}",
                                update_data.data_validation.condition_type
                            );
                            println!(
                                "  📍 更新后的应用范围: {}",
                                update_data.data_validation.range
                            );
                        }
                    }
                    Err(e) => println!("❌ 更新数据校验失败: {:?}", e),
                }

                // 示例3: 查询数据校验
                println!("\n🔍 查询数据校验...");
                match QueryDataValidationsRequest::builder()
                    .spreadsheet_token(spreadsheet_token)
                    .sheet_id(sheet_id)
                    .range("A1:A20")
                    .execute(&client.sheets.v3.spreadsheet_sheet)
                    .await
                {
                    Ok(query_response) => {
                        if let Some(query_data) = query_response.data {
                            println!("✅ 查询到 {} 个数据校验规则:", query_data.items.len());
                            for (i, validation) in query_data.items.iter().enumerate() {
                                println!("  📋 规则 {}: ", i + 1);
                                println!("    🆔 ID: {}", validation.data_validation_id);
                                println!(
                                    "    🔧 类型: {}",
                                    validation.data_validation.condition_type
                                );
                                println!("    📍 范围: {}", validation.data_validation.range);
                                if let Some(ref values) =
                                    validation.data_validation.condition_values
                                {
                                    println!("    🎯 选项: {:?}", values);
                                }
                            }
                        }
                    }
                    Err(e) => println!("❌ 查询数据校验失败: {:?}", e),
                }

                // 示例4: 删除数据校验
                println!("\n🗑️ 删除数据校验...");
                match DeleteDataValidationRequest::builder()
                    .spreadsheet_token(spreadsheet_token)
                    .sheet_id(sheet_id)
                    .data_validation_id(&data_validation_id)
                    .execute(&client.sheets.v3.spreadsheet_sheet)
                    .await
                {
                    Ok(delete_response) => {
                        if let Some(delete_data) = delete_response.data {
                            if delete_data.success {
                                println!("✅ 数据校验删除成功!");
                                if let Some(ref id) = delete_data.data_validation_id {
                                    println!("  🗑️ 删除的校验ID: {}", id);
                                }
                            } else {
                                println!("❌ 数据校验删除失败");
                            }
                        }
                    }
                    Err(e) => println!("❌ 删除数据校验失败: {:?}", e),
                }
            } else {
                println!("❌ 创建数据校验失败: 无响应数据");
            }
        }
        Err(e) => println!("❌ 创建数据校验失败: {:?}", e),
    }

    // 示例5: 创建数字范围校验
    println!("\n🔢 创建数字范围校验...");
    let number_validation = DataValidationRule::number_range("B1:B10", 1.0, 100.0)
        .with_input_message("请输入1-100之间的数字")
        .with_error_message("数字必须在1-100范围内");

    match SetDataValidationRequest::builder()
        .spreadsheet_token(spreadsheet_token)
        .sheet_id(sheet_id)
        .data_validation(number_validation)
        .execute(&client.sheets.v3.spreadsheet_sheet)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 数字范围校验创建成功:");
                println!("  📋 数据校验ID: {}", data.data_validation_id);
                println!("  🔧 校验类型: {}", data.data_validation.condition_type);
            }
        }
        Err(e) => println!("❌ 创建数字范围校验失败: {:?}", e),
    }

    // 示例6: 创建文本长度校验
    println!("\n📝 创建文本长度校验...");
    let text_validation = DataValidationRule::text_length("C1:C10", 5, 20)
        .with_input_message("请输入5-20个字符的文本")
        .with_error_message("文本长度必须在5-20个字符之间");

    match SetDataValidationRequest::builder()
        .spreadsheet_token(spreadsheet_token)
        .sheet_id(sheet_id)
        .data_validation(text_validation)
        .execute(&client.sheets.v3.spreadsheet_sheet)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 文本长度校验创建成功:");
                println!("  📋 数据校验ID: {}", data.data_validation_id);
                println!("  🔧 校验类型: {}", data.data_validation.condition_type);
            }
        }
        Err(e) => println!("❌ 创建文本长度校验失败: {:?}", e),
    }

    println!("\n🎉 数据校验操作示例完成!");
    println!("\n💡 提示:");
    println!("  - 请在 .env 文件中设置正确的 APP_ID 和 APP_SECRET");
    println!("  - 请将 spreadsheet_token 和 sheet_id 替换为实际值");
    println!("  - 数据校验可以帮助确保单元格中输入的数据符合特定标准");
    println!("  - 支持下拉列表、数字范围、文本长度等多种校验类型");

    Ok(())
}
