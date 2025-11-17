//! OpenLark 服务适配器
//!
//! 提供统一的API调用接口，将请求路由到具体的服务实现。

use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};

use super::{
    api::APIError,
    services::{
        CommunicationService, HRService, DocsService, AIService, AuthService,
        communication::{MessageSendResult, MessageListResult},
        hr::{EmployeeListResult, Employee},
        docs::{Spreadsheet, RangeReadResult},
        ai::{TextGenerationResult},
        auth::{TokenInfo},
    },
    error::{UnifiedError, UnifiedResult},
};

/// 服务适配器特征
///
/// 定义服务适配器的基本接口。
#[async_trait]
pub trait ServiceAdapter: Send + Sync {
    /// 服务名称
    fn name(&self) -> &'static str;

    /// 服务版本
    fn version(&self) -> &'static str;

    /// 检查服务是否可用
    fn is_available(&self) -> bool;

    /// 执行健康检查
    async fn health_check(&self) -> UnifiedResult<bool> {
        Ok(self.is_available())
    }
}

/// API请求分发器
///
/// 负责将API请求分发到对应的服务适配器。
pub struct APIDispatcher {
    adapters: HashMap<String, Arc<dyn ServiceAdapter>>,
}

impl APIDispatcher {
    /// 创建新的API分发器
    pub fn new() -> Self {
        Self {
            adapters: HashMap::new(),
        }
    }

    /// 注册服务适配器
    pub fn register_adapter(&mut self, adapter: Arc<dyn ServiceAdapter>) {
        self.adapters.insert(adapter.name().to_string(), adapter);
    }

    /// 获取服务适配器
    pub fn get_adapter(&self, name: &str) -> Option<&Arc<dyn ServiceAdapter>> {
        self.adapters.get(name)
    }

    /// 列出所有服务
    pub fn list_services(&self) -> Vec<&str> {
        self.adapters.keys().map(|s| s.as_str()).collect()
    }

    /// 检查服务是否可用
    pub fn is_service_available(&self, name: &str) -> bool {
        self.adapters
            .get(name)
            .map(|adapter| adapter.is_available())
            .unwrap_or(false)
    }
}

impl Default for APIDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

/// 通信服务适配器
pub struct CommunicationServiceAdapter {
    service: CommunicationService,
}

impl CommunicationServiceAdapter {
    /// 创建新的通信服务适配器
    pub fn new(service: CommunicationService) -> Self {
        Self { service }
    }
}

#[async_trait]
impl ServiceAdapter for CommunicationServiceAdapter {
    fn name(&self) -> &'static str {
        "communication"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

    fn is_available(&self) -> bool {
        self.service.is_enabled()
    }
}

impl CommunicationServiceAdapter {
    /// 发送文本消息
    pub async fn send_text_message(
        &self,
        receive_id: &str,
        receive_id_type: &str,
        content: &str,
    ) -> UnifiedResult<MessageSendResult> {
        self.service.send_text_message(receive_id, receive_id_type, content).await
    }

    /// 发送图片消息
    pub async fn send_image_message(
        &self,
        receive_id: &str,
        receive_id_type: &str,
        image_key: &str,
    ) -> UnifiedResult<MessageSendResult> {
        self.service.send_image_message(receive_id, receive_id_type, image_key).await
    }

    /// 获取消息列表
    pub async fn list_messages(
        &self,
        container_id: &str,
        container_id_type: &str,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> UnifiedResult<MessageListResult> {
        self.service.list_messages(container_id, container_id_type, page_size, page_token).await
    }
}

/// HR服务适配器
pub struct HRServiceAdapter {
    service: HRService,
}

impl HRServiceAdapter {
    /// 创建新的HR服务适配器
    pub fn new(service: HRService) -> Self {
        Self { service }
    }
}

#[async_trait]
impl ServiceAdapter for HRServiceAdapter {
    fn name(&self) -> &'static str {
        "hr"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

    fn is_available(&self) -> bool {
        self.service.is_enabled()
    }
}

impl HRServiceAdapter {
    /// 获取员工列表
    pub async fn list_employees(
        &self,
        user_id_type: Option<&str>,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> UnifiedResult<EmployeeListResult> {
        self.service.list_employees(user_id_type, page_size, page_token).await
    }

    /// 获取单个员工信息
    pub async fn get_employee(
        &self,
        user_id: &str,
        user_id_type: Option<&str>,
    ) -> UnifiedResult<Employee> {
        self.service.get_employee(user_id, user_id_type).await
    }
}

/// 文档服务适配器
pub struct DocsServiceAdapter {
    service: DocsService,
}

impl DocsServiceAdapter {
    /// 创建新的文档服务适配器
    pub fn new(service: DocsService) -> Self {
        Self { service }
    }
}

#[async_trait]
impl ServiceAdapter for DocsServiceAdapter {
    fn name(&self) -> &'static str {
        "docs"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

    fn is_available(&self) -> bool {
        self.service.is_enabled()
    }
}

impl DocsServiceAdapter {
    /// 创建电子表格
    pub async fn create_spreadsheet(
        &self,
        title: &str,
        folder_token: Option<&str>,
    ) -> UnifiedResult<Spreadsheet> {
        self.service.create_spreadsheet(title, folder_token).await
    }

    /// 获取电子表格信息
    pub async fn get_spreadsheet(
        &self,
        spreadsheet_token: &str,
    ) -> UnifiedResult<Spreadsheet> {
        self.service.get_spreadsheet(spreadsheet_token).await
    }

    /// 读取工作表范围
    pub async fn read_range(
        &self,
        spreadsheet_token: &str,
        range: &str,
        value_render_option: Option<&str>,
    ) -> UnifiedResult<RangeReadResult> {
        self.service.read_range(spreadsheet_token, range, value_render_option).await
    }

    /// 写入工作表范围
    pub async fn write_range(
        &self,
        spreadsheet_token: &str,
        range: &str,
        values: Vec<Vec<String>>,
    ) -> UnifiedResult<super::services::docs::WriteResult> {
        self.service.write_range(spreadsheet_token, range, values).await
    }
}

/// AI服务适配器
pub struct AIServiceAdapter {
    service: AIService,
}

impl AIServiceAdapter {
    /// 创建新的AI服务适配器
    pub fn new(service: AIService) -> Self {
        Self { service }
    }
}

#[async_trait]
impl ServiceAdapter for AIServiceAdapter {
    fn name(&self) -> &'static str {
        "ai"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

    fn is_available(&self) -> bool {
        self.service.is_enabled()
    }
}

impl AIServiceAdapter {
    /// AI文本生成
    pub async fn generate_text(
        &self,
        prompt: &str,
        model: Option<&str>,
        temperature: Option<f32>,
        max_tokens: Option<u32>,
    ) -> UnifiedResult<TextGenerationResult> {
        self.service.generate_text(prompt, model, temperature, max_tokens).await
    }

    /// AI对话完成
    pub async fn chat_completion(
        &self,
        messages: Vec<super::services::ai::ChatMessage>,
        model: Option<&str>,
        temperature: Option<f32>,
        max_tokens: Option<u32>,
    ) -> UnifiedResult<super::services::ai::ChatCompletionResult> {
        self.service.chat_completion(messages, model, temperature, max_tokens).await
    }

    /// 文本嵌入
    pub async fn create_embedding(
        &self,
        input: &str,
        model: Option<&str>,
    ) -> UnifiedResult<super::services::ai::EmbeddingResult> {
        self.service.create_embedding(input, model).await
    }

    /// 获取模型列表
    pub async fn list_models(&self) -> UnifiedResult<Vec<super::services::ai::ModelInfo>> {
        self.service.list_models().await
    }
}

/// 认证服务适配器
pub struct AuthServiceAdapter {
    service: AuthService,
}

impl AuthServiceAdapter {
    /// 创建新的认证服务适配器
    pub fn new(service: AuthService) -> Self {
        Self { service }
    }
}

#[async_trait]
impl ServiceAdapter for AuthServiceAdapter {
    fn name(&self) -> &'static str {
        "auth"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

    fn is_available(&self) -> bool {
        self.service.is_enabled()
    }
}

impl AuthServiceAdapter {
    /// 获取应用访问令牌
    pub async fn get_app_access_token(&self) -> UnifiedResult<TokenInfo> {
        self.service.get_app_access_token().await
    }

    /// 获取用户访问令牌
    pub async fn get_user_access_token(&self, code: &str) -> UnifiedResult<TokenInfo> {
        self.service.get_user_access_token(code).await
    }

    /// 刷新访问令牌
    pub async fn refresh_access_token(&self, refresh_token: &str) -> UnifiedResult<TokenInfo> {
        self.service.refresh_access_token(refresh_token).await
    }

    /// 验证令牌
    pub async fn verify_token(&self, token: &str) -> UnifiedResult<super::services::auth::TokenVerificationResult> {
        self.service.verify_token(token).await
    }

    /// 获取OAuth授权URL
    pub fn get_oauth_url(&self, client_id: &str, redirect_uri: &str, scopes: &[&str]) -> String {
        self.service.get_oauth_url(client_id, redirect_uri, scopes)
    }
}

/// 服务适配器工厂
///
/// 负责创建和配置各种服务适配器。
pub struct ServiceAdapterFactory;

impl ServiceAdapterFactory {
    /// 创建所有默认的服务适配器
    pub fn create_default_adapters() -> Vec<Arc<dyn ServiceAdapter>> {
        let mut adapters: Vec<Arc<dyn ServiceAdapter>> = Vec::new();

        // 创建通信服务适配器
        let comm_service = CommunicationService::new();
        adapters.push(Arc::new(CommunicationServiceAdapter::new(comm_service)));

        // 创建HR服务适配器
        let hr_service = HRService::new();
        adapters.push(Arc::new(HRServiceAdapter::new(hr_service)));

        // 创建文档服务适配器
        let docs_service = DocsService::new();
        adapters.push(Arc::new(DocsServiceAdapter::new(docs_service)));

        // 创建AI服务适配器
        let ai_service = AIService::new();
        adapters.push(Arc::new(AIServiceAdapter::new(ai_service)));

        // 创建认证服务适配器
        let auth_service = AuthService::new();
        adapters.push(Arc::new(AuthServiceAdapter::new(auth_service)));

        adapters
    }

    /// 创建API分发器并注册所有适配器
    pub fn create_dispatcher() -> APIDispatcher {
        let mut dispatcher = APIDispatcher::new();
        let adapters = Self::create_default_adapters();

        for adapter in adapters {
            dispatcher.register_adapter(adapter);
        }

        dispatcher
    }

    /// 从配置创建适配器
    pub fn create_adapters_from_config(
        config: &super::config::UnifiedConfig,
    ) -> Vec<Arc<dyn ServiceAdapter>> {
        let mut adapters: Vec<Arc<dyn ServiceAdapter>> = Vec::new();

        // 根据配置创建通信服务适配器
        if config.is_feature_enabled("communication") {
            let mut service = CommunicationService::new();
            if let Some(comm_config) = &config.services.communication {
                if let Err(e) = async_block! {
                    service.configure(comm_config.clone()).await
                } {
                    tracing::warn!("配置通信服务失败: {}", e);
                }
            }
            adapters.push(Arc::new(CommunicationServiceAdapter::new(service)));
        }

        // 根据配置创建HR服务适配器
        if config.is_feature_enabled("hr") {
            let mut service = HRService::new();
            if let Some(hr_config) = &config.services.hr {
                if let Err(e) = async_block! {
                    service.configure(hr_config.clone()).await
                } {
                    tracing::warn!("配置HR服务失败: {}", e);
                }
            }
            adapters.push(Arc::new(HRServiceAdapter::new(service)));
        }

        // 根据配置创建文档服务适配器
        if config.is_feature_enabled("docs") {
            let mut service = DocsService::new();
            if let Some(docs_config) = &config.services.docs {
                if let Err(e) = async_block! {
                    service.configure(docs_config.clone()).await
                } {
                    tracing::warn!("配置文档服务失败: {}", e);
                }
            }
            adapters.push(Arc::new(DocsServiceAdapter::new(service)));
        }

        // 根据配置创建AI服务适配器
        if config.is_feature_enabled("ai") {
            let mut service = AIService::new();
            if let Some(ai_config) = &config.services.ai {
                if let Err(e) = async_block! {
                    service.configure(ai_config.clone()).await
                } {
                    tracing::warn!("配置AI服务失败: {}", e);
                }
            }
            adapters.push(Arc::new(AIServiceAdapter::new(service)));
        }

        // 根据配置创建认证服务适配器
        if config.is_feature_enabled("auth") {
            let mut service = AuthService::new();
            if let Some(auth_config) = &config.services.auth {
                if let Err(e) = async_block! {
                    service.configure(auth_config.clone()).await
                } {
                    tracing::warn!("配置认证服务失败: {}", e);
                }
            }
            adapters.push(Arc::new(AuthServiceAdapter::new(service)));
        }

        adapters
    }
}

/// 异步块辅助函数
async fn async_block<F, T>(future: F) -> T
where
    F: std::future::Future<Output = T>,
{
    future.await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_adapter_creation() {
        let comm_service = CommunicationService::new();
        let adapter = CommunicationServiceAdapter::new(comm_service);

        assert_eq!(adapter.name(), "communication");
        assert_eq!(adapter.version(), "1.0.0");
    }

    #[test]
    fn test_api_dispatcher() {
        let mut dispatcher = APIDispatcher::new();

        let comm_service = CommunicationService::new();
        let adapter = Arc::new(CommunicationServiceAdapter::new(comm_service));

        dispatcher.register_adapter(adapter.clone());

        assert_eq!(dispatcher.list_services().len(), 1);
        assert!(dispatcher.is_service_available("communication"));
        assert_eq!(dispatcher.get_adapter("communication").unwrap().name(), "communication");
    }

    #[test]
    fn test_service_adapter_factory() {
        let adapters = ServiceAdapterFactory::create_default_adapters();
        assert_eq!(adapters.len(), 5); // communication, hr, docs, ai, auth

        let dispatcher = ServiceAdapterFactory::create_dispatcher();
        assert_eq!(dispatcher.list_services().len(), 5);
    }
}