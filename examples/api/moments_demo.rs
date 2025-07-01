use dotenvy::dotenv;
use log::{error, info};
use open_lark::{
    prelude::*,
    service::moments::{
        events::{
            DefaultMomentsEventHandler, MomentsEvent, MomentsEventDispatcher, MomentsEventHandler,
        },
        models::{CommentEvent, PostEvent, PostGetRequest, PostStatisticsEvent, ReactionEvent},
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("APP_ID environment variable not set");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET environment variable not set");

    let client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .with_enable_token_cache(true)
        .build();

    println!("🎯 飞书公司圈API演示");
    println!("================================");

    // 1. 帖子管理演示
    println!("\n1. 帖子管理演示");
    println!("-------------------------------");

    let post_request = PostGetRequest {
        post_id: "example_post_id".to_string(),
        user_id_type: Some("open_id".to_string()),
    };

    match client.moments.post.get_post(post_request, None).await {
        Ok(response) => {
            println!("✅ 获取帖子信息成功");
            if let Some(data) = &response.data {
                println!("帖子详情:");
                if let Some(title) = &data.post.title {
                    println!("  标题: {title}");
                }
                if let Some(author_name) = &data.post.author_name {
                    println!("  作者: {author_name}");
                }
                if let Some(content) = &data.post.content {
                    println!(
                        "  内容预览: {}...",
                        content.chars().take(50).collect::<String>()
                    );
                }
                if let Some(statistics) = &data.post.statistics {
                    println!("  统计数据:");
                    if let Some(comment_count) = statistics.comment_count {
                        println!("    评论数: {comment_count}");
                    }
                    if let Some(like_count) = statistics.like_count {
                        println!("    点赞数: {like_count}");
                    }
                    if let Some(view_count) = statistics.view_count {
                        println!("    阅读数: {view_count}");
                    }
                }
                if let Some(media_list) = &data.post.media_list {
                    println!("  媒体附件: {} 个", media_list.len());
                }
            }
        }
        Err(err) => {
            error!("❌ 获取帖子信息失败: {err:?}");
        }
    }

    // 2. 事件处理演示
    println!("\n2. 事件处理演示");
    println!("-------------------------------");

    // 创建自定义事件处理器
    struct CustomMomentsHandler;

    impl open_lark::service::moments::events::PostEventHandler for CustomMomentsHandler {
        fn handle_post_created(&self, event: PostEvent) {
            info!("🎉 收到帖子发布事件");
            if let Some(post) = &event.post {
                if let Some(title) = &post.title {
                    info!("  新帖子标题: {title}");
                }
                if let Some(author_name) = &post.author_name {
                    info!("  发布者: {author_name}");
                }
            }
        }

        fn handle_post_deleted(&self, event: PostEvent) {
            info!("🗑️ 收到帖子删除事件");
            if let Some(post) = &event.post {
                if let Some(post_id) = &post.post_id {
                    info!("  删除的帖子ID: {post_id}");
                }
            }
        }
    }

    impl open_lark::service::moments::events::CommentEventHandler for CustomMomentsHandler {
        fn handle_comment_created(&self, event: CommentEvent) {
            info!("💬 收到评论发布事件");
            if let Some(comment) = &event.comment {
                if let Some(content) = &comment.content {
                    info!("  评论内容: {content}");
                }
                if let Some(author_name) = &comment.author_name {
                    info!("  评论者: {author_name}");
                }
            }
        }

        fn handle_comment_deleted(&self, event: CommentEvent) {
            info!("🗑️ 收到评论删除事件");
            if let Some(comment) = &event.comment {
                if let Some(comment_id) = &comment.comment_id {
                    info!("  删除的评论ID: {comment_id}");
                }
            }
        }
    }

    impl open_lark::service::moments::events::ReactionEventHandler for CustomMomentsHandler {
        fn handle_reaction_created(&self, event: ReactionEvent) {
            info!("👍 收到表情互动事件");
            if let Some(reaction) = &event.reaction {
                if let Some(reaction_type) = &reaction.reaction_type {
                    info!("  表情类型: {reaction_type}");
                }
                if let Some(user_name) = &reaction.user_name {
                    info!("  互动用户: {user_name}");
                }
            }
        }

        fn handle_reaction_deleted(&self, event: ReactionEvent) {
            info!("👎 收到取消表情互动事件");
            if let Some(reaction) = &event.reaction {
                if let Some(reaction_type) = &reaction.reaction_type {
                    info!("  取消的表情类型: {reaction_type}");
                }
            }
        }
    }

    impl open_lark::service::moments::events::PostStatisticsEventHandler for CustomMomentsHandler {
        fn handle_post_statistics_updated(&self, event: PostStatisticsEvent) {
            info!("📊 收到帖子统计数据变更事件");
            if let Some(post_id) = &event.post_id {
                info!("  帖子ID: {post_id}");
            }
            if let Some(statistics) = &event.statistics {
                if let Some(comment_count) = statistics.comment_count {
                    info!("  当前评论数: {comment_count}");
                }
                if let Some(like_count) = statistics.like_count {
                    info!("  当前点赞数: {like_count}");
                }
            }
        }
    }

    impl MomentsEventHandler for CustomMomentsHandler {}

    // 创建事件分发器
    let custom_handler = CustomMomentsHandler;
    let dispatcher = MomentsEventDispatcher::new(custom_handler);

    println!("✅ 创建自定义事件处理器成功");
    println!(
        "处理器名称: {}",
        dispatcher.get_handler().get_handler_name()
    );

    // 演示事件分发（模拟接收到的事件）
    println!("\n模拟事件分发:");

    // 模拟帖子发布事件
    let mock_post_event = PostEvent {
        event_type: Some("created".to_string()),
        post: Some(open_lark::service::moments::models::Post {
            post_id: Some("mock_post_123".to_string()),
            title: Some("这是一个测试帖子".to_string()),
            author_name: Some("张三".to_string()),
            content: Some("这是测试帖子的内容，用于演示事件处理功能。".to_string()),
            author_id: None,
            content_type: None,
            media_list: None,
            status: None,
            create_time: None,
            update_time: None,
            visibility: None,
            statistics: None,
            extra: None,
        }),
        event_time: Some("2024-01-15T10:30:00Z".to_string()),
        operator_id: Some("user_123".to_string()),
    };

    let moments_event = MomentsEvent::PostCreated {
        event: mock_post_event,
    };
    dispatcher.dispatch_event(moments_event);

    // 模拟评论事件
    let mock_comment_event = CommentEvent {
        event_type: Some("created".to_string()),
        comment: Some(open_lark::service::moments::models::Comment {
            comment_id: Some("comment_456".to_string()),
            post_id: Some("mock_post_123".to_string()),
            author_name: Some("李四".to_string()),
            content: Some("这是一个很棒的帖子！".to_string()),
            author_id: None,
            content_type: None,
            parent_comment_id: None,
            reply_to_user_id: None,
            create_time: None,
            update_time: None,
            media_list: None,
        }),
        event_time: Some("2024-01-15T10:35:00Z".to_string()),
        operator_id: Some("user_456".to_string()),
    };

    let comment_moments_event = MomentsEvent::CommentCreated {
        event: mock_comment_event,
    };
    dispatcher.dispatch_event(comment_moments_event);

    // 3. 使用默认事件处理器
    println!("\n3. 默认事件处理器演示");
    println!("-------------------------------");

    let default_handler = DefaultMomentsEventHandler::new("默认处理器".to_string());
    let default_dispatcher = MomentsEventDispatcher::new(default_handler);

    // 模拟表情互动事件
    let mock_reaction_event = ReactionEvent {
        event_type: Some("created".to_string()),
        reaction: Some(open_lark::service::moments::models::Reaction {
            reaction_id: Some("reaction_789".to_string()),
            post_id: Some("mock_post_123".to_string()),
            user_name: Some("王五".to_string()),
            reaction_type: Some("like".to_string()),
            emoji: Some("👍".to_string()),
            comment_id: None,
            user_id: None,
            create_time: None,
        }),
        event_time: Some("2024-01-15T10:40:00Z".to_string()),
        operator_id: Some("user_789".to_string()),
    };

    let reaction_moments_event = MomentsEvent::ReactionCreated {
        event: mock_reaction_event,
    };
    default_dispatcher.dispatch_event(reaction_moments_event);

    println!("\n🎉 公司圈API和事件处理演示完成!");
    Ok(())
}
