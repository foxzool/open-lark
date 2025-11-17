//! OpenLark 高级统一API接口
//!
//! 提供最上层的统一API调用接口，隐藏服务细节，提供一致的用户体验。

use std::sync::Arc;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use super::{
    client::UnifiedClient,
    error::{UnifiedError, UnifiedResult},
    traits::FromEnvConfig,
    api::{
        APIRequest, APIResponse, SendTextMessageRequest, SendImageMessageRequest,
        ListMessagesRequest, ListEmployeesRequest, GetEmployeeRequest,
        CreateSpreadsheetRequest, ReadRangeRequest, GenerateTextRequest,
        GetAppAccessTokenRequest
    },
};

/// OpenLark 统一API客户端
///
/// 提供最高层次的API访问接口，用户可以通过一个客户端访问所有飞书服务。
#[derive(Debug, Clone)]
pub struct OpenLarkClient {
    client: Arc<UnifiedClient>,
}

impl OpenLarkClient {
    /// 创建新的OpenLark客户端
    pub async fn new(config: super::config::UnifiedConfig) -> UnifiedResult<Self> {
        let client = UnifiedClient::new(config).await?;
        Ok(Self {
            client: Arc::new(client),
        })
    }

    /// 从环境变量创建客户端
    pub async fn from_env() -> UnifiedResult<Self> {
        let config = super::config::UnifiedConfig::from_env()?;
        Self::new(config).await
    }

    /// 获取底层统一客户端
    pub fn inner(&self) -> &UnifiedClient {
        &self.client
    }

    // ==================== 通信服务API ====================

    /// 发送文本消息
    ///
    /// # 参数
    /// - `receive_id`: 接收者ID
    /// - `receive_id_type`: 接收者ID类型（open_id, user_id, union_id, chat_id等）
    /// - `content`: 消息内容
    ///
    /// # 示例
    /// ```rust
    /// let client = OpenLarkClient::from_env().await?;
    /// let result = client.send_text_message("user_xxx", "open_id", "Hello, World!").await?;
    /// println!("消息发送成功: {}", result.message_id);
    /// ```
    pub async fn send_text_message(
        &self,
        receive_id: &str,
        receive_id_type: &str,
        content: &str,
    ) -> UnifiedResult<super::services::communication::MessageSendResult> {
        let request = SendTextMessageRequest {
            receive_id: receive_id.to_string(),
            receive_id_type: receive_id_type.to_string(),
            content: content.to_string(),
        };

        let api_client = self.client.api();
        api_client.call::<super::services::CommunicationService, _>(request).await
    }

    /// 发送图片消息
    ///
    /// # 参数
    /// - `receive_id`: 接收者ID
    /// - `receive_id_type`: 接收者ID类型
    /// - `image_key`: 图片key
    pub async fn send_image_message(
        &self,
        receive_id: &str,
        receive_id_type: &str,
        image_key: &str,
    ) -> UnifiedResult<super::services::communication::MessageSendResult> {
        let request = SendImageMessageRequest {
            receive_id: receive_id.to_string(),
            receive_id_type: receive_id_type.to_string(),
            image_key: image_key.to_string(),
        };

        let api_client = self.client.api();
        api_client.call::<super::services::CommunicationService, _>(request).await
    }

    /// 获取消息列表
    ///
    /// # 参数
    /// - `container_id`: 容器ID（聊天ID等）
    /// - `container_id_type`: 容器ID类型
    /// - `page_size`: 分页大小（可选）
    /// - `page_token`: 分页令牌（可选）
    pub async fn list_messages(
        &self,
        container_id: &str,
        container_id_type: &str,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> UnifiedResult<super::services::communication::MessageListResult> {
        let request = ListMessagesRequest {
            container_id: container_id.to_string(),
            container_id_type: container_id_type.to_string(),
            page_size,
            page_token: page_token.map(|s| s.to_string()),
        };

        let api_client = self.client.api();
        api_client.call::<super::services::CommunicationService, _>(request).await
    }

    // ==================== HR服务API ====================

    /// 获取员工列表
    ///
    /// # 参数
    /// - `user_id_type`: 用户ID类型（可选）
    /// - `page_size`: 分页大小（可选）
    /// - `page_token`: 分页令牌（可选）
    ///
    /// # 示例
    /// ```rust
    /// let client = OpenLarkClient::from_env().await?;
    /// let result = client.list_employees(Some("open_id"), Some(50), None).await?;
    /// for employee in result.employees {
    ///     println!("员工: {} ({})", employee.name, employee.user_id);
    /// }
    /// ```
    pub async fn list_employees(
        &self,
        user_id_type: Option<&str>,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> UnifiedResult<super::services::hr::EmployeeListResult> {
        let request = ListEmployeesRequest {
            user_id_type: user_id_type.map(|s| s.to_string()),
            page_size,
            page_token: page_token.map(|s| s.to_string()),
        };

        let api_client = self.client.api();
        api_client.call::<super::services::HRService, _>(request).await
    }

    /// 获取单个员工信息
    ///
    /// # 参数
    /// - `user_id`: 用户ID
    /// - `user_id_type`: 用户ID类型（可选）
    pub async fn get_employee(
        &self,
        user_id: &str,
        user_id_type: Option<&str>,
    ) -> UnifiedResult<super::services::hr::Employee> {
        let request = GetEmployeeRequest {
            user_id: user_id.to_string(),
            user_id_type: user_id_type.map(|s| s.to_string()),
        };

        let api_client = self.client.api();
        api_client.call::<super::services::HRService, _>(request).await
    }

    // ==================== 文档服务API ====================

    /// 创建电子表格
    ///
    /// # 参数
    /// - `title`: 表格标题
    /// - `folder_token`: 文件夹token（可选）
    ///
    /// # 示例
    /// ```rust
    /// let client = OpenLarkClient::from_env().await?;
    /// let spreadsheet = client.create_spreadsheet("我的新表格", None).await?;
    /// println!("表格创建成功: {}", spreadsheet.title);
    /// println!("访问链接: {}", spreadsheet.url);
    /// ```
    pub async fn create_spreadsheet(
        &self,
        title: &str,
        folder_token: Option<&str>,
    ) -> UnifiedResult<super::services::docs::Spreadsheet> {
        let request = CreateSpreadsheetRequest {
            title: title.to_string(),
            folder_token: folder_token.map(|s| s.to_string()),
        };

        let api_client = self.client.api();
        api_client.call::<super::services::DocsService, _>(request).await
    }

    /// 读取工作表范围
    ///
    /// # 参数
    /// - `spreadsheet_token`: 表格token
    /// - `range`: 读取范围（如 "Sheet1!A1:C10"）
    /// - `value_render_option`: 值渲染选项（可选）
    pub async fn read_range(
        &self,
        spreadsheet_token: &str,
        range: &str,
        value_render_option: Option<&str>,
    ) -> UnifiedResult<super::services::docs::RangeReadResult> {
        let request = ReadRangeRequest {
            spreadsheet_token: spreadsheet_token.to_string(),
            range: range.to_string(),
            value_render_option: value_render_option.map(|s| s.to_string()),
        };

        let api_client = self.client.api();
        api_client.call::<super::services::DocsService, _>(request).await
    }

    // ==================== AI服务API ====================

    /// 生成文本
    ///
    /// # 参数
    /// - `prompt`: 提示词
    /// - `model`: 模型名称（可选）
    /// - `temperature`: 温度参数（可选，0.0-2.0）
    /// - `max_tokens`: 最大token数（可选）
    ///
    /// # 示例
    /// ```rust
    /// let client = OpenLarkClient::from_env().await?;
    /// let result = client.generate_text(
    ///     "写一首关于春天的诗",
    ///     Some("gpt-3.5-turbo"),
    ///     Some(0.7),
    ///     Some(100)
    /// ).await?;
    /// println!("生成的文本: {}", result.text);
    /// ```
    pub async fn generate_text(
        &self,
        prompt: &str,
        model: Option<&str>,
        temperature: Option<f32>,
        max_tokens: Option<u32>,
    ) -> UnifiedResult<super::services::ai::TextGenerationResult> {
        let request = GenerateTextRequest {
            prompt: prompt.to_string(),
            model: model.map(|s| s.to_string()),
            temperature,
            max_tokens,
        };

        let api_client = self.client.api();
        api_client.call::<super::services::AIService, _>(request).await
    }

    // ==================== 认证服务API ====================

    /// 获取应用访问令牌
    ///
    /// # 示例
    /// ```rust
    /// let client = OpenLarkClient::from_env().await?;
    /// let token_info = client.get_app_access_token().await?;
    /// println!("访问令牌: {}", token_info.access_token);
    /// println!("过期时间: {}", token_info.expires_at);
    /// ```
    pub async fn get_app_access_token(&self) -> UnifiedResult<super::services::auth::TokenInfo> {
        let request = GetAppAccessTokenRequest;

        let api_client = self.client.api();
        api_client.call::<super::services::AuthService, _>(request).await
    }

    // ==================== 批量操作API ====================

    /// 批量发送消息
    ///
    /// # 参数
    /// - `messages`: 消息列表，每个元素包含 (receive_id, receive_id_type, content)
    pub async fn batch_send_text_messages(
        &self,
        messages: Vec<(String, String, String)>,
    ) -> UnifiedResult<Vec<UnifiedResult<super::services::communication::MessageSendResult>>> {
        let mut results = Vec::with_capacity(messages.len());

        for (receive_id, receive_id_type, content) in messages {
            let result = self.send_text_message(&receive_id, &receive_id_type, &content).await;
            results.push(result);
        }

        Ok(results)
    }

    /// 批量获取员工信息
    ///
    /// # 参数
    /// - `user_ids`: 用户ID列表
    /// - `user_id_type`: 用户ID类型（可选）
    pub async fn batch_get_employees(
        &self,
        user_ids: Vec<String>,
        user_id_type: Option<&str>,
    ) -> UnifiedResult<Vec<UnifiedResult<super::services::hr::Employee>>> {
        let mut results = Vec::with_capacity(user_ids.len());

        for user_id in user_ids {
            let result = self.get_employee(&user_id, user_id_type).await;
            results.push(result);
        }

        Ok(results)
    }

    // ==================== 服务管理API ====================

    /// 检查服务可用性
    ///
    /// # 参数
    /// - `service_name`: 服务名称（communication, hr, docs, ai, auth）
    pub fn is_service_available(&self, service_name: &str) -> bool {
        self.client.is_service_available(service_name)
    }

    /// 获取所有可用服务列表
    pub fn available_services(&self) -> Vec<&str> {
        self.client.list_services()
    }

    /// 执行健康检查
    pub async fn health_check(&self) -> UnifiedResult<std::collections::HashMap<String, bool>> {
        self.client.health_check().await
    }

    /// 获取服务状态
    pub async fn service_status(&self) -> UnifiedResult<std::collections::HashMap<String, String>> {
        let services = self.client.list_services();
        let mut status_map = std::collections::HashMap::new();

        for service_name in services {
            if let Some(service_info) = self.client.registry().get_service_info(&service_name) {
                status_map.insert(
                    service_name,
                    format!("{:?}", service_info.status)
                );
            }
        }

        Ok(status_map)
    }
}

/// OpenLark客户端构建器
///
/// 提供流畅的API来配置和构建OpenLark客户端。
#[derive(Debug, Clone)]
pub struct OpenLarkClientBuilder {
    config: Option<super::config::UnifiedConfig>,
    timeout: Option<std::time::Duration>,
    retries: Option<u32>,
}

impl OpenLarkClientBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self {
            config: None,
            timeout: None,
            retries: None,
        }
    }

    /// 设置配置
    pub fn config(mut self, config: super::config::UnifiedConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// 从环境变量加载配置
    pub fn from_env() -> UnifiedResult<Self> {
        let config = super::config::UnifiedConfig::from_env()?;
        Ok(Self::new().config(config))
    }

    /// 设置默认超时时间
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// 设置默认重试次数
    pub fn retries(mut self, retries: u32) -> Self {
        self.retries = Some(retries);
        self
    }

    /// 构建客户端
    pub async fn build(self) -> UnifiedResult<OpenLarkClient> {
        let config = self.config.ok_or_else(|| {
            UnifiedError::ConfigurationError("客户端配置未设置".to_string())
        })?;

        OpenLarkClient::new(config).await
    }
}

impl Default for OpenLarkClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_openlark_client_builder() {
        let builder = OpenLarkClientBuilder::new()
            .timeout(std::time::Duration::from_secs(30))
            .retries(3);

        // 由于没有实际配置，这里只测试构建器创建
        assert!(builder.timeout.is_some());
        assert_eq!(builder.retries, Some(3));
    }

    #[tokio::test]
    async fn test_service_availability() {
        // 这个测试需要实际的客户端配置
        // 在实际环境中可以从环境变量加载配置
        // let client = OpenLarkClient::from_env().await?;
        // let services = client.available_services();
        // assert!(!services.is_empty());
    }
}