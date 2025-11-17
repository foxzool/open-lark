//! AI服务适配器
//!
//! 将openlark-ai服务适配到统一客户端接口。

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;

use crate::unified::{
    traits::{UnifiedService, ServiceDescriptor, ServiceStatus, ServiceLifecycle},
    config::{UnifiedConfig, AIConfig},
    error::{UnifiedError, UnifiedResult},
};

/// AI服务适配器
///
/// 将openlark-ai的功能适配到统一客户端接口。
#[derive(Debug, Clone)]
pub struct AIService {
    /// 服务配置
    config: Option<AIConfig>,
    /// 服务状态
    status: ServiceStatus,
    /// 核心客户端（用于实际API调用）
    core_client: Option<Arc<openlark_core::client::LarkClient>>,
    /// 服务元数据
    metadata: HashMap<String, String>,
}

impl AIService {
    /// 创建新的AI服务适配器
    pub fn new() -> Self {
        Self {
            config: None,
            status: ServiceStatus::Uninitialized,
            core_client: None,
            metadata: HashMap::new(),
        }
    }

    /// 从配置创建服务
    pub fn with_config(mut self, config: AIConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// 从核心客户端创建服务
    pub fn with_core_client(mut self, core_client: Arc<openlark_core::client::LarkClient>) -> Self {
        self.core_client = Some(core_client);
        self
    }

    /// 检查服务是否可用
    pub fn is_enabled(&self) -> bool {
        self.config
            .as_ref()
            .map(|config| config.enabled)
            .unwrap_or(false)
    }

    /// AI文本生成
    pub async fn generate_text(
        &self,
        prompt: &str,
        model: Option<&str>,
        temperature: Option<f32>,
        max_tokens: Option<u32>,
    ) -> UnifiedResult<TextGenerationResult> {
        self.ensure_available()?;

        let generation_request = serde_json::json!({
            "prompt": prompt,
            "model": model.unwrap_or("gpt-3.5-turbo"),
            "temperature": temperature.unwrap_or(0.7),
            "max_tokens": max_tokens.unwrap_or(1000)
        });

        tracing::info!("AI文本生成: {:?}", generation_request);

        Ok(TextGenerationResult {
            text: "Mock generated text".to_string(),
            model: model.unwrap_or("gpt-3.5-turbo").to_string(),
            usage: TokenUsage {
                prompt_tokens: 100,
                completion_tokens: 200,
                total_tokens: 300,
            },
            finish_reason: "stop".to_string(),
            created_at: chrono::Utc::now(),
        })
    }

    /// AI对话
    pub async fn chat_completion(
        &self,
        messages: Vec<ChatMessage>,
        model: Option<&str>,
        temperature: Option<f32>,
        max_tokens: Option<u32>,
    ) -> UnifiedResult<ChatCompletionResult> {
        self.ensure_available()?;

        let chat_request = serde_json::json!({
            "messages": messages,
            "model": model.unwrap_or("gpt-3.5-turbo"),
            "temperature": temperature.unwrap_or(0.7),
            "max_tokens": max_tokens.unwrap_or(1000)
        });

        tracing::info!("AI对话完成: {:?}", chat_request);

        Ok(ChatCompletionResult {
            id: "mock_chat_id".to_string(),
            object: "chat.completion".to_string(),
            created: chrono::Utc::now().timestamp() as u64,
            model: model.unwrap_or("gpt-3.5-turbo").to_string(),
            choices: vec![ChatChoice {
                index: 0,
                message: ChatMessage {
                    role: "assistant".to_string(),
                    content: "Mock assistant response".to_string(),
                },
                finish_reason: Some("stop".to_string()),
            }],
            usage: TokenUsage {
                prompt_tokens: 150,
                completion_tokens: 250,
                total_tokens: 400,
            },
        })
    }

    /// 文本嵌入
    pub async fn create_embedding(
        &self,
        input: &str,
        model: Option<&str>,
    ) -> UnifiedResult<EmbeddingResult> {
        self.ensure_available()?;

        let embedding_request = serde_json::json!({
            "input": input,
            "model": model.unwrap_or("text-embedding-ada-002")
        });

        tracing::info!("创建文本嵌入: {:?}", embedding_request);

        Ok(EmbeddingResult {
            object: "embedding".to_string(),
            model: model.unwrap_or("text-embedding-ada-002").to_string(),
            embedding: vec![0.1; 1536], // Mock 1536-dimensional vector
            usage: TokenUsage {
                prompt_tokens: 10,
                completion_tokens: 0,
                total_tokens: 10,
            },
        })
    }

    /// 图像生成
    pub async fn generate_image(
        &self,
        prompt: &str,
        size: Option<&str>,
        quality: Option<&str>,
    ) -> UnifiedResult<ImageGenerationResult> {
        self.ensure_available()?;

        let image_request = serde_json::json!({
            "prompt": prompt,
            "size": size.unwrap_or("1024x1024"),
            "quality": quality.unwrap_or("standard"),
            "n": 1
        });

        tracing::info!("AI图像生成: {:?}", image_request);

        Ok(ImageGenerationResult {
            id: "mock_image_id".to_string(),
            object: "image".to_string(),
            created: chrono::Utc::now().timestamp() as u64,
            url: "https://example.com/mock-image.png".to_string(),
            size: size.unwrap_or("1024x1024").to_string(),
        })
    }

    /// 检查模型可用性
    pub async fn list_models(&self) -> UnifiedResult<Vec<ModelInfo>> {
        self.ensure_available()?;

        tracing::info!("获取AI模型列表");

        Ok(vec![
            ModelInfo {
                id: "gpt-3.5-turbo".to_string(),
                object: "model".to_string(),
                created: chrono::Utc::now().timestamp() as u64,
                owned_by: "openai".to_string(),
            },
            ModelInfo {
                id: "gpt-4".to_string(),
                object: "model".to_string(),
                created: chrono::Utc::now().timestamp() as u64,
                owned_by: "openai".to_string(),
            },
        ])
    }

    /// 确保服务可用
    fn ensure_available(&self) -> UnifiedResult<()> {
        if !self.is_enabled() {
            return Err(UnifiedError::ServiceNotAvailable("ai".to_string()));
        }

        if self.status != ServiceStatus::Running {
            return Err(UnifiedError::ServiceNotAvailable(
                "ai service not running".to_string(),
            ));
        }

        Ok(())
    }
}

/// 文本生成结果
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TextGenerationResult {
    /// 生成的文本
    pub text: String,
    /// 使用的模型
    pub model: String,
    /// Token使用情况
    pub usage: TokenUsage,
    /// 完成原因
    pub finish_reason: String,
    /// 创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// 对话消息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChatMessage {
    /// 角色（system, user, assistant）
    pub role: String,
    /// 消息内容
    pub content: String,
}

/// 对话完成结果
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChatCompletionResult {
    /// 对话ID
    pub id: String,
    /// 对象类型
    pub object: String,
    /// 创建时间
    pub created: u64,
    /// 使用的模型
    pub model: String,
    /// 选择列表
    pub choices: Vec<ChatChoice>,
    /// Token使用情况
    pub usage: TokenUsage,
}

/// 对话选择
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChatChoice {
    /// 选择索引
    pub index: u32,
    /// 消息
    pub message: ChatMessage,
    /// 完成原因
    pub finish_reason: Option<String>,
}

/// 嵌入结果
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EmbeddingResult {
    /// 对象类型
    pub object: String,
    /// 使用的模型
    pub model: String,
    /// 嵌入向量
    pub embedding: Vec<f32>,
    /// Token使用情况
    pub usage: TokenUsage,
}

/// 图像生成结果
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ImageGenerationResult {
    /// 图像ID
    pub id: String,
    /// 对象类型
    pub object: String,
    /// 创建时间
    pub created: u64,
    /// 图像URL
    pub url: String,
    /// 图像尺寸
    pub size: String,
}

/// 模型信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ModelInfo {
    /// 模型ID
    pub id: String,
    /// 对象类型
    pub object: String,
    /// 创建时间
    pub created: u64,
    /// 所有者
    pub owned_by: String,
}

/// Token使用情况
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TokenUsage {
    /// 提示Token数量
    pub prompt_tokens: u32,
    /// 完成Token数量
    pub completion_tokens: u32,
    /// 总Token数量
    pub total_tokens: u32,
}

impl Default for AIService {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl UnifiedService for AIService {
    type Config = AIConfig;
    type Error = UnifiedError;

    fn name(&self) -> &'static str {
        "ai"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

    async fn configure(&mut self, config: Self::Config) -> UnifiedResult<()> {
        if !config.enabled {
            self.status = ServiceStatus::Stopped;
            return Ok(());
        }

        self.config = Some(config);

        // 创建核心客户端
        let core_config = self.config.as_ref().map(|config| {
            openlark_core::config::ConfigBuilder::new()
                .base_url(&config.api_url)
                .build()
                .unwrap_or_else(|_| openlark_core::config::Config::default())
        });

        if let Some(core_config) = core_config {
            match openlark_core::client::LarkClient::new(
                core_config.app_id.clone(),
                core_config.app_secret.clone(),
            ) {
                Ok(client) => {
                    self.core_client = Some(Arc::new(client));
                    self.status = ServiceStatus::Running;
                    tracing::info!("AI服务配置成功");
                    Ok(())
                }
                Err(e) => {
                    self.status = ServiceStatus::Error;
                    Err(UnifiedError::ConfigurationError(
                        format!("创建核心客户端失败: {}", e),
                    ))
                }
            }
        } else {
            self.status = ServiceStatus::Error;
            Err(UnifiedError::ConfigurationError("AI配置无效".to_string()))
        }
    }

    fn is_available(&self) -> bool {
        self.is_enabled() && self.status == ServiceStatus::Running && self.core_client.is_some()
    }

    fn status(&self) -> ServiceStatus {
        self.status
    }

    fn descriptor(&self) -> ServiceDescriptor {
        let mut descriptor = ServiceDescriptor::new(
            "ai",
            "1.0.0",
            "飞书AI服务，提供智能助手、文本生成、图像识别等功能",
        )
        .with_tag("ai")
        .with_tag("intelligence")
        .with_dependency("openlark-core");

        if let Some(config) = &self.config {
            descriptor = descriptor
                .with_metadata("api_url", config.api_url.clone())
                .with_metadata("enabled", config.enabled.to_string())
                .with_metadata(
                    "default_model",
                    config.models.default_model.clone(),
                )
                .with_metadata(
                    "requests_per_minute",
                    config.rate_limit.requests_per_minute.to_string(),
                );
        }

        descriptor
    }
}

#[async_trait]
impl ServiceLifecycle for AIService {
    async fn start(&mut self) -> SDKResult<()> {
        if let Some(config) = self.config.clone() {
            self.configure(config).await?;
        } else {
            tracing::warn!("AI服务配置未设置，服务将处于未初始化状态");
        }
        Ok(())
    }

    async fn stop(&mut self) -> SDKResult<()> {
        self.status = ServiceStatus::Stopped;
        self.core_client = None;
        tracing::info!("AI服务已停止");
        Ok(())
    }

    async fn health_check(&self) -> SDKResult<bool> {
        Ok(self.is_available())
    }
}

/// AI服务构建器
pub struct AIServiceBuilder {
    config: Option<AIConfig>,
    core_client: Option<Arc<openlark_core::client::LarkClient>>,
}

impl AIServiceBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self {
            config: None,
            core_client: None,
        }
    }

    /// 设置配置
    pub fn config(mut self, config: AIConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// 设置核心客户端
    pub fn core_client(mut self, core_client: Arc<openlark_core::client::LarkClient>) -> Self {
        self.core_client = Some(core_client);
        self
    }

    /// 构建服务
    pub fn build(self) -> UnifiedResult<AIService> {
        let mut service = AIService::new();

        if let Some(config) = self.config {
            service = service.with_config(config);
        }

        if let Some(core_client) = self.core_client {
            service = service.with_core_client(core_client);
        }

        Ok(service)
    }
}

impl Default for AIServiceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_service_creation() {
        let service = AIService::new();
        assert_eq!(service.name(), "ai");
        assert_eq!(service.version(), "1.0.0");
    }

    #[test]
    fn test_ai_service_builder() {
        let config = AIConfig::default();
        let service = AIServiceBuilder::new()
            .config(config)
            .build()
            .unwrap();

        assert!(service.is_enabled());
    }

    #[tokio::test]
    async fn test_service_lifecycle() {
        let mut service = AIService::new();

        // 测试启动
        service.start().await.unwrap();
        // 由于没有配置，服务应该是未初始化状态
        assert_eq!(service.status(), ServiceStatus::Stopped);

        // 测试停止
        service.stop().await.unwrap();
        assert_eq!(service.status(), ServiceStatus::Stopped);
    }

    #[tokio::test]
    async fn test_ai_operations() {
        let service = AIService::new();

        // 测试文本生成
        let result = service
            .generate_text("Generate text about AI", Some("gpt-3.5-turbo"), None, None)
            .await;
        assert!(result.is_ok());

        // 测试对话完成
        let messages = vec![
            ChatMessage {
                role: "user".to_string(),
                content: "Hello, AI!".to_string(),
            },
        ];
        let result = service
            .chat_completion(messages, Some("gpt-3.5-turbo"), None, None)
            .await;
        assert!(result.is_ok());

        // 测试文本嵌入
        let result = service.create_embedding("Hello world", None).await;
        assert!(result.is_ok());

        // 测试图像生成
        let result = service
            .generate_image("A beautiful sunset", Some("1024x1024"), None)
            .await;
        assert!(result.is_ok());

        // 测试模型列表
        let result = service.list_models().await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_service_descriptors() {
        let service = AIService::new();
        let descriptor = service.descriptor();

        assert_eq!(descriptor.name, "ai");
        assert_eq!(descriptor.version, "1.0.0");
        assert!(descriptor.tags.contains(&"ai".to_string()));
    }
}