use dotenvy::dotenv;
use open_lark::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    
    let app_id = std::env::var("APP_ID").expect("APP_ID not found");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET not found");

    let client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .with_enable_token_cache(true)
        .build();

    // 设置下拉列表数据校验示例
    println!("--- 1. 设置下拉列表数据校验 ---");
    
    use open_lark::service::sheets::v3::data_validation::DataValidationRule;
    
    // 创建一个下拉列表校验规则
    let validation_rule = DataValidationRule::dropdown(
        "A1:A10", 
        vec!["优秀".to_string(), "良好".to_string(), "一般".to_string(), "较差".to_string()]
    )
    .with_input_message("请选择评价等级")
    .with_error_message("请从下拉列表中选择有效选项");
    
    let set_req = open_lark::service::sheets::v3::data_validation::SetDataValidationRequest::builder()
        .spreadsheet_token("shtcnmBA*****yGehy8") // 替换为实际的表格 token
        .sheet_id("Sheet1") // 替换为实际的工作表 ID
        .data_validation(validation_rule)
        .build();

    let validation_id = match client.sheets.v3.spreadsheet_sheet.set_data_validation(set_req, None).await {
        Ok(resp) => {
            if let Some(data) = resp.data {
                println!("✅ 设置下拉列表数据校验成功!");
                println!("🆔 数据校验 ID: {}", data.data_validation_id);
                println!("📋 校验类型: {}", data.data_validation.condition_type);
                println!("📊 应用范围: {}", data.data_validation.range);
                println!("📝 输入提示: {:?}", data.data_validation.input_message);
                println!("❌ 错误提示: {:?}", data.data_validation.error_message);
                data.data_validation_id
            } else {
                eprintln!("❌ 响应数据为空");
                return Ok(());
            }
        }
        Err(e) => {
            eprintln!("❌ 设置数据校验失败: {:?}", e);
            return Ok(());
        }
    };

    // 再设置一个数字范围校验
    println!("\n--- 2. 设置数字范围校验 ---");
    let number_validation = DataValidationRule::number_range("B1:B10", 1.0, 100.0)
        .with_input_message("请输入1到100之间的数字")
        .with_error_message("数字必须在1到100之间");
    
    let set_number_req = open_lark::service::sheets::v3::data_validation::SetDataValidationRequest::builder()
        .spreadsheet_token("shtcnmBA*****yGehy8")
        .sheet_id("Sheet1")
        .data_validation(number_validation)
        .build();

    let number_validation_id = match client.sheets.v3.spreadsheet_sheet.set_data_validation(set_number_req, None).await {
        Ok(resp) => {
            if let Some(data) = resp.data {
                println!("✅ 设置数字范围校验成功!");
                println!("🆔 数据校验 ID: {}", data.data_validation_id);
                println!("📋 校验类型: {}", data.data_validation.condition_type);
                println!("📊 应用范围: {}", data.data_validation.range);
                data.data_validation_id
            } else {
                eprintln!("❌ 响应数据为空");
                return Ok(());
            }
        }
        Err(e) => {
            eprintln!("❌ 设置数字范围校验失败: {:?}", e);
            return Ok(());
        }
    };

    // 查询所有数据校验设置
    println!("\n--- 3. 查询所有数据校验设置 ---");
    let query_req = open_lark::service::sheets::v3::data_validation::QueryDataValidationsRequest::builder()
        .spreadsheet_token("shtcnmBA*****yGehy8")
        .sheet_id("Sheet1")
        .build();

    match client.sheets.v3.spreadsheet_sheet.query_data_validations(query_req, None).await {
        Ok(resp) => {
            if let Some(data) = resp.data {
                println!("✅ 查询数据校验设置成功!");
                println!("📊 共找到 {} 个数据校验规则:", data.items.len());
                for (i, item) in data.items.iter().enumerate() {
                    println!("  {}. ID: {}, 类型: {}, 范围: {}, 严格模式: {}", 
                        i + 1, 
                        item.data_validation_id,
                        item.data_validation.condition_type,
                        item.data_validation.range,
                        item.data_validation.strict
                    );
                    if let Some(values) = &item.data_validation.condition_values {
                        println!("     条件值: {:?}", values);
                    }
                }
            } else {
                println!("❌ 响应数据为空");
            }
        }
        Err(e) => {
            eprintln!("❌ 查询数据校验设置失败: {:?}", e);
        }
    }

    // 更新数据校验设置
    println!("\n--- 4. 更新数据校验设置 ---");
    let new_validation_rule = DataValidationRule::dropdown(
        "A1:A15", // 扩大范围
        vec!["优秀".to_string(), "良好".to_string(), "一般".to_string(), "较差".to_string(), "未评价".to_string()] // 增加选项
    )
    .with_input_message("请选择新的评价等级")
    .with_error_message("请从下拉列表中选择有效的新选项")
    .with_strict(false); // 改为非严格模式
    
    let update_req = open_lark::service::sheets::v3::data_validation::UpdateDataValidationRequest::builder()
        .spreadsheet_token("shtcnmBA*****yGehy8")
        .sheet_id("Sheet1")
        .data_validation_id(&validation_id)
        .data_validation(new_validation_rule)
        .build();

    match client.sheets.v3.spreadsheet_sheet.update_data_validation(update_req, None).await {
        Ok(resp) => {
            if let Some(data) = resp.data {
                println!("✅ 更新数据校验设置成功!");
                println!("🆔 数据校验 ID: {}", data.data_validation_id);
                println!("📊 新的应用范围: {}", data.data_validation.range);
                println!("🔒 严格模式: {}", data.data_validation.strict);
                if let Some(values) = &data.data_validation.condition_values {
                    println!("📝 新的条件值: {:?}", values);
                }
            } else {
                println!("❌ 响应数据为空");
            }
        }
        Err(e) => {
            eprintln!("❌ 更新数据校验设置失败: {:?}", e);
        }
    }

    // 删除数据校验设置 (演示用)
    println!("\n--- 5. 删除数据校验设置 (演示用) ---");
    println!("⚠️  注意：这将永久删除数据校验设置!");
    
    // 取消注释以下代码来执行删除操作
    /*
    let delete_req = open_lark::service::sheets::v3::data_validation::DeleteDataValidationRequest::builder()
        .spreadsheet_token("shtcnmBA*****yGehy8")
        .sheet_id("Sheet1")
        .data_validation_id(&number_validation_id)
        .build();

    match client.sheets.v3.spreadsheet_sheet.delete_data_validation(delete_req, None).await {
        Ok(resp) => {
            if let Some(data) = resp.data {
                println!("✅ 删除数据校验设置成功: {}", data.success);
                if let Some(deleted_id) = data.data_validation_id {
                    println!("🗑️  已删除数据校验 ID: {}", deleted_id);
                }
            } else {
                println!("❌ 响应数据为空");
            }
        }
        Err(e) => {
            eprintln!("❌ 删除数据校验设置失败: {:?}", e);
        }
    }
    */

    println!("\n💡 数据校验功能说明:");
    println!("- 数据校验用于限制单元格可以输入的数据类型和范围");
    println!("- 支持多种校验类型：下拉列表、数字范围、文本长度等");
    println!("- 可以设置输入提示和错误提示消息");
    println!("- 支持严格模式和非严格模式");
    
    println!("\n📋 支持的校验类型:");
    println!("- dropdown: 下拉列表，用户只能从预设选项中选择");
    println!("- number_between: 数字范围，限制数字在指定范围内");
    println!("- text_length: 文本长度，限制文本的最小和最大长度");
    
    println!("\n⚙️ 校验设置说明:");
    println!("- strict: true = 严格模式，拒绝无效输入");
    println!("- strict: false = 非严格模式，允许无效输入但显示警告");
    println!("- input_message: 用户选择单元格时显示的提示");
    println!("- error_message: 用户输入无效数据时显示的错误信息");

    Ok(())
}