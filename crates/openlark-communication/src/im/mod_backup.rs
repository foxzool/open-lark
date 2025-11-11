//! 即时消息（IM）服务
//!
//! 提供飞书即时消息相关的所有API功能，包括消息发送、接收、管理等。
//! 支持多种消息类型：文本、富文本、图片、文件、卡片等。
//!
//! # API版本
//!
//! - **v1**: 稳定版本，包含核心消息功能
//! - **v2**: 新版本，包含增强功能
//!
//! # 主要功能
//!
//! - 📨 消息发送和接收
//! - 🎨 富文本和卡片消息
//! - 📁 文件和媒体消息
//! - 👥 群聊管理
//! - 🔔 消息推送和通知
//!
//! # 快速开始
//!
//! ## 基础消息发送
//!
//! ```rust
//! use open_lark::prelude::*;
//!
//! let client = LarkClient::builder("your_app_id", "your_app_secret")
//!     .with_app_type(AppType::SelfBuild)
//!     .build();
//!
//! // 发送简单文本消息
//! let message = CreateMessageRequestBody::builder()
//!     .receive_id_type("open_id")
//!     .msg_type("text")
//!     .content("{\"text\":\"Hello, World!\"}")
//!     .build();
//!
//! let request = CreateMessageRequest::builder()
//!     .request_body(message)
//!     .build();
//!
//! let response = client.im.v1.message.create(request, None).await?;
//! println!("消息发送成功: {:?}", response);
//! ```
//!
//! ## 富文本消息
//!
//! ```rust
//! use open_lark::prelude::*;
//! use serde_json::json;
//!
//! let rich_content = json!({
//!     "config": {
//!         "wide_screen_mode": true
//!     },
//!     "elements": [
//!         {
//!             "tag": "text",
//!             "text": "🎉 这是一条富文本消息"
//!         },
//!         {
//!             "tag": "a",
//!             "text": "点击查看详情",
//!             "href": "https://example.com"
//!         }
//!     ]
//! });
//!
//! let message = CreateMessageRequestBody::builder()
//!     .receive_id_type("open_id")
//!     .msg_type("post")
//!     .content(rich_content.to_string())
//!     .build();
//!
//! let response = client.im.v1.message.create(request, None).await?;
//! ```
//!
//! ## 发送图片消息
//!
//! ```rust
//! use open_lark::prelude::*;
//! use openlark_core::constants::AccessTokenType;
//!
//! // 需要先上传图片文件获取 image_key
//! let upload_response = client.im.v1.file.upload().await?;
//! let image_key = upload_response?.data?.image_key;
//!
//! let message = CreateMessageRequestBody::builder()
//!     .receive_id_type("open_id")
//!     .msg_type("image")
//!     .content(json!({
//!         "image_key": image_key
//!     }).to_string())
//!     .build();
//!
//! let response = client.im.v1.message.create(request, Some(AccessTokenType::Tenant)).await?;
//! ```
//!
//! ## 批量消息发送
//!
//! ```rust
//! use open_lark::prelude::*;
//!
//! let messages = vec![
//!     CreateMessageRequest::builder()
//!         .request_body("第一条消息")
//!         .build(),
//!     CreateMessageRequest::builder()
//!         .request_body("第二条消息")
//!         .build(),
//!     CreateMessageRequest::builder()
//!         .request_body("第三条消息")
//!         .build(),
//! ];
//!
//! for request in messages {
//!     match client.im.v1.message.create(request, None).await {
//!         Ok(response) => println!("消息 {} 发送成功", response),
//!         Err(error) => println!("消息发送失败: {}", error.user_friendly_message()),
//!     }
//! }
//! ```
//!
//! ## 消息接收与事件处理
//!
//! ```rust
//! use open_lark::prelude::*;
//! use open_lark_event::v1::{MessageReceiveV1Handler, MessageReceiveV1Data};
//!
//! // 创建消息接收处理器
//! let handler = MessageReceiveV1Handler::new(|event_data| {
//!     println!("收到消息: {:?}", event_data);
//!     // 处理消息逻辑
//! });
//!
//! // 注册处理器（通常在客户端初始化时进行）
//! client.event.add_handler(Box::new(handler));
//! ```
//!
//! ## 最佳实践
//!
//! ### 错误处理
//!
//! ```rust
//! use open_lark::prelude::*;
//!
//! match client.im.v1.message.create(request, None).await {
//!     Ok(response) => {
//!         println!("消息发送成功: {:?}", response);
//!     }
//!     Err(error) => {
//!         println!("消息发送失败: {}", error.user_friendly_message());
//!         // 实现重试逻辑
//!         match error.error_code() {
//!             9999 => {
//!                 // 系统繁忙，稍后重试
//!                 tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
//!                 // 重试逻辑
//!             }
//!             _ => {
//!                 // 其他错误的处理逻辑
//!             }
//!         }
//!     }
//! ```
//!
//! ### 性能优化
//!
//! ```rust
//! use open_lark::prelude::*;
//! use openlark_core::constants::AccessTokenType;
//!
//! // 1. 启用令牌缓存
//! let client = LarkClient::builder("app_id", "app_secret")
//!     .enable_token_cache(true)
//!     .build();
//!
//! // 2. 使用合适的访问令牌类型
//! let response = client.im.v1.message.create(request, Some(AccessTokenType::Tenant)).await?;
//!
//! // 3. 批量操作优化
//! let messages = vec![
//!     CreateMessageRequest::builder().request_body("第一条消息").build(),
//!     CreateMessageRequest::builder().request_body("第二条消息").build(),
//!     CreateMessageRequest::builder().request_body("第三条消息").build(),
//! ];
//!
//! for request in messages {
//!     match client.im.v1.message.create(request, None).await {
//!         Ok(response) => println!("消息发送成功"),
//!         Err(error) => println!("消息发送失败: {}", error.user_friendly_message()),
//!     }
//! }
//! ```
//!
//! ### 开发者体验改进
//!
//! ```rust
//! use open_lark::prelude::*;
//!
//! // 设置合理的超时时间
//! let client = LarkClient::builder("app_id", "app_secret")
//!     .req_timeout(std::time::Duration::from_secs(30))
//!     .build();
//!
//! // 使用自定义用户代理字符串
//! let custom_client = client
//!     .with_user_agent("MyApp/1.0")
//!     .build();
//! ```