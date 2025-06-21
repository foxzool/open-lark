use open_lark::{
    core::config::{AppType, Config},
    service::wiki::v2::SearchWikiRequest,
    LarkClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenvy::dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("APP_ID environment variable not set");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET environment variable not set");

    // 创建客户端
    let client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .with_enable_token_cache(true)
        .build();

    // 1. 基本搜索 - 搜索所有有权限的知识空间
    println!("1. 基本搜索 - 搜索所有有权限的知识空间...");

    let basic_search_request = SearchWikiRequest::builder()
        .query("产品设计")
        .page_size(10)
        .search_all_spaces()
        .build();

    match client.wiki.v2.search_wiki(basic_search_request, None).await {
        Ok(search_response) => {
            println!("搜索完成:");
            println!("  - 找到 {} 个结果", search_response.items.len());
            println!("  - 是否有更多: {}", search_response.has_more);

            if let Some(page_token) = &search_response.page_token {
                println!("  - 下页标记: {}", page_token);
            }

            for (index, item) in search_response.items.iter().enumerate() {
                println!("\n  结果 {}:", index + 1);
                println!("    - 节点Token: {}", item.node_token);
                println!("    - 标题: {}", item.display_title());
                if let Some(space_name) = &item.space_name {
                    println!("    - 知识空间: {}", space_name);
                }
                if let Some(obj_type) = &item.obj_type {
                    println!("    - 文档类型: {}", obj_type);
                }
                if let Some(snippet) = &item.snippet {
                    println!("    - 匹配片段: {}", snippet);
                }
                if let Some(edit_time) = &item.obj_edit_time {
                    println!("    - 最后编辑: {}", edit_time);
                }
                if let Some(url) = item.get_doc_url() {
                    println!("    - 文档链接: {}", url);
                }
            }
        }
        Err(e) => {
            println!("基本搜索失败: {:?}", e);
            println!("可能的原因：");
            println!("- 没有访问任何知识空间的权限");
            println!("- 搜索关键词为空或无效");
            println!("- 网络连接问题");
        }
    }

    // 2. 指定知识空间搜索
    println!("\n2. 指定知识空间搜索...");

    let space_ids = vec!["spcxxxxxx".to_string(), "spcyyyyyy".to_string()]; // 需要真实的知识空间ID

    let specific_search_request = SearchWikiRequest::builder()
        .query("API文档")
        .page_size(5)
        .space_ids(space_ids.clone())
        .build();

    match client
        .wiki
        .v2
        .search_wiki(specific_search_request, None)
        .await
    {
        Ok(search_response) => {
            println!("指定空间搜索完成:");
            println!("  - 搜索空间: {:?}", space_ids);
            println!("  - 找到 {} 个结果", search_response.items.len());

            for item in &search_response.items {
                println!(
                    "  - {}: {} ({})",
                    item.display_title(),
                    item.space_name.as_deref().unwrap_or("未知空间"),
                    item.obj_type.as_deref().unwrap_or("未知类型")
                );
            }
        }
        Err(e) => {
            println!("指定空间搜索失败: {:?}", e);
        }
    }

    // 3. 逐步添加搜索空间
    println!("\n3. 逐步添加搜索空间...");

    let incremental_request = SearchWikiRequest::builder()
        .query("会议纪要")
        .add_space_id("spcspace001")
        .add_space_id("spcspace002")
        .add_space_id("spcspace003")
        .page_size(15)
        .build();

    match client.wiki.v2.search_wiki(incremental_request, None).await {
        Ok(search_response) => {
            println!("增量添加空间搜索完成:");
            println!("  - 找到 {} 个结果", search_response.items.len());

            // 按空间分组显示结果
            let mut space_groups: std::collections::HashMap<String, Vec<&_>> =
                std::collections::HashMap::new();
            for item in &search_response.items {
                space_groups
                    .entry(item.space_id.clone())
                    .or_default()
                    .push(item);
            }

            for (space_id, items) in space_groups {
                let space_name = items
                    .first()
                    .and_then(|item| item.space_name.as_deref())
                    .unwrap_or("未知空间");

                println!("\n  空间 {} ({}):", space_name, space_id);
                for item in items {
                    println!("    - {}", item.display_title());
                    if item.has_snippet() {
                        if let Some(snippet) = &item.snippet {
                            println!("      片段: {}", snippet);
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("增量搜索失败: {:?}", e);
        }
    }

    // 4. 分页搜索示例
    println!("\n4. 分页搜索示例...");

    let mut page_token: Option<String> = None;
    let mut page_number = 1;
    let max_pages = 3; // 最多获取3页

    loop {
        let mut paged_request = SearchWikiRequest::builder().query("技术文档").page_size(5);

        if let Some(token) = &page_token {
            paged_request = paged_request.page_token(token);
        }

        let request = paged_request.search_all_spaces().build();

        match client.wiki.v2.search_wiki(request, None).await {
            Ok(search_response) => {
                println!("第 {} 页结果:", page_number);
                println!("  - 本页结果数: {}", search_response.items.len());

                for (index, item) in search_response.items.iter().enumerate() {
                    println!(
                        "    {}. {} ({})",
                        (page_number - 1) * 5 + index + 1,
                        item.display_title(),
                        item.obj_type.as_deref().unwrap_or("未知类型")
                    );
                }

                if search_response.has_more && page_number < max_pages {
                    page_token = search_response.page_token;
                    page_number += 1;
                    println!("  - 继续获取下一页...\n");
                } else {
                    if !search_response.has_more {
                        println!("  - 已获取所有结果");
                    } else {
                        println!("  - 已达到最大页数限制");
                    }
                    break;
                }
            }
            Err(e) => {
                println!("分页搜索失败: {:?}", e);
                break;
            }
        }
    }

    // 5. 搜索不同类型的文档
    println!("\n5. 搜索不同类型的文档...");

    let search_queries = vec![
        ("文档", "doc"),
        ("表格", "sheet"),
        ("思维导图", "mindnote"),
        ("数据库", "bitable"),
    ];

    for (query, expected_type) in search_queries {
        let type_search_request = SearchWikiRequest::builder()
            .query(query)
            .page_size(3)
            .search_all_spaces()
            .build();

        match client.wiki.v2.search_wiki(type_search_request, None).await {
            Ok(search_response) => {
                println!("搜索 '{}' 的结果:", query);

                let filtered_items: Vec<_> = search_response
                    .items
                    .iter()
                    .filter(|item| item.obj_type.as_deref() == Some(expected_type))
                    .collect();

                if !filtered_items.is_empty() {
                    println!(
                        "  找到 {} 个 {} 类型的文档:",
                        filtered_items.len(),
                        expected_type
                    );
                    for item in filtered_items {
                        println!("    - {}", item.display_title());
                    }
                } else {
                    println!("  未找到 {} 类型的文档", expected_type);
                }
            }
            Err(e) => {
                println!("搜索 '{}' 失败: {:?}", query, e);
            }
        }
    }

    println!("\n知识库搜索操作示例完成！");
    println!("提示：");
    println!("- 搜索只能找到有权限访问的知识空间内容");
    println!("- 支持中文、英文等多语言搜索");
    println!("- 可以指定特定知识空间进行搜索");
    println!("- 支持分页获取大量搜索结果");
    println!("- 搜索结果包含匹配的文本片段");
    println!("- 可以根据文档类型进行过滤");

    Ok(())
}
