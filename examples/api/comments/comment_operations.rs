use open_lark::{
    core::config::{AppType, Config},
    service::comments::{
        BatchQueryCommentsRequest, ContentBuilder, CreateCommentRequest, DeleteReplyRequest,
        GetCommentRequest, ListCommentsRequest, ListRepliesRequest, PatchCommentRequest,
        UpdateReplyRequest,
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
    let client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .with_enable_token_cache(true)
        .build();

    // 配置必要参数
    let file_token = "doccnxxxxxx"; // 需要一个真实的文档token
    let file_type = "doc"; // 文档类型

    // 1. 获取云文档所有评论
    println!("1. 获取云文档所有评论...");

    let list_request = ListCommentsRequest::builder()
        .file_token(file_token)
        .as_doc()
        .all_comment_types()
        .all_comments()
        .page_size(20)
        .with_open_id()
        .build();

    let mut comment_id = String::new();

    match client.comments.list(list_request, None).await {
        Ok(list_response) => {
            println!("评论列表获取成功:");
            println!("  - 评论数量: {}", list_response.items.len());
            println!("  - 是否有更多: {}", list_response.has_more);

            for (index, comment) in list_response.items.iter().enumerate() {
                println!("\n  评论 {}:", index + 1);
                println!("    - ID: {}", comment.comment_id);
                println!("    - 用户: {}", comment.user_id);
                println!("    - 状态: {}", if comment.is_solved { "已解决" } else { "未解决" });
                println!("    - 是否全文: {}", comment.is_whole.unwrap_or(false));
                println!("    - 回复数: {}", comment.reply_count());
                println!("    - 创建时间: {}", comment.create_time);

                if let Some(quote) = &comment.quote {
                    println!("    - 引用: {}", quote);
                }

                if comment.has_replies() {
                    println!("    - 有回复");
                }

                // 保存第一个评论ID用于后续操作
                if index == 0 {
                    comment_id = comment.comment_id.clone();
                }
            }
        }
        Err(e) => {
            println!("获取评论列表失败: {:?}", e);
            println!("可能的原因：");
            println!("- 文档token不存在或无效");
            println!("- 没有访问该文档的权限");
            println!("- 文档类型错误");
        }
    }

    // 2. 添加全文评论
    println!("\n2. 添加全文评论...");

    let create_request = CreateCommentRequest::builder()
        .file_token(file_token)
        .as_doc()
        .text("这是一条测试评论")
        .bold_text("重要提醒：")
        .text("请及时查看此文档")
        .with_open_id()
        .build();

    match client.comments.create(create_request, None).await {
        Ok(create_response) => {
            let comment = &create_response.comment;
            comment_id = comment.comment_id.clone(); // 更新comment_id
            println!("评论创建成功:");
            println!("  - 评论ID: {}", comment.comment_id);
            println!("  - 用户ID: {}", comment.user_id);
            println!("  - 创建时间: {}", comment.create_time);
            println!("  - 是否全文: {}", comment.is_whole_comment());
        }
        Err(e) => {
            println!("创建评论失败: {:?}", e);
        }
    }

    // 3. 获取单个评论详情
    if !comment_id.is_empty() {
        println!("\n3. 获取单个评论详情...");

        let get_request = GetCommentRequest::builder()
            .file_token(file_token)
            .as_doc()
            .comment_id(&comment_id)
            .with_open_id()
            .build();

        match client.comments.get(get_request, None).await {
            Ok(get_response) => {
                println!("评论详情获取成功:");
                println!("  {}", get_response.summary());
                if let Some(quote) = get_response.quote() {
                    println!("  - 引用内容: {}", quote);
                }
            }
            Err(e) => {
                println!("获取评论详情失败: {:?}", e);
            }
        }
    }

    // 4. 批量获取评论
    if !comment_id.is_empty() {
        println!("\n4. 批量获取评论...");

        let batch_request = BatchQueryCommentsRequest::builder()
            .file_token(file_token)
            .as_doc()
            .add_comment_id(&comment_id)
            .with_open_id()
            .build();

        match client.comments.batch_query(batch_request, None).await {
            Ok(batch_response) => {
                println!("批量获取评论成功:");
                println!("  - 获取到 {} 个评论", batch_response.count());
                println!("  - 已解决: {}", batch_response.solved_comments().len());
                println!("  - 未解决: {}", batch_response.unsolved_comments().len());
                println!("  - 全文评论: {}", batch_response.whole_comments().len());
            }
            Err(e) => {
                println!("批量获取评论失败: {:?}", e);
            }
        }
    }

    // 5. 解决评论
    if !comment_id.is_empty() {
        println!("\n5. 解决评论...");

        let solve_request = PatchCommentRequest::builder()
            .file_token(file_token)
            .as_doc()
            .comment_id(&comment_id)
            .solve_comment()
            .with_open_id()
            .build();

        match client.comments.patch(solve_request, None).await {
            Ok(patch_response) => {
                println!("评论解决成功:");
                println!("  - 评论ID: {}", patch_response.comment_id);
                println!("  - 状态: {}", if patch_response.is_solved() { "已解决" } else { "未解决" });
                if let Some(time) = patch_response.solved_time {
                    println!("  - 解决时间: {}", time);
                }
                if let Some(solver) = &patch_response.solver_user_id {
                    println!("  - 解决者: {}", solver);
                }
            }
            Err(e) => {
                println!("解决评论失败: {:?}", e);
            }
        }
    }

    // 6. 恢复评论
    if !comment_id.is_empty() {
        println!("\n6. 恢复评论...");

        let restore_request = PatchCommentRequest::builder()
            .file_token(file_token)
            .as_doc()
            .comment_id(&comment_id)
            .restore_comment()
            .with_open_id()
            .build();

        match client.comments.patch(restore_request, None).await {
            Ok(patch_response) => {
                println!("评论恢复成功:");
                println!("  - 评论ID: {}", patch_response.comment_id);
                println!("  - 状态: {}", if patch_response.is_restored() { "已恢复" } else { "已解决" });
            }
            Err(e) => {
                println!("恢复评论失败: {:?}", e);
            }
        }
    }

    // 7. 获取回复信息
    if !comment_id.is_empty() {
        println!("\n7. 获取回复信息...");

        let list_replies_request = ListRepliesRequest::builder()
            .file_token(file_token)
            .as_doc()
            .comment_id(&comment_id)
            .page_size(10)
            .with_open_id()
            .build();

        match client.comments.list_replies(list_replies_request, None).await {
            Ok(replies_response) => {
                println!("回复列表获取成功:");
                println!("  {}", replies_response.summary());

                if !replies_response.is_empty() {
                    let sorted_replies = replies_response.sorted_by_time();
                    for (index, reply) in sorted_replies.iter().enumerate() {
                        println!("  回复 {}:", index + 1);
                        println!("    - ID: {}", reply.reply_id);
                        println!("    - 用户: {}", reply.user_id);
                        println!("    - 内容: {}", reply.get_text_content());
                        println!("    - 时间: {}", reply.create_time);
                    }
                }
            }
            Err(e) => {
                println!("获取回复信息失败: {:?}", e);
            }
        }
    }

    // 8. 创建复杂内容的评论示例
    println!("\n8. 创建复杂内容的评论...");

    let complex_content = ContentBuilder::new()
        .add_text("普通文本，")
        .add_bold("粗体文本，")
        .add_italic("斜体文本，")
        .add_underline("下划线文本")
        .build();

    let complex_request = CreateCommentRequest::builder()
        .file_token(file_token)
        .as_doc()
        .content(complex_content)
        .with_open_id()
        .build();

    match client.comments.create(complex_request, None).await {
        Ok(create_response) => {
            println!("复杂评论创建成功:");
            println!("  - 评论ID: {}", create_response.comment.comment_id);
            println!("  - 包含多种格式的文本");
        }
        Err(e) => {
            println!("创建复杂评论失败: {:?}", e);
        }
    }

    // 9. 不同文档类型的评论操作示例
    println!("\n9. 不同文档类型的评论操作示例...");

    let doc_types = vec![
        ("doc", "文档"),
        ("docx", "新版文档"),
        ("sheet", "电子表格"),
        ("bitable", "多维表格"),
    ];

    for (file_type, type_name) in doc_types {
        let type_request = ListCommentsRequest::builder()
            .file_token(file_token)
            .file_type(file_type)
            .page_size(5)
            .all_comments()
            .with_open_id()
            .build();

        match client.comments.list(type_request, None).await {
            Ok(response) => {
                println!("  {} ({}) 评论数量: {}", type_name, file_type, response.items.len());
            }
            Err(e) => {
                println!("  {} ({}) 获取失败: {:?}", type_name, file_type, e);
            }
        }
    }

    // 10. 筛选评论示例
    println!("\n10. 筛选评论示例...");

    // 只获取全文评论
    let whole_comments_request = ListCommentsRequest::builder()
        .file_token(file_token)
        .as_doc()
        .whole_comments_only()
        .all_comments()
        .with_open_id()
        .build();

    if let Ok(response) = client.comments.list(whole_comments_request, None).await {
        println!("  - 全文评论数量: {}", response.items.len());
    }

    // 只获取已解决的评论
    let solved_comments_request = ListCommentsRequest::builder()
        .file_token(file_token)
        .as_doc()
        .all_comment_types()
        .solved_comments_only()
        .with_open_id()
        .build();

    if let Ok(response) = client.comments.list(solved_comments_request, None).await {
        println!("  - 已解决评论数量: {}", response.items.len());
    }

    // 只获取未解决的评论
    let unsolved_comments_request = ListCommentsRequest::builder()
        .file_token(file_token)
        .as_doc()
        .all_comment_types()
        .unsolved_comments_only()
        .with_open_id()
        .build();

    if let Ok(response) = client.comments.list(unsolved_comments_request, None).await {
        println!("  - 未解决评论数量: {}", response.items.len());
    }

    println!("\n评论操作示例完成！");
    println!("提示：");
    println!("- 需要对文档有评论权限才能添加和管理评论");
    println!("- 支持文档、新版文档、电子表格、多维表格等类型");
    println!("- 评论支持富文本格式，包括粗体、斜体、下划线等");
    println!("- 可以按状态、类型等条件筛选评论");
    println!("- 评论的解决/恢复状态可以动态管理");
    println!("- 回复支持完整的CRUD操作");

    Ok(())
}