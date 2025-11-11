//! Sheets v2 图片写入示例
//!
//! 本示例展示如何使用飞书开放平台SDK v2版本的电子表格API来写入图片到电子表格。
//! 支持图片定位、尺寸调整、批量写入等多种企业级功能。

use open_lark::core::config::Config;
use open_lark::prelude::*;
use open_lark::service::sheets::v2::{
    ImagePosition, ImageSize, ImageWriteRequest, ImageWriteService,
};

#[tokio::main]
async fn main() -> SDKResult<()> {
    // 初始化配置和服务
    let config = Config::builder()
        .app_id("your_app_id")
        .app_secret("your_app_secret")
        .build();

    let image_service = ImageWriteService::new(config);

    // 示例1: 基础图片写入
    println!("=== 示例1: 基础图片写入 ===");
    let basic_request = ImageWriteRequest::simple(
        "shtcnmBA*****yGehy8",          // 电子表格令牌
        "Sheet1",                       // 工作表ID
        "https://example.com/logo.png", // 图片URL
        "A1",                           // 锚点单元格
    )
    .alt_text("公司Logo"); // 替代文本

    match image_service.write_image(basic_request, None).await {
        Ok(response) => {
            println!("✅ 基础图片写入成功!");
            if let Some(data) = &response.data {
                println!("图片ID: {}", data.image_id);
                println!("位置: {}", data.position.anchor_cell);
                println!(
                    "尺寸: {}x{}",
                    data.actual_size.width, data.actual_size.height
                );
                println!("图片URL: {}", data.image_url);
            }
        }
        Err(error) => {
            println!("❌ 基础图片写入失败: {}", error.user_friendly_message());
        }
    }

    // 示例2: 使用Builder模式进行精确定位
    println!("\n=== 示例2: 使用Builder模式进行精确定位 ===");
    let builder_request = ImageWriteRequest::builder()
        .spreadsheet_token("shtcnmBA*****yGehy8")
        .sheet_id("Sheet1")
        .image_url("https://example.com/product-image.jpg")
        .at_cell("C3", 15, 25) // 锚点C3，偏移(15, 25)
        .with_dimensions(400, 300, true) // 宽400，高300，保持宽高比
        .alt_text("产品图片")
        .user_id_type("open_id")
        .z_index(5) // Z层级为5
        .build()
        .expect("有效的图片写入请求");

    match image_service.write_image(builder_request, None).await {
        Ok(response) => {
            println!("✅ Builder模式图片写入成功!");
            if let Some(data) = &response.data {
                println!("图片ID: {}", data.image_id);
                let (x, y) = data.position.calculate_absolute_position();
                println!("绝对位置: ({}, {})", x, y);
                println!("Z层级: {:?}", data.position.z_index);
            }
        }
        Err(error) => {
            println!(
                "❌ Builder模式图片写入失败: {}",
                error.user_friendly_message()
            );
        }
    }

    // 示例3: 批量写入多张图片
    println!("\n=== 示例3: 批量写入多张图片 ===");
    let batch_requests = vec![
        ImageWriteRequest::with_size(
            "shtcnmBA*****yGehy8",
            "Sheet1",
            "https://example.com/avatar1.png",
            "A1",
            100,
            100,
        )
        .alt_text("用户头像1"),
        ImageWriteRequest::with_offset(
            "shtcnmBA*****yGehy8",
            "Sheet1",
            "https://example.com/avatar2.png",
            "B1",
            10,
            5,
        )
        .alt_text("用户头像2"),
        ImageWriteRequest::new(
            "shtcnmBA*****yGehy8",
            "Sheet1",
            "https://example.com/avatar3.png",
            ImagePosition::new("C1", 20, 10).z_index(2),
            ImageSize::new(100, 100, true),
        )
        .alt_text("用户头像3"),
    ];

    match image_service
        .write_images_batch("shtcnmBA*****yGehy8", batch_requests, None)
        .await
    {
        Ok(responses) => {
            println!("✅ 批量图片写入完成!");
            for (index, response) in responses.iter().enumerate() {
                match response {
                    Ok(resp) => {
                        if let Some(data) = &resp.data {
                            println!(
                                "  图片{}: ID={}, 位置={}",
                                index + 1,
                                data.image_id,
                                data.position.anchor_cell
                            );
                        }
                    }
                    Err(error) => {
                        println!(
                            "  图片{} 写入失败: {}",
                            index + 1,
                            error.user_friendly_message()
                        );
                    }
                }
            }
        }
        Err(error) => {
            println!("❌ 批量图片写入失败: {}", error.user_friendly_message());
        }
    }

    // 示例4: 复杂的图片布局设计
    println!("\n=== 示例4: 复杂的图片布局设计 ===");

    // 创建一个仪表板布局
    let dashboard_requests = vec![
        // 主标题图片
        ImageWriteRequest::builder()
            .spreadsheet_token("shtcnmBA*****yGehy8")
            .sheet_id("Dashboard")
            .image_url("https://example.com/title-banner.png")
            .at_cell("A1", 0, 0)
            .with_dimensions(800, 120, false)
            .alt_text("仪表板标题")
            .z_index(10)
            .build()
            .expect("有效的标题图片请求"),
        // KPI指标图片
        ImageWriteRequest::builder()
            .spreadsheet_token("shtcnmBA*****yGehy8")
            .sheet_id("Dashboard")
            .image_url("https://example.com/kpi-chart1.png")
            .at_cell("A3", 0, 0)
            .with_dimensions(250, 200, true)
            .alt_text("销售图表")
            .z_index(5)
            .build()
            .expect("有效的KPI图片请求1"),
        ImageWriteRequest::builder()
            .spreadsheet_token("shtcnmBA*****yGehy8")
            .sheet_id("Dashboard")
            .image_url("https://example.com/kpi-chart2.png")
            .at_cell("D3", 0, 0)
            .with_dimensions(250, 200, true)
            .alt_text("用户增长图表")
            .z_index(5)
            .build()
            .expect("有效的KPI图片请求2"),
        ImageWriteRequest::builder()
            .spreadsheet_token("shtcnmBA*****yGehy8")
            .sheet_id("Dashboard")
            .image_url("https://example.com/kpi-chart3.png")
            .at_cell("G3", 0, 0)
            .with_dimensions(250, 200, true)
            .alt_text("收入图表")
            .z_index(5)
            .build()
            .expect("有效的KPI图片请求3"),
    ];

    match image_service
        .write_images_batch("shtcnmBA*****yGehy8", dashboard_requests, None)
        .await
    {
        Ok(responses) => {
            println!("✅ 仪表板布局创建完成!");
            let success_count = responses.iter().filter(|r| r.is_ok()).count();
            println!("成功写入: {}/{} 张图片", success_count, responses.len());
        }
        Err(error) => {
            println!("❌ 仪表板布局创建失败: {}", error.user_friendly_message());
        }
    }

    // 示例5: 动态图片尺寸调整
    println!("\n=== 示例5: 动态图片尺寸调整 ===");

    // 模拟根据容器大小动态调整图片尺寸
    let container_width = 600;
    let container_height = 400;
    let image_url = "https://example.com/dynamic-image.png";

    // 计算适合容器的图片尺寸（保持宽高比）
    let image_width = container_width;
    let image_height = (container_width as f64 * 0.75) as u32; // 假设原始宽高比为4:3

    let dynamic_request = ImageWriteRequest::builder()
        .spreadsheet_token("shtcnmBA*****yGehy8")
        .sheet_id("Sheet1")
        .image_url(image_url)
        .at_cell("F5", 0, 0)
        .with_dimensions(image_width, image_height, true)
        .alt_text("动态调整尺寸的图片")
        .user_id_type("open_id")
        .build()
        .expect("有效的动态尺寸请求");

    match image_service.write_image(dynamic_request, None).await {
        Ok(response) => {
            println!("✅ 动态尺寸图片写入成功!");
            if let Some(data) = &response.data {
                println!("容器尺寸: {}x{}", container_width, container_height);
                println!(
                    "图片实际尺寸: {}x{}",
                    data.actual_size.width, data.actual_size.height
                );
                println!("宽高比保持: {}", data.size.maintain_aspect_ratio);
            }
        }
        Err(error) => {
            println!("❌ 动态尺寸图片写入失败: {}", error.user_friendly_message());
        }
    }

    // 示例6: 图片管理和更新
    println!("\n=== 示例6: 图片管理和更新 ===");

    // 首先写入一张图片
    let initial_request = ImageWriteRequest::simple(
        "shtcnmBA*****yGehy8",
        "Sheet1",
        "https://example.com/temporary-logo.png",
        "H1",
    )
    .alt_text("临时Logo");

    let mut image_id = String::new();
    match image_service.write_image(initial_request, None).await {
        Ok(response) => {
            if let Some(data) = &response.data {
                image_id = data.image_id.clone();
                println!("✅ 初始图片写入成功，图片ID: {}", image_id);

                // 获取图片信息
                match image_service
                    .get_image_info("shtcnmBA*****yGehy8", &image_id, None)
                    .await
                {
                    Ok(info_response) => {
                        if let Some(info) = &info_response.data {
                            println!("图片信息获取成功:");
                            println!("  位置: {}", info.position.anchor_cell);
                            println!(
                                "  尺寸: {}x{}",
                                info.actual_size.width, info.actual_size.height
                            );
                            println!("  URL: {}", info.image_url);
                        }
                    }
                    Err(error) => {
                        println!("❌ 获取图片信息失败: {}", error.user_friendly_message());
                    }
                }

                // 更新图片位置
                let new_position = ImagePosition::new("H10", 50, 30).z_index(8);
                let new_size = Some(ImageSize::new(150, 100, true));

                match image_service
                    .update_image(
                        "shtcnmBA*****yGehy8",
                        &image_id,
                        new_position,
                        new_size,
                        None,
                    )
                    .await
                {
                    Ok(update_response) => {
                        println!("✅ 图片更新成功!");
                        if let Some(data) = &update_response.data {
                            println!("新位置: {}", data.position.anchor_cell);
                            println!(
                                "新尺寸: {}x{}",
                                data.actual_size.width, data.actual_size.height
                            );
                        }
                    }
                    Err(error) => {
                        println!("❌ 图片更新失败: {}", error.user_friendly_message());
                    }
                }

                // 删除图片
                match image_service
                    .delete_image("shtcnmBA*****yGehy8", &image_id, None)
                    .await
                {
                    Ok(_) => {
                        println!("✅ 图片删除成功!");
                    }
                    Err(error) => {
                        println!("❌ 图片删除失败: {}", error.user_friendly_message());
                    }
                }
            }
        }
        Err(error) => {
            println!("❌ 初始图片写入失败: {}", error.user_friendly_message());
        }
    }

    // 示例7: 错误处理演示
    println!("\n=== 示例7: 错误处理演示 ===");

    // 测试无效的电子表格令牌
    let invalid_token_request = ImageWriteRequest::simple(
        "", // 空的令牌
        "Sheet1",
        "https://example.com/logo.png",
        "A1",
    );

    match image_service.write_image(invalid_token_request, None).await {
        Ok(_) => {
            println!("意外成功，应该失败");
        }
        Err(error) => {
            println!("✅ 正确捕获无效令牌错误: {}", error.user_friendly_message());
        }
    }

    // 测试无效的图片URL
    let invalid_url_request = ImageWriteRequest::builder()
        .spreadsheet_token("shtcnmBA*****yGehy8")
        .sheet_id("Sheet1")
        .image_url("not-a-valid-url") // 无效的URL格式
        .at_cell("A1", 0, 0)
        .with_dimensions(200, 150, true)
        .build();

    match invalid_url_request {
        Ok(request) => match image_service.write_image(request, None).await {
            Ok(_) => {
                println!("意外成功，应该失败");
            }
            Err(error) => {
                println!("✅ 正确捕获无效URL错误: {}", error.user_friendly_message());
            }
        },
        Err(error) => {
            println!(
                "✅ 正确在构建阶段捕获URL错误: {}",
                error.user_friendly_message()
            );
        }
    }

    // 测试无效的单元格引用
    let invalid_cell_request = ImageWriteRequest::builder()
        .spreadsheet_token("shtcnmBA*****yGehy8")
        .sheet_id("Sheet1")
        .image_url("https://example.com/logo.png")
        .at_cell("InvalidCell", 0, 0) // 无效的单元格引用
        .with_dimensions(200, 150, true)
        .build();

    match invalid_cell_request {
        Ok(request) => match image_service.write_image(request, None).await {
            Ok(_) => {
                println!("意外成功，应该失败");
            }
            Err(error) => {
                println!(
                    "✅ 正确捕获无效单元格错误: {}",
                    error.user_friendly_message()
                );
            }
        },
        Err(error) => {
            println!(
                "✅ 正确在构建阶段捕获单元格错误: {}",
                error.user_friendly_message()
            );
        }
    }

    // 示例8: 性能测试和大规模操作
    println!("\n=== 示例8: 性能测试和大规模操作 ===");

    // 创建大量图片写入请求进行性能测试
    let mut large_batch_requests = Vec::new();
    let base_url = "https://example.com/test-image-";

    for i in 1..=20 {
        let row = ((i - 1) / 5) + 1; // 每5个图片一行
        let col = ((i - 1) % 5) + 1; // 列从1到5
        let cell = format!("{}{}", char::('A' as u8 + col as u8 - 1), row + 10); // 从A11开始

        let request = ImageWriteRequest::builder()
            .spreadsheet_token("shtcnmBA*****yGehy8")
            .sheet_id("PerformanceTest")
            .image_url(&format!("{}{}.png", base_url, i))
            .at_cell(&cell, 0, 0)
            .with_dimensions(80, 60, true)
            .alt_text(&format!("测试图片{}", i))
            .z_index(i)
            .build()
            .expect("有效的批量请求");

        large_batch_requests.push(request);
    }

    println!("准备批量写入 {} 张图片", large_batch_requests.len());
    let start_time = std::time::Instant::now();

    match image_service
        .write_images_batch("shtcnmBA*****yGehy8", large_batch_requests, None)
        .await
    {
        Ok(responses) => {
            let duration = start_time.elapsed();
            let success_count = responses.iter().filter(|r| r.is_ok()).count();

            println!("✅ 大规模批量写入完成!");
            println!("成功: {}/{} 张图片", success_count, responses.len());
            println!("耗时: {:?}", duration);
            println!("平均每张图片: {:?}", duration / responses.len() as u32);
        }
        Err(error) => {
            println!("❌ 大规模批量写入失败: {}", error.user_friendly_message());
        }
    }

    println!("\n=== 所有示例执行完成 ===");
    println!("注意：当前实现返回模拟数据，实际使用时需要配置有效的访问令牌");
    println!("功能特性总结:");
    println!("✅ 基础图片写入");
    println!("✅ Builder模式API");
    println!("✅ 批量图片写入");
    println!("✅ 精确定位和尺寸调整");
    println!("✅ Z层级管理");
    println!("✅ 图片更新和删除");
    println!("✅ 全面的错误处理");
    println!("✅ 高性能批量操作");
    println!("✅ 可访问性支持(alt_text)");

    Ok(())
}
