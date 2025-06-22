use open_lark::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let app_id = std::env::var("APP_ID").unwrap();
    let app_secret = std::env::var("APP_SECRET").unwrap();

    let client = LarkClient::builder(app_id, app_secret)
        .with_enable_token_cache(true)
        .build();

    // 获取画板所有节点
    let request = ListWhiteboardNodesRequest::builder()
        .whiteboard_token("wbdcnxxxxxx")
        .medium_page()
        .build();

    match client.board.list_nodes(&request, None).await {
        Ok(response) => {
            println!("获取画板节点成功");

            let nodes_data = &response.data;
            println!("节点统计: {}", nodes_data.statistics_summary());
            println!("分页信息: {}", nodes_data.pagination_info());

            if nodes_data.is_empty() {
                println!("画板中没有节点");
                return Ok(());
            }

            // 按类型统计节点
            println!("\n--- 节点类型分析 ---");
            let type_counts = nodes_data.count_by_type();
            for (node_type, count) in type_counts {
                println!(
                    "- {}: {}个 ({})",
                    node_type.description(),
                    count,
                    node_type.category()
                );
            }

            // 显示前几个节点的详细信息
            println!("\n--- 节点详情 (前5个) ---");
            for (i, node) in nodes_data.items.iter().take(5).enumerate() {
                println!("{}. {}", i + 1, node.node_summary());

                let status = node.node_status();
                for stat in status {
                    println!("   {}", stat);
                }

                if let Some(create_time) = node.create_time_formatted() {
                    println!("   创建时间: {}", create_time);
                }

                if let Some(update_time) = node.update_time_formatted() {
                    println!("   更新时间: {}", update_time);
                }

                println!("   复杂度: {} 分", node.complexity_score());
                println!();
            }

            // 特殊节点分析
            println!("--- 特殊节点分析 ---");

            let content_nodes = nodes_data.content_nodes();
            if !content_nodes.is_empty() {
                println!("有内容的节点: {}个", content_nodes.len());
                for node in content_nodes.iter().take(3) {
                    if let Some(ref content) = node.content {
                        println!(
                            "- {} ({}): {}",
                            node.node_type.description(),
                            node.node_id,
                            content.content_summary()
                        );
                    }
                }
            }

            let locked_nodes = nodes_data.locked_nodes();
            if !locked_nodes.is_empty() {
                println!("\n锁定的节点: {}个", locked_nodes.len());
                for node in locked_nodes.iter().take(3) {
                    println!("- {}: {}", node.node_type.description(), node.node_id);
                }
            }

            let hidden_nodes = nodes_data.hidden_nodes();
            if !hidden_nodes.is_empty() {
                println!("\n隐藏的节点: {}个", hidden_nodes.len());
                for node in hidden_nodes.iter().take(3) {
                    println!("- {}: {}", node.node_type.description(), node.node_id);
                }
            }

            // 复杂度分析
            println!("\n--- 复杂度分析 ---");
            let complex_nodes = nodes_data.nodes_by_complexity();
            if !complex_nodes.is_empty() {
                println!("最复杂的3个节点:");
                for (i, node) in complex_nodes.iter().take(3).enumerate() {
                    println!(
                        "{}. {} (复杂度: {} 分)",
                        i + 1,
                        node.node_summary(),
                        node.complexity_score()
                    );
                }
            }

            // 绘图元素分析
            println!("\n--- 绘图元素分析 ---");
            let drawing_nodes: Vec<_> = nodes_data
                .items
                .iter()
                .filter(|node| node.node_type.is_drawing_type())
                .collect();

            if !drawing_nodes.is_empty() {
                println!("绘图元素: {}个", drawing_nodes.len());

                // 计算绘图区域
                if drawing_nodes.len() > 1 {
                    let mut total_area = 0.0;
                    for node in &drawing_nodes {
                        total_area += node.position.area();
                    }
                    println!("总绘图面积: {:.0} 平方像素", total_area);
                }

                // 显示绘图元素详情
                for node in drawing_nodes.iter().take(3) {
                    println!(
                        "- {}: {} (面积: {:.0})",
                        node.node_type.description(),
                        node.position.bounds_description(),
                        node.position.area()
                    );
                }
            }

            // 内容元素分析
            println!("\n--- 内容元素分析 ---");
            let content_type_nodes: Vec<_> = nodes_data
                .items
                .iter()
                .filter(|node| node.node_type.is_content_type())
                .collect();

            if !content_type_nodes.is_empty() {
                println!("内容元素: {}个", content_type_nodes.len());

                let mut text_nodes = 0;
                let mut image_nodes = 0;
                let mut total_text_length = 0;

                for node in &content_type_nodes {
                    if let Some(ref content) = node.content {
                        if content.has_text() {
                            text_nodes += 1;
                            total_text_length += content.text_length();
                        }
                        if content.has_image() {
                            image_nodes += 1;
                        }
                    }
                }

                if text_nodes > 0 {
                    println!(
                        "- 文本节点: {}个，总字符数: {}",
                        text_nodes, total_text_length
                    );
                }
                if image_nodes > 0 {
                    println!("- 图片节点: {}个", image_nodes);
                }
            }

            // 容器元素分析
            let container_nodes: Vec<_> = nodes_data
                .items
                .iter()
                .filter(|node| node.node_type.is_container_type())
                .collect();

            if !container_nodes.is_empty() {
                println!("\n--- 容器元素分析 ---");
                println!("容器元素: {}个", container_nodes.len());

                for node in container_nodes.iter().take(3) {
                    println!(
                        "- {}: {} (层级: {})",
                        node.node_type.description(),
                        node.position.bounds_description(),
                        node.z_index.unwrap_or(0)
                    );
                }
            }

            // 如果还有更多页面
            if nodes_data.has_more {
                println!("\n⏩ 还有更多节点可以获取");
                if let Some(ref next_token) = nodes_data.page_token {
                    println!("下一页token: {}", next_token);

                    // 演示获取下一页
                    println!("\n--- 获取下一页节点 ---");
                    let next_request = ListWhiteboardNodesRequest::builder()
                        .whiteboard_token("wbdcnxxxxxx")
                        .medium_page()
                        .page_token(next_token)
                        .build();

                    match client.board.list_nodes(&next_request, None).await {
                        Ok(next_response) => {
                            println!("下一页节点: {}个", next_response.data.node_count());
                            println!("分页信息: {}", next_response.data.pagination_info());
                        }
                        Err(e) => {
                            eprintln!("获取下一页失败: {:?}", e);
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("获取画板节点失败: {:?}", e);
        }
    }

    // 不同分页大小的示例
    println!("\n--- 不同分页大小示例 ---");

    // 小分页示例
    let small_request = ListWhiteboardNodesRequest::builder()
        .whiteboard_token("wbdcnxxxxxx")
        .small_page()
        .build();

    match client.board.list_nodes(&small_request, None).await {
        Ok(response) => {
            println!("小分页 (20个): {}", response.data.statistics_summary());
        }
        Err(e) => {
            eprintln!("小分页查询失败: {:?}", e);
        }
    }

    // 大分页示例
    let large_request = ListWhiteboardNodesRequest::builder()
        .whiteboard_token("wbdcnxxxxxx")
        .large_page()
        .build();

    match client.board.list_nodes(&large_request, None).await {
        Ok(response) => {
            println!("大分页 (100个): {}", response.data.statistics_summary());
        }
        Err(e) => {
            eprintln!("大分页查询失败: {:?}", e);
        }
    }

    Ok(())
}
