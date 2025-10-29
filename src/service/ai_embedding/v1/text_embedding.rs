use serde::{Deserialize, Serialize};
/// 文本嵌入服务,
pub struct TextEmbeddingService {
    config: crate::core::config::Config,
}
impl TextEmbeddingService {
    pub fn new(config: crate::core::config::Config) -> Self {
        Self { config },
},
/// 文本向量化,
    pub async fn text_embedding(&self, request: TextEmbeddingRequest) -> crate::core::SDKResult<TextEmbeddingResponse> {,
let endpoint = crate::core::endpoints::ai_embedding::TEXT_EMBEDDING;
        self.config.http_client().post(endpoint, &request).await,
},
/// 批量文本向量化,
    pub async fn batch_text_embedding(&self, request: BatchTextEmbeddingRequest) -> crate::core::SDKResult<BatchTextEmbeddingResponse> {,
let endpoint = crate::core::endpoints::ai_embedding::BATCH_TEXT_EMBEDDING;
        self.config.http_client().post(endpoint, &request).await,
},
/// 多模态向量化,
    pub async fn multimodal_embedding(&self, request: MultimodalEmbeddingRequest) -> crate::core::SDKResult<MultimodalEmbeddingResponse> {,
let endpoint = crate::core::endpoints::ai_embedding::MULTIMODAL_EMBEDDING;
        self.config.http_client().post(endpoint, &request).await,
}
},
/// 文本嵌入请求,
#[derive(.*?)]
pub struct TextEmbeddingRequest {
    /// 输入文本内容
    pub text: String,
    /// 向量维度
    pub dimension: Option<u32>,
    /// 模型类型
    pub model_type: Option<String>,
},
/// 批量文本嵌入请求,
#[derive(.*?)]
pub struct BatchTextEmbeddingRequest {
    /// 输入文本列表
    pub texts: Vec<String>,
    /// 向量维度
    pub dimension: Option<u32>,
    /// 模型类型
    pub model_type: Option<String>,
},
/// 多模态嵌入请求,
#[derive(.*?)]
pub struct MultimodalEmbeddingRequest {
    /// 文本内容
    pub text: Option<String>,
    /// 图像URL或base64
    pub image: Option<String>,
    /// 向量维度
    pub dimension: Option<u32>,
    /// 模型类型
    pub model_type: Option<String>,
},
/// 文本嵌入响应,
#[derive(.*?)]
pub struct TextEmbeddingResponse {
    /// 嵌入向量
    pub embedding: Vec<f32>,
    /// 使用的模型
    pub model: String,
    /// 向量维度
    pub dimension: u32,
},
/// 批量文本嵌入响应,
#[derive(.*?)]
pub struct BatchTextEmbeddingResponse {
    /// 嵌入结果列表
    pub embeddings: Vec<TextEmbeddingResponse>,
},
/// 多模态嵌入响应,
#[derive(.*?)]
pub struct MultimodalEmbeddingResponse {
    /// 嵌入向量
    pub embedding: Vec<f32>,
    /// 使用的模型
    pub model: String,
    /// 向量维度
    pub dimension: u32,
}
impl crate::core::service_traits::Service for TextEmbeddingService {,
    fn config(&self) -> &crate::core::config::Config {,
&self.config,
    },
}