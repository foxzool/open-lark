//! ai_embedding 服务端点常量定义
//!
//! AI向量嵌入相关 API 端点常量，包括：
//! - 文本向量化
//! - 图像向量化
//! - 相似度计算
//! - 向量检索

/// 文本向量化
pub const TEXT_EMBEDDING: &str = "/open-apis/ai_embedding/v1/text/embedding";

/// 批量文本向量化
pub const BATCH_TEXT_EMBEDDING: &str = "/open-apis/ai_embedding/v1/text/batch_embedding";

/// 图像向量化
pub const IMAGE_EMBEDDING: &str = "/open-apis/ai_embedding/v1/image/embedding";

/// 向量相似度计算
pub const VECTOR_SIMILARITY: &str = "/open-apis/ai_embedding/v1/vector/similarity";

/// 向量检索
pub const VECTOR_SEARCH: &str = "/open-apis/ai_embedding/v1/vector/search";

/// 批量向量检索
pub const BATCH_VECTOR_SEARCH: &str = "/open-apis/ai_embedding/v1/vector/batch_search";

/// 向量数据库管理 - 创建索引
pub const VECTOR_INDEX_CREATE: &str = "/open-apis/ai_embedding/v1/index/create";

/// 向量数据库管理 - 删除索引
pub const VECTOR_INDEX_DELETE: &str = "/open-apis/ai_embedding/v1/index/delete";

/// 向量数据库管理 - 查询索引
pub const VECTOR_INDEX_QUERY: &str = "/open-apis/ai_embedding/v1/index/query";

/// 向量数据库管理 - 更新索引
pub const VECTOR_INDEX_UPDATE: &str = "/open-apis/ai_embedding/v1/index/update";

/// 多模态向量化 - 文本+图像
pub const MULTIMODAL_EMBEDDING: &str = "/open-apis/ai_embedding/v1/multimodal/embedding";

/// 语义搜索
pub const SEMANTIC_SEARCH: &str = "/open-apis/ai_embedding/v1/semantic/search";

/// 智能文档分析
pub const SMART_DOCUMENT_ANALYSIS: &str = "/open-apis/ai_embedding/v1/document/analysis";

/// 智能推荐
pub const SMART_RECOMMENDATION: &str = "/open-apis/ai_embedding/v1/recommendation";

/// 智能标签生成
pub const SMART_TAG_GENERATION: &str = "/open-apis/ai_embedding/v1/tag/generate";
