pub use text_embedding::*;
pub use vector_similarity::*;
pub use vector_search::*;
mod text_embedding;
mod vector_similarity;
mod vector_search;
/// AI嵌入V1服务
pub struct V1 {
}

impl V1 {
}
    pub fn new(config: Config) -> Self {
        Self { config }
}