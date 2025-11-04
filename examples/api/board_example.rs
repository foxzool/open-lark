//! 画板模块使用示例
//!
//! 演示如何使用board模块进行画板管理操作，包括：
//! - 画板主题管理
//! - 画板缩略图获取
//! - 节点创建和管理
//! - 画板内容查询

use open_lark::prelude::*;
use open_lark_extensions::prelude::{
    CreateWhiteboardNodeRequest, DownloadWhiteboardAsImageRequest, GetWhiteboardThemeRequest,
    ListWhiteboardNodesRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    env_logger::init();

    println!("🚀 画板模块示例演示");

    // 创建客户端（这里使用测试配置）
    let client = LarkClient::builder("test_app_id", "test_app_secret")
        .with_app_type(AppType::SelfBuild)
        .build();

    println!("✅ 客户端创建成功");

    // 演示获取画板主题
    println!("\n📋 获取画板主题");
    let theme_request = GetWhiteboardThemeRequest {
        whiteboard_id: "test_whiteboard_id".to_string(),
    };

    match client.board.v1.get_whiteboard_theme(&theme_request).await {
        Ok(response) => {
            println!("✅ 画板主题获取成功");
            if let Some(data) = response.data {
                println!("   主题ID: {}", data.theme_id);
                println!("   主题名称: {}", data.theme_name);
                println!("   背景颜色: {}", data.background_color);
                println!("   网格颜色: {}", data.grid_color);
            }
        }
        Err(e) => {
            println!("❌ 画板主题获取失败: {}", e);
        }
    }

    // 演示获取画板缩略图片
    println!("\n📋 获取画板缩略图片");
    let image_request = DownloadWhiteboardAsImageRequest {
        whiteboard_id: "test_whiteboard_id".to_string(),
        width: Some(1920),
        height: Some(1080),
        quality: Some("high".to_string()),
    };

    match client
        .board
        .v1
        .download_whiteboard_as_image(&image_request)
        .await
    {
        Ok(response) => {
            println!("✅ 画板缩略图获取成功");
            if let Some(data) = response.data {
                println!("   图片URL: {}", data.image_url);
                println!("   图片尺寸: {}x{}", data.image_width, data.image_height);
                println!("   文件大小: {} bytes", data.file_size);
                println!("   MIME类型: {}", data.mime_type);
            }
        }
        Err(e) => {
            println!("❌ 画板缩略图获取失败: {}", e);
        }
    }

    // 演示创建节点
    println!("\n📋 创建节点");
    let node_request = CreateWhiteboardNodeRequest {
        whiteboard_id: "test_whiteboard_id".to_string(),
        node_type: "text".to_string(),
        content: "这是一个测试文本节点".to_string(),
        x: 200.0,
        y: 150.0,
        width: Some(300.0),
        height: Some(100.0),
        rotation: Some(0.0),
        style: Some(serde_json::json!({
            "fontSize": 16,
            "fontColor": "#000000",
            "backgroundColor": "#FFFFFF"
        })),
    };

    match client.board.v1.create_whiteboard_node(&node_request).await {
        Ok(response) => {
            println!("✅ 节点创建成功");
            if let Some(data) = response.data {
                println!("   节点ID: {}", data.node_id);
                println!("   节点类型: {}", data.node_type);
                println!("   节点内容: {}", data.content);
                println!("   位置: ({}, {})", data.x, data.y);
                println!("   尺寸: {}x{}", data.width, data.height);
            }
        }
        Err(e) => {
            println!("❌ 节点创建失败: {}", e);
        }
    }

    // 演示获取所有节点
    println!("\n📋 获取所有节点");
    let nodes_request = ListWhiteboardNodesRequest {
        whiteboard_id: "test_whiteboard_id".to_string(),
        page_size: Some(20),
        page_token: None,
        node_type: Some("text".to_string()),
    };

    match client.board.v1.list_whiteboard_nodes(&nodes_request).await {
        Ok(response) => {
            println!("✅ 节点列表获取成功");
            if let Some(data) = response.data {
                println!("   找到 {} 个节点", data.nodes.len());
                println!("   总数: {}", data.total);
                println!("   是否有更多: {}", data.has_more);

                for node in &data.nodes {
                    println!(
                        "   - {}: {} ({})",
                        node.node_id, node.content, node.node_type
                    );
                }
            }
        }
        Err(e) => {
            println!("❌ 节点列表获取失败: {}", e);
        }
    }

    // 演示创建不同类型的节点
    println!("\n📋 创建不同类型的节点");

    // 创建形状节点
    let shape_request = CreateWhiteboardNodeRequest {
        whiteboard_id: "test_whiteboard_id".to_string(),
        node_type: "shape".to_string(),
        content: "rectangle".to_string(),
        x: 100.0,
        y: 300.0,
        width: Some(200.0),
        height: Some(150.0),
        rotation: Some(45.0),
        style: Some(serde_json::json!({
            "fillColor": "#3498db",
            "borderColor": "#2980b9",
            "borderWidth": 2
        })),
    };

    match client.board.v1.create_whiteboard_node(&shape_request).await {
        Ok(response) => {
            println!("✅ 形状节点创建成功");
            if let Some(data) = response.data {
                println!("   节点ID: {}", data.node_id);
                println!("   节点类型: {}", data.node_type);
                println!("   旋转角度: {}°", data.rotation);
            }
        }
        Err(e) => {
            println!("❌ 形状节点创建失败: {}", e);
        }
    }

    // 创建图片节点
    let image_request = CreateWhiteboardNodeRequest {
        whiteboard_id: "test_whiteboard_id".to_string(),
        node_type: "image".to_string(),
        content: "https://example.com/test-image.png".to_string(),
        x: 400.0,
        y: 300.0,
        width: Some(250.0),
        height: Some(180.0),
        rotation: Some(0.0),
        style: Some(serde_json::json!({
            "borderRadius": 8,
            "shadow": true
        })),
    };

    match client.board.v1.create_whiteboard_node(&image_request).await {
        Ok(response) => {
            println!("✅ 图片节点创建成功");
            if let Some(data) = response.data {
                println!("   节点ID: {}", data.node_id);
                println!("   图片URL: {}", data.content);
                println!("   位置: ({}, {})", data.x, data.y);
            }
        }
        Err(e) => {
            println!("❌ 图片节点创建失败: {}", e);
        }
    }

    println!("\n🎉 画板模块示例演示完成！");
    Ok(())
}
