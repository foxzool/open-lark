//! 📡 通讯服务访问层
//!
//! 提供统一的通讯服务接口，封装底层openlark-communication crate
//! 集成 CoreError 错误处理系统，提供企业级错误管理

use crate::{
    error::{api_error, validation_error},
    error::{with_context, with_operation_context},
    Config, DefaultServiceRegistry, Result,
};
use openlark_core::error::ErrorTrait;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// 📡 通讯服务 - 统一访问接口
///
/// 包装openlark-communication crate的功能，提供简洁的API
/// 支持现代错误处理、上下文管理和用户友好的错误消息
#[derive(Debug)]
pub struct CommunicationService<'a> {
    /// 🔧 客户端配置
    config: &'a Config,
    /// 📋 服务注册表
    registry: &'a DefaultServiceRegistry,
    /// 🌐 API端点映射
    endpoints: HashMap<&'static str, &'static str>,
}

impl<'a> CommunicationService<'a> {
    /// 🆕 创建新的通讯服务实例
    pub(crate) fn new(config: &'a Config, registry: &'a DefaultServiceRegistry) -> Result<Self> {
        tracing::info!("初始化通讯服务");

        // 验证配置
        if config.app_id.is_empty() {
            return with_context(
                Err(validation_error("app_id", "应用ID不能为空")),
                "service",
                "communication",
            );
        }

        if config.app_secret.is_empty() {
            return with_context(
                Err(validation_error("app_secret", "应用密钥不能为空")),
                "service",
                "communication",
            );
        }

        // 初始化端点映射
        let mut endpoints = HashMap::new();
        endpoints.insert("send_message", "/open-apis/im/v1/messages");
        endpoints.insert("list_messages", "/open-apis/im/v1/messages");
        endpoints.insert("delete_message", "/open-apis/im/v1/messages");
        endpoints.insert("get_message", "/open-apis/im/v1/messages");
        endpoints.insert("update_message", "/open-apis/im/v1/messages");
        endpoints.insert("send_rich_text", "/open-apis/im/v1/rich_texts");

        let service = Self {
            config,
            registry,
            endpoints,
        };

        tracing::debug!("通讯服务初始化成功，应用ID: {}", config.app_id);

        Ok(service)
    }

    /// 💬 发送文本消息
    ///
    /// # 参数
    /// - `receive_id`: 接收者ID（用户ID、群组ID等）
    /// - `receive_id_type`: 接收者ID类型（open_id、user_id、chat_id等）
    /// - `content`: 消息内容
    ///
    /// # 返回
    /// 返回发送消息的响应信息
    ///
    /// # 示例
    /// ```rust
    /// let response = service.send_text_message("user_123", "open_id", "Hello World!").await?;
    /// println!("消息发送成功，ID: {}", response.message_id);
    /// ```
    pub async fn send_text_message(
        &self,
        receive_id: &str,
        receive_id_type: &str,
        content: &str,
    ) -> Result<SendMessageResponse> {
        let operation_name = "send_text_message";
        tracing::info!("发送文本消息到 {}: {}", receive_id, content);

        // 参数验证
        if receive_id.is_empty() {
            return with_context(
                Err(validation_error("receive_id", "接收者ID不能为空")),
                "operation",
                operation_name,
            );
        }

        if content.is_empty() {
            return with_context(
                Err(validation_error("content", "消息内容不能为空")),
                "operation",
                operation_name,
            );
        }

        if !self.is_valid_receive_id_type(receive_id_type) {
            return with_context(
                Err(validation_error(
                    "receive_id_type",
                    format!("不支持的接收者ID类型: {}", receive_id_type),
                )),
                "operation",
                operation_name,
            );
        }

        // 检查消息长度限制
        if content.len() > 4096 {
            return with_context(
                Err(validation_error(
                    "content",
                    format!("消息内容过长，当前长度: {}, 最大支持: 4096", content.len()),
                )),
                "operation",
                operation_name,
            );
        }

        // 模拟API调用（实际实现中会调用真实的飞书API）
        let api_result = self
            .simulate_send_message(receive_id, receive_id_type, content)
            .await;

        match api_result {
            Ok(response) => {
                tracing::info!("文本消息发送成功，消息ID: {}", response.message_id);
                with_context(Ok(response), "operation", operation_name)
            }
            Err(e) => {
                tracing::error!(
                    "文本消息发送失败: {}",
                    e.user_message().unwrap_or("未知错误")
                );
                with_context(Err(e), "operation", operation_name)
            }
        }
    }

    /// 📨 发送富文本消息
    ///
    /// 发送包含格式化内容的富文本消息
    pub async fn send_rich_text_message(
        &self,
        receive_id: &str,
        receive_id_type: &str,
        rich_content: &RichTextContent,
    ) -> Result<SendMessageResponse> {
        let operation_name = "send_rich_text_message";
        tracing::info!("发送富文本消息到 {}", receive_id);

        // 参数验证
        if receive_id.is_empty() {
            return with_context(
                Err(validation_error("receive_id", "接收者ID不能为空")),
                "operation",
                operation_name,
            );
        }

        if rich_content.is_empty() {
            return with_context(
                Err(validation_error("rich_content", "富文本内容不能为空")),
                "operation",
                operation_name,
            );
        }

        // 序列化富文本内容
        let content_json = serde_json::to_string(rich_content)
            .map_err(|e| crate::error::serialization_error(format!("富文本序列化失败: {}", e)))?;

        // 模拟API调用
        let api_result = self
            .simulate_send_rich_text(receive_id, receive_id_type, &content_json)
            .await;

        match api_result {
            Ok(response) => {
                tracing::info!("富文本消息发送成功，消息ID: {}", response.message_id);
                with_context(Ok(response), "operation", operation_name)
            }
            Err(e) => {
                tracing::error!(
                    "富文本消息发送失败: {}",
                    e.user_message().unwrap_or("未知错误")
                );
                with_context(Err(e), "operation", operation_name)
            }
        }
    }

    /// 📋 获取消息列表
    ///
    /// # 参数
    /// - `container_id_type`: 容器ID类型
    /// - `container_id`: 容器ID
    /// - `page_size`: 分页大小
    /// - `page_token`: 分页令牌
    pub async fn list_messages(
        &self,
        container_id_type: &str,
        container_id: &str,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<ListMessagesResponse> {
        let operation_name = "list_messages";
        tracing::info!(
            "获取消息列表，容器: {} ({})",
            container_id,
            container_id_type
        );

        // 参数验证
        if container_id.is_empty() {
            return with_context(
                Err(validation_error("container_id", "容器ID不能为空")),
                "operation",
                operation_name,
            );
        }

        if !self.is_valid_container_id_type(container_id_type) {
            return with_context(
                Err(validation_error(
                    "container_id_type",
                    format!("不支持的容器ID类型: {}", container_id_type),
                )),
                "operation",
                operation_name,
            );
        }

        if let Some(size) = page_size {
            if size == 0 || size > 200 {
                return with_context(
                    Err(validation_error(
                        "page_size",
                        format!("分页大小必须在1-200之间，当前: {}", size),
                    )),
                    "operation",
                    operation_name,
                );
            }
        }

        // 模拟API调用
        let api_result = self
            .simulate_list_messages(container_id_type, container_id, page_size, page_token)
            .await;

        match api_result {
            Ok(response) => {
                tracing::info!("消息列表获取成功，共 {} 条消息", response.total);
                with_context(Ok(response), "operation", operation_name)
            }
            Err(e) => {
                tracing::error!(
                    "消息列表获取失败: {}",
                    e.user_message().unwrap_or("未知错误")
                );
                with_context(Err(e), "operation", operation_name)
            }
        }
    }

    /// 🗑️ 删除消息
    ///
    /// # 参数
    /// - `message_id`: 消息ID
    /// - `receive_id_type`: 接收者ID类型
    /// - `receive_id`: 接收者ID
    pub async fn delete_message(
        &self,
        message_id: &str,
        receive_id_type: &str,
        receive_id: &str,
    ) -> Result<DeleteMessageResponse> {
        let operation_name = "delete_message";
        tracing::info!("删除消息: {}", message_id);

        // 参数验证
        if message_id.is_empty() {
            return with_context(
                Err(validation_error("message_id", "消息ID不能为空")),
                "operation",
                operation_name,
            );
        }

        if receive_id.is_empty() {
            return with_context(
                Err(validation_error("receive_id", "接收者ID不能为空")),
                "operation",
                operation_name,
            );
        }

        // 模拟API调用
        let api_result = self
            .simulate_delete_message(message_id, receive_id_type, receive_id)
            .await;

        match api_result {
            Ok(response) => {
                tracing::info!("消息删除成功，消息ID: {}", message_id);
                with_context(Ok(response), "operation", operation_name)
            }
            Err(e) => {
                tracing::error!("消息删除失败: {}", e.user_message().unwrap_or("未知错误"));
                with_context(Err(e), "operation", operation_name)
            }
        }
    }

    // ========================================================================
    // 私有辅助方法
    // ========================================================================

    /// 验证接收者ID类型是否有效
    fn is_valid_receive_id_type(&self, receive_id_type: &str) -> bool {
        matches!(
            receive_id_type,
            "open_id" | "user_id" | "union_id" | "chat_id"
        )
    }

    /// 验证容器ID类型是否有效
    fn is_valid_container_id_type(&self, container_id_type: &str) -> bool {
        matches!(
            container_id_type,
            "open_id" | "user_id" | "union_id" | "chat_id"
        )
    }

    /// 模拟发送消息的API调用
    async fn simulate_send_message(
        &self,
        receive_id: &str,
        receive_id_type: &str,
        content: &str,
    ) -> Result<SendMessageResponse> {
        // 模拟网络延迟
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // 模拟可能的错误情况（5%失败率）
        // 使用系统时间戳作为简单的随机源
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        if timestamp % 100 < 5 {
            return with_operation_context(
                Err(api_error(
                    500,
                    self.endpoints
                        .get("send_message")
                        .map_or("/unknown", |v| *v),
                    "模拟API调用失败",
                    Some("req_sim_001".to_string()),
                )),
                "simulate_send_message",
                "CommunicationService",
            );
        }

        // 成功响应
        let response = SendMessageResponse {
            message_id: format!("msg_{}_{}", receive_id, chrono::Utc::now().timestamp()),
            create_time: chrono::Utc::now().timestamp(),
            msg_type: "text".to_string(),
        };

        Ok(response)
    }

    /// 模拟发送富文本消息的API调用
    async fn simulate_send_rich_text(
        &self,
        receive_id: &str,
        receive_id_type: &str,
        content_json: &str,
    ) -> Result<SendMessageResponse> {
        // 模拟网络延迟
        tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;

        // 成功响应
        let response = SendMessageResponse {
            message_id: format!("rich_msg_{}_{}", receive_id, chrono::Utc::now().timestamp()),
            create_time: chrono::Utc::now().timestamp(),
            msg_type: "rich_text".to_string(),
        };

        Ok(response)
    }

    /// 模拟获取消息列表的API调用
    async fn simulate_list_messages(
        &self,
        container_id_type: &str,
        container_id: &str,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<ListMessagesResponse> {
        // 模拟网络延迟
        tokio::time::sleep(tokio::time::Duration::from_millis(80)).await;

        // 生成模拟消息
        let mut messages = Vec::new();
        let message_count = page_size.unwrap_or(20) as usize;

        for i in 0..message_count {
            messages.push(MessageInfo {
                message_id: format!("msg_{}_{}", container_id, i + 1),
                create_time: chrono::Utc::now().timestamp() - (i as i64 * 60),
                msg_type: "text".to_string(),
                content: format!("模拟消息内容 {}", i + 1),
                sender_id: "user_mock_001".to_string(),
                chat_id: container_id.to_string(),
            });
        }

        Ok(ListMessagesResponse {
            items: messages,
            total: 100, // 模拟总数
            has_more: true,
            page_token: page_token.unwrap_or("").to_string(),
        })
    }

    /// 模拟删除消息的API调用
    async fn simulate_delete_message(
        &self,
        message_id: &str,
        _receive_id_type: &str,
        _receive_id: &str,
    ) -> Result<DeleteMessageResponse> {
        // 模拟网络延迟
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        // 模拟权限检查
        if message_id.contains("protected") {
            return with_operation_context(
                Err(api_error(
                    403,
                    self.endpoints
                        .get("delete_message")
                        .map_or("/unknown", |v| *v),
                    "无权限删除该消息",
                    Some("req_sim_002".to_string()),
                )),
                "simulate_delete_message",
                "CommunicationService",
            );
        }

        Ok(DeleteMessageResponse {
            message_id: message_id.to_string(),
            deleted: true,
        })
    }
}

// ========================================================================
// 数据结构定义
// ========================================================================

/// 📤 发送消息响应
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SendMessageResponse {
    /// 🏷️ 消息ID
    pub message_id: String,
    /// ⏰ 创建时间
    pub create_time: i64,
    /// 📝 消息类型
    pub msg_type: String,
}

/// 📨 富文本内容
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RichTextContent {
    /// 📄 富文本片段列表
    pub elements: Vec<RichTextElement>,
}

impl RichTextContent {
    /// 创建新的富文本内容
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    /// 添加文本片段
    pub fn add_text(&mut self, content: &str) {
        self.elements.push(RichTextElement::Text {
            content: content.to_string(),
        });
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

/// 📄 富文本元素
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "tag")]
pub enum RichTextElement {
    /// 文本内容
    Text {
        /// 文本内容
        content: String,
    },
    /// 链接
    Link {
        /// 链接文本
        text: String,
        /// 链接地址
        href: String,
    },
    /// 用户提及
    UserMention {
        /// 用户ID
        user_id: String,
        /// 用户名
        user_name: String,
    },
}

/// 📋 消息信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MessageInfo {
    /// 🏷️ 消息ID
    pub message_id: String,
    /// ⏰ 创建时间
    pub create_time: i64,
    /// 📝 消息类型
    pub msg_type: String,
    /// 📄 消息内容
    pub content: String,
    /// 👤 发送者ID
    pub sender_id: String,
    /// 💬 群组ID
    pub chat_id: String,
}

/// 📋 获取消息列表响应
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ListMessagesResponse {
    /// 📝 消息列表
    pub items: Vec<MessageInfo>,
    /// 🔢 总数
    pub total: u32,
    /// 📖 是否有更多数据
    pub has_more: bool,
    /// 📄 分页令牌
    pub page_token: String,
}

/// 🗑️ 删除消息响应
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeleteMessageResponse {
    /// 🏷️ 消息ID
    pub message_id: String,
    /// ✅ 是否已删除
    pub deleted: bool,
}

impl Default for RichTextContent {
    fn default() -> Self {
        Self::new()
    }
}

// ========================================================================
// 测试模块
// ========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_config() -> Config {
        Config {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        }
    }

    #[test]
    fn test_communication_service_creation_success() {
        let config = create_test_config();
        let registry = DefaultServiceRegistry::new();

        let result = CommunicationService::new(&config, &registry);

        assert!(result.is_ok(), "通讯服务创建应该成功");

        let service = result.unwrap();
        assert_eq!(service.config.app_id, "test_app_id");
        assert_eq!(service.config.app_secret, "test_app_secret");
    }

    #[test]
    fn test_communication_service_creation_with_empty_app_id() {
        let mut config = create_test_config();
        config.app_id = "".to_string();
        let registry = DefaultServiceRegistry::new();

        let result = CommunicationService::new(&config, &registry);

        assert!(result.is_err(), "空的app_id应该导致创建失败");

        if let Err(error) = result {
            assert!(error.is_validation_error());
            assert!(error
                .user_message()
                .unwrap_or("未知错误")
                .contains("应用ID不能为空"));
        }
    }

    #[test]
    fn test_communication_service_creation_with_empty_app_secret() {
        let mut config = create_test_config();
        config.app_secret = "".to_string();
        let registry = DefaultServiceRegistry::new();

        let result = CommunicationService::new(&config, &registry);

        assert!(result.is_err(), "空的app_secret应该导致创建失败");

        if let Err(error) = result {
            assert!(error.is_validation_error());
            assert!(error
                .user_message()
                .unwrap_or("未知错误")
                .contains("应用密钥不能为空"));
        }
    }

    #[tokio::test]
    async fn test_send_text_message_success() {
        let config = create_test_config();
        let registry = DefaultServiceRegistry::new();
        let service = CommunicationService::new(&config, &registry).unwrap();

        let result = service
            .send_text_message("test_user_123", "open_id", "Hello, World!")
            .await;

        assert!(result.is_ok(), "发送文本消息应该成功");

        if let Ok(response) = result {
            assert_eq!(response.msg_type, "text");
            assert!(!response.message_id.is_empty());
            assert!(response.create_time > 0);
        }
    }

    #[tokio::test]
    async fn test_send_text_message_with_empty_receive_id() {
        let config = create_test_config();
        let registry = DefaultServiceRegistry::new();
        let service = CommunicationService::new(&config, &registry).unwrap();

        let result = service
            .send_text_message("", "open_id", "Hello, World!")
            .await;

        assert!(result.is_err(), "空的接收者ID应该导致发送失败");

        if let Err(error) = result {
            assert!(error.is_validation_error());
            assert!(error
                .user_message()
                .unwrap_or("未知错误")
                .contains("接收者ID不能为空"));
        }
    }

    #[tokio::test]
    async fn test_send_text_message_with_empty_content() {
        let config = create_test_config();
        let registry = DefaultServiceRegistry::new();
        let service = CommunicationService::new(&config, &registry).unwrap();

        let result = service
            .send_text_message("test_user_123", "open_id", "")
            .await;

        assert!(result.is_err(), "空的消息内容应该导致发送失败");

        if let Err(error) = result {
            assert!(error.is_validation_error());
            assert!(error
                .user_message()
                .unwrap_or("未知错误")
                .contains("消息内容不能为空"));
        }
    }

    #[tokio::test]
    async fn test_send_text_message_with_invalid_receive_id_type() {
        let config = create_test_config();
        let registry = DefaultServiceRegistry::new();
        let service = CommunicationService::new(&config, &registry).unwrap();

        let result = service
            .send_text_message("test_user_123", "invalid_type", "Hello, World!")
            .await;

        assert!(result.is_err(), "无效的接收者ID类型应该导致发送失败");

        if let Err(error) = result {
            assert!(error.is_validation_error());
            assert!(error
                .user_message()
                .unwrap_or("未知错误")
                .contains("不支持的接收者ID类型"));
        }
    }

    #[tokio::test]
    async fn test_send_text_message_with_oversized_content() {
        let config = create_test_config();
        let registry = DefaultServiceRegistry::new();
        let service = CommunicationService::new(&config, &registry).unwrap();

        let long_content = "x".repeat(5000); // 超过4096字符限制
        let result = service
            .send_text_message("test_user_123", "open_id", &long_content)
            .await;

        assert!(result.is_err(), "过长的消息内容应该导致发送失败");

        if let Err(error) = result {
            assert!(error.is_validation_error());
            assert!(error
                .user_message()
                .unwrap_or("未知错误")
                .contains("消息内容过长"));
        }
    }

    #[tokio::test]
    async fn test_send_rich_text_message_success() {
        let config = create_test_config();
        let registry = DefaultServiceRegistry::new();
        let service = CommunicationService::new(&config, &registry).unwrap();

        let mut rich_content = RichTextContent::new();
        rich_content.add_text("Hello, ");
        rich_content.add_text("World!");

        let result = service
            .send_rich_text_message("test_user_123", "open_id", &rich_content)
            .await;

        assert!(result.is_ok(), "发送富文本消息应该成功");

        if let Ok(response) = result {
            assert_eq!(response.msg_type, "rich_text");
            assert!(!response.message_id.is_empty());
        }
    }

    #[tokio::test]
    async fn test_list_messages_success() {
        let config = create_test_config();
        let registry = DefaultServiceRegistry::new();
        let service = CommunicationService::new(&config, &registry).unwrap();

        let result = service
            .list_messages("chat", "chat_123", Some(10), None)
            .await;

        assert!(result.is_ok(), "获取消息列表应该成功");

        if let Ok(response) = result {
            assert_eq!(response.items.len(), 10); // 请求了10条消息
            assert!(response.total > 0);
            assert!(response.has_more);
        }
    }

    #[tokio::test]
    async fn test_list_messages_with_invalid_page_size() {
        let config = create_test_config();
        let registry = DefaultServiceRegistry::new();
        let service = CommunicationService::new(&config, &registry).unwrap();

        let result = service
            .list_messages("chat", "chat_123", Some(0), None) // 无效的page_size
            .await;

        assert!(result.is_err(), "无效的分页大小应该导致获取失败");

        if let Err(error) = result {
            assert!(error.is_validation_error());
            assert!(error
                .user_message()
                .unwrap_or("未知错误")
                .contains("分页大小必须在1-200之间"));
        }
    }

    #[tokio::test]
    async fn test_delete_message_success() {
        let config = create_test_config();
        let registry = DefaultServiceRegistry::new();
        let service = CommunicationService::new(&config, &registry).unwrap();

        let result = service
            .delete_message("msg_123", "open_id", "user_123")
            .await;

        assert!(result.is_ok(), "删除消息应该成功");

        if let Ok(response) = result {
            assert_eq!(response.message_id, "msg_123");
            assert!(response.deleted);
        }
    }

    #[tokio::test]
    async fn test_delete_protected_message_should_fail() {
        let config = create_test_config();
        let registry = DefaultServiceRegistry::new();
        let service = CommunicationService::new(&config, &registry).unwrap();

        let result = service
            .delete_message("msg_protected_123", "open_id", "user_123")
            .await;

        assert!(result.is_err(), "删除受保护的消息应该失败");

        if let Err(error) = result {
            assert!(error.is_business_error() || error.is_api_error());
            assert!(
                error
                    .user_message()
                    .unwrap_or("未知错误")
                    .contains("无权限")
                    || error.user_message().unwrap_or("未知错误").contains("权限")
            );
        }
    }

    #[test]
    fn test_rich_text_content() {
        let mut content = RichTextContent::new();
        assert!(content.is_empty());

        content.add_text("Hello");
        assert!(!content.is_empty());
        assert_eq!(content.elements.len(), 1);
    }

    #[test]
    fn test_receive_id_type_validation() {
        let config = create_test_config();
        let registry = DefaultServiceRegistry::new();
        let service = CommunicationService::new(&config, &registry).unwrap();

        // 有效的接收者ID类型
        assert!(service.is_valid_receive_id_type("open_id"));
        assert!(service.is_valid_receive_id_type("user_id"));
        assert!(service.is_valid_receive_id_type("union_id"));
        assert!(service.is_valid_receive_id_type("chat_id"));

        // 无效的接收者ID类型
        assert!(!service.is_valid_receive_id_type("invalid"));
        assert!(!service.is_valid_receive_id_type("email"));
        assert!(!service.is_valid_receive_id_type("phone"));
    }

    #[test]
    fn test_container_id_type_validation() {
        let config = create_test_config();
        let registry = DefaultServiceRegistry::new();
        let service = CommunicationService::new(&config, &registry).unwrap();

        // 有效的容器ID类型
        assert!(service.is_valid_container_id_type("open_id"));
        assert!(service.is_valid_container_id_type("user_id"));
        assert!(service.is_valid_container_id_type("union_id"));
        assert!(service.is_valid_container_id_type("chat_id"));

        // 无效的容器ID类型
        assert!(!service.is_valid_container_id_type("invalid"));
        assert!(!service.is_valid_container_id_type("department_id"));
    }

    #[tokio::test]
    async fn test_error_context_and_analysis() {
        let config = create_test_config();
        let registry = DefaultServiceRegistry::new();
        let service = CommunicationService::new(&config, &registry).unwrap();

        // 触发验证错误
        let result = service.send_text_message("", "open_id", "test").await;
        assert!(result.is_err());

        if let Err(error) = result {
            // 检查错误上下文
            assert!(error.has_context("operation"));
            assert_eq!(error.get_context("operation"), Some("send_text_message"));

            // 检查错误分析功能
            let report = error.detailed_report();
            assert!(report.contains("错误分析报告"));
            assert!(report.contains("验证错误"));

            // 检查用户友好的错误消息
            let user_msg = error.user_friendly_with_suggestion();
            assert!(user_msg.contains("建议"));
            assert!(user_msg.contains("可以尝试"));
        }
    }
}
