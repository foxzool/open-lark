//! OpenLark 统一API接口
//!
//! 提供统一的API调用模式，确保所有服务都有一致的调用体验。

use std::marker::PhantomData;
use std::sync::Arc;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use openlark_core::SDKResult;

use super::{
    traits::{UnifiedService, APICall, ServiceStatus},
    error::{UnifiedError, UnifiedResult},
    services::{
        communication::{MessageSendResult, MessageListResult},
        hr::{EmployeeListResult, Employee},
        docs::{Spreadsheet, RangeReadResult},
        ai::{TextGenerationResult},
        auth::{TokenInfo},
    },
};

/// 统一API请求特征
///
/// 所有API请求都必须实现此特征。
pub trait APIRequest: Send + Sync + Clone + Serialize {
    /// 响应类型
    type Response: APIResponse + Send + 'static;

    /// 请求类型标识符
    fn request_type(&self) -> &'static str;

    /// 验证请求参数
    fn validate(&self) -> UnifiedResult<()>;

    /// 获取请求元数据
    fn metadata(&self) -> std::collections::HashMap<String, String> {
        std::collections::HashMap::new()
    }
}

/// 统一API响应特征
///
/// 所有API响应都必须实现此特征。
pub trait APIResponse: Send + Sync + Clone + Deserialize<'static> {
    /// 请求类型标识符
    fn response_type(&self) -> &'static str;

    /// 检查响应是否成功
    fn is_success(&self) -> bool;

    /// 获取错误信息（如果有）
    fn error(&self) -> Option<&APIError> {
        None
    }

    /// 获取响应元数据
    fn metadata(&self) -> std::collections::HashMap<String, String> {
        std::collections::HashMap::new()
    }
}

/// API错误信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIError {
    /// 错误代码
    pub code: i32,
    /// 错误消息
    pub message: String,
    /// 错误详情
    pub details: Option<serde_json::Value>,
    /// 错误类型
    pub error_type: String,
}

impl std::fmt::Display for APIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "API错误[{}]: {}", self.code, self.message)
    }
}

impl std::error::Error for APIError {}

/// 统一API调用构建器
///
/// 提供流畅的API来构建和执行API调用。
#[derive(Debug, Clone)]
pub struct APIBuilder<S, R> {
    service: PhantomData<S>,
    request: PhantomData<R>,
    params: std::collections::HashMap<String, serde_json::Value>,
    headers: std::collections::HashMap<String, String>,
    timeout: Option<std::time::Duration>,
    retries: u32,
    middleware: Vec<String>,
}

impl<S, R> Default for APIBuilder<S, R> {
    fn default() -> Self {
        Self {
            service: PhantomData,
            request: PhantomData,
            params: std::collections::HashMap::new(),
            headers: std::collections::HashMap::new(),
            timeout: None,
            retries: 3,
            middleware: Vec::new(),
        }
    }
}

impl<S, R> APIBuilder<S, R>
where
    S: UnifiedService + 'static,
    R: APIRequest,
{
    /// 创建新的API构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 添加请求参数
    pub fn param<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<serde_json::Value>,
    {
        self.params.insert(key.into(), value.into());
        self
    }

    /// 批量添加请求参数
    pub fn params<I, K, V>(mut self, params: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<serde_json::Value>,
    {
        for (key, value) in params {
            self.params.insert(key.into(), value.into());
        }
        self
    }

    /// 添加请求头
    pub fn header<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.headers.insert(key.into(), value.into());
        self
    }

    /// 批量添加请求头
    pub fn headers<I, K, V>(mut self, headers: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        for (key, value) in headers {
            self.headers.insert(key.into(), value.into());
        }
        self
    }

    /// 设置超时时间
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// 设置重试次数
    pub fn retries(mut self, retries: u32) -> Self {
        self.retries = retries;
        self
    }

    /// 添加中间件
    pub fn middleware(mut self, middleware: &str) -> Self {
        self.middleware.push(middleware.to_string());
        self
    }

    /// 构建请求对象
    pub fn build_request(&self) -> UnifiedResult<R> {
        // 这里需要根据具体的请求类型来构建请求
        // 暂时返回一个错误，实际实现中会根据R类型来构建
        Err(UnifiedError::NotImplemented(
            "build_request需要为具体请求类型实现".to_string()
        ))
    }

    /// 执行API调用
    pub async fn execute(&self, client: &super::client::UnifiedClient) -> UnifiedResult<R::Response> {
        // 构建请求
        let request = self.build_request()?;

        // 验证请求
        request.validate()?;

        // 获取服务并执行调用
        let service = client.service::<S>()?;

        if service.status() != ServiceStatus::Running {
            return Err(UnifiedError::ServiceNotAvailable(
                std::any::type_name::<S>().to_string()
            ));
        }

        // 创建API调用执行器
        let executor = APIExecutor::new(client.clone());

        // 执行调用
        if self.retries > 0 {
            executor.execute_with_retry(request, self.retries).await
        } else {
            executor.execute(request).await
        }
    }
}

/// API执行器
///
/// 负责实际执行API调用的组件。
#[derive(Debug, Clone)]
pub struct APIExecutor {
    client: Arc<super::client::UnifiedClient>,
}

impl APIExecutor {
    /// 创建新的API执行器
    pub fn new(client: Arc<super::client::UnifiedClient>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<R> APICall<R, R::Response> for APIExecutor
where
    R: APIRequest + Send + Sync + 'static,
{
    async fn execute(&self, request: R) -> SDKResult<R::Response> {
        // 这里实现实际的API调用逻辑
        // 包括：序列化请求、HTTP调用、反序列化响应等

        tracing::debug!("执行API调用: {}", request.request_type());

        // 验证请求
        request.validate().map_err(|e| openlark_core::error::LarkAPIError::InvalidRequest(e.to_string()))?;

        // 暂时返回一个错误，表示需要具体实现
        // 在实际使用中，这里会调用具体的服务方法
        Err(openlark_core::error::LarkAPIError::UnknownError(
            "API执行逻辑需要根据具体服务实现".to_string()
        ))
    }
}

/// 统一API客户端
///
/// 提供所有服务的统一API访问接口。
#[derive(Debug, Clone)]
pub struct UnifiedAPIClient {
    client: Arc<super::client::UnifiedClient>,
}

impl UnifiedAPIClient {
    /// 创建新的统一API客户端
    pub fn new(client: Arc<super::client::UnifiedClient>) -> Self {
        Self { client }
    }

    /// 获取底层客户端
    pub fn client(&self) -> &super::client::UnifiedClient {
        &self.client
    }

    /// 创建API构建器
    pub fn api<S, R>(&self) -> APIBuilder<S, R>
    where
        S: UnifiedService + 'static,
        R: APIRequest,
    {
        APIBuilder::new()
    }

    /// 直接执行API调用
    pub async fn call<S, R>(&self, request: R) -> UnifiedResult<R::Response>
    where
        S: UnifiedService + 'static,
        R: APIRequest + Send + Sync,
    {
        let executor = APIExecutor::new(self.client.clone());
        executor.execute(request).await
    }

    /// 批量执行API调用
    pub async fn call_batch<S, R>(&self, requests: Vec<R>) -> UnifiedResult<Vec<R::Response>>
    where
        S: UnifiedService + 'static,
        R: APIRequest + Send + Sync + Clone,
    {
        let executor = APIExecutor::new(self.client.clone());
        executor.execute_batch(requests).await
    }

    /// 带重试的API调用
    pub async fn call_with_retry<S, R>(&self, request: R, max_retries: u32) -> UnifiedResult<R::Response>
    where
        S: UnifiedService + 'static,
        R: APIRequest + Send + Sync + Clone,
    {
        let executor = APIExecutor::new(self.client.clone());
        executor.execute_with_retry(request, max_retries).await
    }
}

/// 服务API特征
///
/// 为每个服务定义特定的API接口。
pub trait ServiceAPI: Send + Sync {
    /// 服务类型
    type Service: UnifiedService;

    /// 获取服务实例
    fn service(&self) -> &Self::Service;

    /// 检查服务是否可用
    fn is_available(&self) -> bool {
        self.service().is_available()
    }

    /// 获取服务状态
    fn status(&self) -> ServiceStatus {
        self.service().status()
    }
}

/// 通信服务API
pub struct CommunicationAPI {
    client: UnifiedAPIClient,
}

impl CommunicationAPI {
    /// 创建通信服务API
    pub fn new(client: UnifiedAPIClient) -> Self {
        Self { client }
    }

    /// 发送文本消息
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

        self.client.call::<super::services::CommunicationService, _>(request).await
    }

    /// 发送图片消息
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

        self.client.call::<super::services::CommunicationService, _>(request).await
    }

    /// 获取消息列表
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

        self.client.call::<super::services::CommunicationService, _>(request).await
    }
}

/// HR服务API
pub struct HRAPI {
    client: UnifiedAPIClient,
}

impl HRAPI {
    /// 创建HR服务API
    pub fn new(client: UnifiedAPIClient) -> Self {
        Self { client }
    }

    /// 获取员工列表
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

        self.client.call::<super::services::HRService, _>(request).await
    }

    /// 获取单个员工信息
    pub async fn get_employee(
        &self,
        user_id: &str,
        user_id_type: Option<&str>,
    ) -> UnifiedResult<super::services::hr::Employee> {
        let request = GetEmployeeRequest {
            user_id: user_id.to_string(),
            user_id_type: user_id_type.map(|s| s.to_string()),
        };

        self.client.call::<super::services::HRService, _>(request).await
    }
}

/// 文档服务API
pub struct DocsAPI {
    client: UnifiedAPIClient,
}

impl DocsAPI {
    /// 创建文档服务API
    pub fn new(client: UnifiedAPIClient) -> Self {
        Self { client }
    }

    /// 创建电子表格
    pub async fn create_spreadsheet(
        &self,
        title: &str,
        folder_token: Option<&str>,
    ) -> UnifiedResult<super::services::docs::Spreadsheet> {
        let request = CreateSpreadsheetRequest {
            title: title.to_string(),
            folder_token: folder_token.map(|s| s.to_string()),
        };

        self.client.call::<super::services::DocsService, _>(request).await
    }

    /// 读取工作表范围
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

        self.client.call::<super::services::DocsService, _>(request).await
    }
}

/// AI服务API
pub struct AIAPI {
    client: UnifiedAPIClient,
}

impl AIAPI {
    /// 创建AI服务API
    pub fn new(client: UnifiedAPIClient) -> Self {
        Self { client }
    }

    /// 生成文本
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

        self.client.call::<super::services::AIService, _>(request).await
    }
}

/// 认证服务API
pub struct AuthAPI {
    client: UnifiedAPIClient,
}

impl AuthAPI {
    /// 创建认证服务API
    pub fn new(client: UnifiedAPIClient) -> Self {
        Self { client }
    }

    /// 获取应用访问令牌
    pub async fn get_app_access_token(&self) -> UnifiedResult<super::services::auth::TokenInfo> {
        let request = GetAppAccessTokenRequest;
        self.client.call::<super::services::AuthService, _>(request).await
    }
}

// 以下是具体的请求类型定义
// 注意：这些是示例定义，实际实现中需要根据各个服务的具体API来定义

#[derive(Debug, Clone, Serialize)]
pub struct SendTextMessageRequest {
    pub receive_id: String,
    pub receive_id_type: String,
    pub content: String,
}

impl APIRequest for SendTextMessageRequest {
    type Response = super::services::communication::MessageSendResult;

    fn request_type(&self) -> &'static str {
        "send_text_message"
    }

    fn validate(&self) -> UnifiedResult<()> {
        if self.receive_id.is_empty() {
            return Err(UnifiedError::ValidationError("receive_id不能为空".to_string()));
        }
        if self.content.is_empty() {
            return Err(UnifiedError::ValidationError("content不能为空".to_string()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SendImageMessageRequest {
    pub receive_id: String,
    pub receive_id_type: String,
    pub image_key: String,
}

impl APIRequest for SendImageMessageRequest {
    type Response = super::services::communication::MessageSendResult;

    fn request_type(&self) -> &'static str {
        "send_image_message"
    }

    fn validate(&self) -> UnifiedResult<()> {
        if self.receive_id.is_empty() {
            return Err(UnifiedError::ValidationError("receive_id不能为空".to_string()));
        }
        if self.image_key.is_empty() {
            return Err(UnifiedError::ValidationError("image_key不能为空".to_string()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ListMessagesRequest {
    pub container_id: String,
    pub container_id_type: String,
    pub page_size: Option<u32>,
    pub page_token: Option<String>,
}

impl APIRequest for ListMessagesRequest {
    type Response = super::services::communication::MessageListResult;

    fn request_type(&self) -> &'static str {
        "list_messages"
    }

    fn validate(&self) -> UnifiedResult<()> {
        if self.container_id.is_empty() {
            return Err(UnifiedError::ValidationError("container_id不能为空".to_string()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ListEmployeesRequest {
    pub user_id_type: Option<String>,
    pub page_size: Option<u32>,
    pub page_token: Option<String>,
}

impl APIRequest for ListEmployeesRequest {
    type Response = super::services::hr::EmployeeListResult;

    fn request_type(&self) -> &'static str {
        "list_employees"
    }

    fn validate(&self) -> UnifiedResult<()> {
        if let Some(page_size) = self.page_size {
            if page_size == 0 || page_size > 1000 {
                return Err(UnifiedError::ValidationError("page_size必须在1-1000之间".to_string()));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct GetEmployeeRequest {
    pub user_id: String,
    pub user_id_type: Option<String>,
}

impl APIRequest for GetEmployeeRequest {
    type Response = super::services::hr::Employee;

    fn request_type(&self) -> &'static str {
        "get_employee"
    }

    fn validate(&self) -> UnifiedResult<()> {
        if self.user_id.is_empty() {
            return Err(UnifiedError::ValidationError("user_id不能为空".to_string()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateSpreadsheetRequest {
    pub title: String,
    pub folder_token: Option<String>,
}

impl APIRequest for CreateSpreadsheetRequest {
    type Response = super::services::docs::Spreadsheet;

    fn request_type(&self) -> &'static str {
        "create_spreadsheet"
    }

    fn validate(&self) -> UnifiedResult<()> {
        if self.title.is_empty() {
            return Err(UnifiedError::ValidationError("title不能为空".to_string()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ReadRangeRequest {
    pub spreadsheet_token: String,
    pub range: String,
    pub value_render_option: Option<String>,
}

impl APIRequest for ReadRangeRequest {
    type Response = super::services::docs::RangeReadResult;

    fn request_type(&self) -> &'static str {
        "read_range"
    }

    fn validate(&self) -> UnifiedResult<()> {
        if self.spreadsheet_token.is_empty() {
            return Err(UnifiedError::ValidationError("spreadsheet_token不能为空".to_string()));
        }
        if self.range.is_empty() {
            return Err(UnifiedError::ValidationError("range不能为空".to_string()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct GenerateTextRequest {
    pub prompt: String,
    pub model: Option<String>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
}

impl APIRequest for GenerateTextRequest {
    type Response = super::services::ai::TextGenerationResult;

    fn request_type(&self) -> &'static str {
        "generate_text"
    }

    fn validate(&self) -> UnifiedResult<()> {
        if self.prompt.is_empty() {
            return Err(UnifiedError::ValidationError("prompt不能为空".to_string()));
        }
        if let Some(temperature) = self.temperature {
            if temperature < 0.0 || temperature > 2.0 {
                return Err(UnifiedError::ValidationError("temperature必须在0.0-2.0之间".to_string()));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct GetAppAccessTokenRequest;

impl APIRequest for GetAppAccessTokenRequest {
    type Response = super::services::auth::TokenInfo;

    fn request_type(&self) -> &'static str {
        "get_app_access_token"
    }

    fn validate(&self) -> UnifiedResult<()> {
        Ok(())
    }
}

// 为服务响应类型实现APIResponse特征
impl APIResponse for MessageSendResult {
    fn response_type(&self) -> &'static str {
        "message_send_result"
    }

    fn is_success(&self) -> bool {
        !self.message_id.is_empty()
    }
}

impl APIResponse for MessageListResult {
    fn response_type(&self) -> &'static str {
        "message_list_result"
    }

    fn is_success(&self) -> bool {
        true // 总是成功，除非有错误
    }
}

impl APIResponse for EmployeeListResult {
    fn response_type(&self) -> &'static str {
        "employee_list_result"
    }

    fn is_success(&self) -> bool {
        true // 总是成功，员工列表可能为空
    }
}

impl APIResponse for Employee {
    fn response_type(&self) -> &'static str {
        "employee"
    }

    fn is_success(&self) -> bool {
        !self.user_id.is_empty()
    }
}

impl APIResponse for Spreadsheet {
    fn response_type(&self) -> &'static str {
        "spreadsheet"
    }

    fn is_success(&self) -> bool {
        !self.spreadsheet_token.is_empty()
    }
}

impl APIResponse for RangeReadResult {
    fn response_type(&self) -> &'static str {
        "range_read_result"
    }

    fn is_success(&self) -> bool {
        true // 总是成功，范围可能为空
    }
}

impl APIResponse for TextGenerationResult {
    fn response_type(&self) -> &'static str {
        "text_generation_result"
    }

    fn is_success(&self) -> bool {
        !self.text.is_empty()
    }
}

impl APIResponse for TokenInfo {
    fn response_type(&self) -> &'static str {
        "token_info"
    }

    fn is_success(&self) -> bool {
        !self.access_token.is_empty()
    }
}