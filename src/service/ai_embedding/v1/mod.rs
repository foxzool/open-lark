pub use text_embedding::*;
pub use vector_similarity::*;
pub use vector_search::*;
mod text_embedding;
mod vector_similarity;
mod vector_search;
/// AI嵌入V1服务
pub struct V1 {
    /// 文本嵌入服务
    pub text_embedding: TextEmbeddingService,
    /// 向量相似度服务
    pub vector_similarity: VectorSimilarityService,
    /// 向量搜索服务
    pub vector_search: VectorSearchService,
}
impl V1 {
    pub fn new() -> Self {
Self {,
            text_embedding: TextEmbeddingService::new(config.clone()),
            vector_similarity: VectorSimilarityService::new(config.clone()),
            vector_search: VectorSearchService::new(config),
        }
}
}