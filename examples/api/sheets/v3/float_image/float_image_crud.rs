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

    // 创建浮动图片示例
    println!("--- 1. 创建浮动图片 ---");

    use open_lark::service::sheets::v3::float_image::{FloatImageData, ImagePosition, ImageSize};

    // 创建图片位置和大小
    let position = ImagePosition::new(2, 3).with_offset(15.0, 25.0);
    let size = ImageSize::new(300.0, 200.0);

    // 创建浮动图片（需要先上传图片获取image_token）
    let float_image =
        FloatImageData::new("img_token_example_123", position, size).with_name("示例浮动图片");

    let create_req =
        open_lark::service::sheets::v3::float_image::CreateFloatImageRequest::builder()
            .spreadsheet_token("shtcnmBA*****yGehy8") // 替换为实际的表格 token
            .sheet_id("Sheet1") // 替换为实际的工作表 ID
            .float_image(float_image)
            .build();

    let float_image_id = match client
        .sheets
        .v3
        .spreadsheet_sheet
        .create_float_image(create_req, None)
        .await
    {
        Ok(resp) => {
            if let Some(data) = resp.data {
                println!("✅ 创建浮动图片成功!");
                println!("🆔 浮动图片 ID: {}", data.float_image_id);
                println!("🖼️  图片 Token: {}", data.float_image.image_token);
                println!(
                    "📍 位置: 列={}, 行={}, 偏移=({}, {})",
                    data.float_image.position.start_col_index,
                    data.float_image.position.start_row_index,
                    data.float_image.position.offset_x,
                    data.float_image.position.offset_y
                );
                println!(
                    "📏 大小: {}x{} 像素",
                    data.float_image.size.width, data.float_image.size.height
                );
                if let Some(name) = &data.float_image.name {
                    println!("📝 名称: {}", name);
                }
                data.float_image_id
            } else {
                eprintln!("❌ 响应数据为空");
                return Ok(());
            }
        }
        Err(e) => {
            eprintln!("❌ 创建浮动图片失败: {:?}", e);
            return Ok(());
        }
    };

    // 创建另一个浮动图片
    println!("\n--- 2. 创建第二个浮动图片 ---");
    let position2 = ImagePosition::new(5, 6);
    let size2 = ImageSize::square(150.0); // 正方形图片
    let float_image2 =
        FloatImageData::new("img_token_example_456", position2, size2).with_name("第二张图片");

    let create_req2 =
        open_lark::service::sheets::v3::float_image::CreateFloatImageRequest::builder()
            .spreadsheet_token("shtcnmBA*****yGehy8")
            .sheet_id("Sheet1")
            .float_image(float_image2)
            .build();

    let float_image_id2 = match client
        .sheets
        .v3
        .spreadsheet_sheet
        .create_float_image(create_req2, None)
        .await
    {
        Ok(resp) => {
            if let Some(data) = resp.data {
                println!("✅ 创建第二个浮动图片成功!");
                println!("🆔 浮动图片 ID: {}", data.float_image_id);
                data.float_image_id
            } else {
                eprintln!("❌ 响应数据为空");
                return Ok(());
            }
        }
        Err(e) => {
            eprintln!("❌ 创建第二个浮动图片失败: {:?}", e);
            return Ok(());
        }
    };

    // 查询所有浮动图片
    println!("\n--- 3. 查询所有浮动图片 ---");
    let query_req = open_lark::service::sheets::v3::float_image::QueryFloatImagesRequest::builder()
        .spreadsheet_token("shtcnmBA*****yGehy8")
        .sheet_id("Sheet1")
        .build();

    match client
        .sheets
        .v3
        .spreadsheet_sheet
        .query_float_images(query_req, None)
        .await
    {
        Ok(resp) => {
            if let Some(data) = resp.data {
                println!("✅ 查询浮动图片成功!");
                println!("📊 共找到 {} 个浮动图片:", data.items.len());
                for (i, item) in data.items.iter().enumerate() {
                    println!(
                        "  {}. ID: {}, 图片Token: {}, 位置: ({}, {}), 大小: {}x{}",
                        i + 1,
                        item.float_image_id,
                        item.float_image.image_token,
                        item.float_image.position.start_col_index,
                        item.float_image.position.start_row_index,
                        item.float_image.size.width,
                        item.float_image.size.height
                    );
                    if let Some(name) = &item.float_image.name {
                        println!("     名称: {}", name);
                    }
                }
            } else {
                println!("❌ 响应数据为空");
            }
        }
        Err(e) => {
            eprintln!("❌ 查询浮动图片失败: {:?}", e);
        }
    }

    // 获取特定浮动图片
    println!("\n--- 4. 获取特定浮动图片 ---");
    let get_req = open_lark::service::sheets::v3::float_image::GetFloatImageRequest::builder()
        .spreadsheet_token("shtcnmBA*****yGehy8")
        .sheet_id("Sheet1")
        .float_image_id(&float_image_id)
        .build();

    match client
        .sheets
        .v3
        .spreadsheet_sheet
        .get_float_image(get_req, None)
        .await
    {
        Ok(resp) => {
            if let Some(data) = resp.data {
                println!("✅ 获取浮动图片详情成功!");
                println!("🆔 图片 ID: {}", data.float_image.float_image_id);
                println!(
                    "🖼️  图片 Token: {}",
                    data.float_image.float_image.image_token
                );
                println!(
                    "📍 位置详情: 列={}, 行={}, X偏移={}, Y偏移={}",
                    data.float_image.float_image.position.start_col_index,
                    data.float_image.float_image.position.start_row_index,
                    data.float_image.float_image.position.offset_x,
                    data.float_image.float_image.position.offset_y
                );
                println!(
                    "📏 尺寸: 宽={}像素, 高={}像素",
                    data.float_image.float_image.size.width,
                    data.float_image.float_image.size.height
                );
            } else {
                println!("❌ 响应数据为空");
            }
        }
        Err(e) => {
            eprintln!("❌ 获取浮动图片详情失败: {:?}", e);
        }
    }

    // 更新浮动图片
    println!("\n--- 5. 更新浮动图片 ---");
    let new_position = ImagePosition::new(1, 1).with_offset(50.0, 50.0);
    let new_size = ImageSize::new(400.0, 300.0);
    let updated_float_image = FloatImageData::new("img_token_example_789", new_position, new_size)
        .with_name("更新后的浮动图片");

    let update_req =
        open_lark::service::sheets::v3::float_image::UpdateFloatImageRequest::builder()
            .spreadsheet_token("shtcnmBA*****yGehy8")
            .sheet_id("Sheet1")
            .float_image_id(&float_image_id)
            .float_image(updated_float_image)
            .build();

    match client
        .sheets
        .v3
        .spreadsheet_sheet
        .update_float_image(update_req, None)
        .await
    {
        Ok(resp) => {
            if let Some(data) = resp.data {
                println!("✅ 更新浮动图片成功!");
                println!("🆔 图片 ID: {}", data.float_image_id);
                println!("🖼️  新图片 Token: {}", data.float_image.image_token);
                println!(
                    "📍 新位置: 列={}, 行={}, 偏移=({}, {})",
                    data.float_image.position.start_col_index,
                    data.float_image.position.start_row_index,
                    data.float_image.position.offset_x,
                    data.float_image.position.offset_y
                );
                println!(
                    "📏 新大小: {}x{} 像素",
                    data.float_image.size.width, data.float_image.size.height
                );
            } else {
                println!("❌ 响应数据为空");
            }
        }
        Err(e) => {
            eprintln!("❌ 更新浮动图片失败: {:?}", e);
        }
    }

    // 删除浮动图片 (演示用)
    println!("\n--- 6. 删除浮动图片 (演示用) ---");
    println!("⚠️  注意：这将永久删除浮动图片!");

    // 取消注释以下代码来执行删除操作
    // let delete_req = open_lark::service::sheets::v3::float_image::DeleteFloatImageRequest::builder()
    // .spreadsheet_token("shtcnmBA*****yGehy8")
    // .sheet_id("Sheet1")
    // .float_image_id(&float_image_id2)
    // .build();
    //
    // match client.sheets.v3.spreadsheet_sheet.delete_float_image(delete_req, None).await {
    // Ok(resp) => {
    // if let Some(data) = resp.data {
    // println!("✅ 删除浮动图片成功: {}", data.success);
    // if let Some(deleted_id) = data.float_image_id {
    // println!("🗑️  已删除浮动图片 ID: {}", deleted_id);
    // }
    // } else {
    // println!("❌ 响应数据为空");
    // }
    // }
    // Err(e) => {
    // eprintln!("❌ 删除浮动图片失败: {:?}", e);
    // }
    // }

    println!("\n💡 浮动图片功能说明:");
    println!("- 浮动图片是悬浮在工作表上方的图片，不占用单元格内容");
    println!("- 可以精确控制图片的位置、大小和显示层级");
    println!("- 图片位置基于单元格坐标系，支持像素级偏移");
    println!("- 支持完整的CRUD操作：创建、读取、更新、删除");

    println!("\n📍 位置设置说明:");
    println!("- start_col_index: 起始列索引（从0开始）");
    println!("- start_row_index: 起始行索引（从0开始）");
    println!("- offset_x: 在单元格内的水平偏移量（像素）");
    println!("- offset_y: 在单元格内的垂直偏移量（像素）");

    println!("\n📏 大小设置说明:");
    println!("- width: 图片显示宽度（像素）");
    println!("- height: 图片显示高度（像素）");
    println!("- 可以与原始图片尺寸不同，系统会自动缩放");

    println!("\n🖼️  图片 Token 说明:");
    println!("- 需要先通过文件上传接口获取 image_token");
    println!("- 支持常见图片格式：PNG、JPG、GIF等");
    println!("- 图片文件大小通常有限制，具体请参考API文档");

    Ok(())
}
