use dotenvy::dotenv;
use open_lark::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    
    let app_id = std::env::var("APP_ID").expect("APP_ID not found");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET not found");

    let client = LarkClient::builder(app_id, app_secret)
        .with_enable_token_cache(true)
        .build();

    // 写入图片示例
    use open_lark::service::sheets::v3::data_operation::ImageData;

    // 创建图片数据 - 注意：图片token需要先通过文件上传接口获得
    let image1 = ImageData::new("img_v2_041b9112-02e8-4c12-b2f2-**********g")
        .with_size(200, 150); // 设置图片大小
    
    let image2 = ImageData::new("img_v2_7b2e5f8c-16a9-4d3e-a8c1-**********h")
        .with_size(300, 200);

    let req = open_lark::service::sheets::v3::data_operation::WriteImagesRequest::builder()
        .spreadsheet_token("shtcnmBA*****yGehy8") // 替换为实际的表格 token
        .range("Sheet1!A1:B2") // 写入图片的范围
        .add_image(image1) // 添加第一张图片
        .add_image(image2) // 添加第二张图片
        .build();

    match client.sheets.v3.data_operation.write_images(req, None).await {
        Ok(resp) => {
            println!("✅ 写入图片成功!");
            println!("📊 表格 Token: {}", resp.data.spreadsheet_token);
            println!("📍 更新范围: {}", resp.data.table_range);
            println!("🔄 版本号: {}", resp.data.revision);
            println!("📝 更新信息:");
            println!("  - 更新范围: {}", resp.data.updates.updated_range);
            println!("  - 更新行数: {}", resp.data.updates.updated_rows);
            println!("  - 更新列数: {}", resp.data.updates.updated_columns);
            println!("  - 更新单元格数: {}", resp.data.updates.updated_cells);
            println!("  - 写入图片数: {}", resp.data.updates.updated_images);
        }
        Err(e) => {
            eprintln!("❌ 写入图片失败: {:?}", e);
        }
    }

    // 也可以直接用images方法批量设置
    println!("\n--- 使用批量方式写入图片 ---");
    
    let image3 = ImageData::new("img_v2_9c4d7a2b-08e5-4f1c-b3d6-**********i");
    let image4 = ImageData::new("img_v2_5e8f1d3a-12b7-4a9e-c2f5-**********j");
    
    // 组织成二维数组格式 (每行包含多个图片)
    let images = vec![
        vec![image3], // 第一行第一列
        vec![image4], // 第二行第一列  
    ];

    let req2 = open_lark::service::sheets::v3::data_operation::WriteImagesRequest::builder()
        .spreadsheet_token("shtcnmBA*****yGehy8") // 替换为实际的表格 token
        .range("Sheet1!C1:C2") // 写入到C列
        .images(images)
        .build();

    match client.sheets.v3.data_operation.write_images(req2, None).await {
        Ok(resp) => {
            println!("✅ 批量写入图片成功!");
            println!("📊 写入了 {} 张图片", resp.data.updates.updated_images);
        }
        Err(e) => {
            eprintln!("❌ 批量写入图片失败: {:?}", e);
        }
    }

    // 提示：如何获取图片token
    println!("\n💡 提示：图片token获取方法:");
    println!("1. 使用飞书文件上传API先上传图片文件");
    println!("2. 从上传响应中获取image_token");
    println!("3. 使用该token调用此接口写入到表格中");
    println!("4. 图片将以实际图片形式显示在单元格中");

    Ok(())
}