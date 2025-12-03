//! URL构建工具

/// URL构建器
pub struct UrlBuilder {
    base_url: String,
}

impl UrlBuilder {
    /// 创建新的URL构建器
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
        }
    }

    /// 构建API端点URL
    pub fn build_endpoint(&self, path: &str) -> String {
        format!("{}{}", self.base_url.trim_end_matches('/'), path)
    }
}